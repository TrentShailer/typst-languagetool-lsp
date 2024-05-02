use std::{cmp::min, collections::HashMap};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower_lsp::{
    jsonrpc::{self},
    lsp_types::*,
    Client, LanguageServer, LspService, Server,
};
use typst_languagetool_checker::check_file;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::FULL),
                    will_save: None,
                    will_save_wait_until: None,
                    save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                        include_text: Some(true),
                    })),
                },
            )),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),

            ..Default::default()
        };
        Ok(InitializeResult {
            capabilities,
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.check_file(
            params.text_document.uri,
            params.text_document.text,
            Some(params.text_document.version),
        )
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.check_file(
            params.text_document.uri,
            std::mem::take(&mut params.content_changes[0].text),
            Some(params.text_document.version),
        )
        .await
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Saved file")
            .await;
        let text = match params.text {
            Some(v) => v,
            None => return,
        };

        self.check_file(params.text_document.uri, text, None).await
    }

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> jsonrpc::Result<Option<CodeActionResponse>> {
        let mut actions = vec![];

        for diagnostic in params.context.diagnostics {
            match diagnostic.source {
                Some(v) => {
                    if v != String::from("typst-languagetool") {
                        continue;
                    }
                }
                None => continue,
            };

            let data = match diagnostic.data {
                Some(v) => v,
                None => continue,
            };

            let data: DiagnosticData = match serde_json::from_value(data) {
                Ok(v) => v,
                Err(_e) => continue,
            };

            let mut replacements: Vec<_> = data.replacements[..min(10, data.replacements.len())]
                .iter()
                .map(|replacement| {
                    let mut edits = HashMap::new();

                    edits.insert(
                        params.text_document.uri.clone(),
                        vec![TextEdit::new(diagnostic.range, replacement.to_string())],
                    );

                    CodeActionOrCommand::CodeAction(CodeAction {
                        title: format!("Replace with '{}'", replacement),
                        kind: Some(CodeActionKind::QUICKFIX),
                        edit: Some(WorkspaceEdit::new(edits)),
                        ..Default::default()
                    })
                })
                .collect();

            actions.append(&mut replacements);

            let ignore_action = CodeAction {
                title: format!("Ignore '{}' in workspace", data.match_string),
                kind: Some(CodeActionKind::QUICKFIX),
                command: Some(Command::new(
                    "Ignore Word".to_string(),
                    "typst-languagetool-lsp.ignore_word".to_string(),
                    Some(vec![serde_json::to_value(data.match_string).unwrap()]),
                )),
                ..Default::default()
            };
            let diable_rule_action = CodeAction {
                title: format!("Disable rule in workspace '{}'", data.rule_id),
                kind: Some(CodeActionKind::QUICKFIX),
                command: Some(Command::new(
                    "Disable Rule".to_string(),
                    "typst-languagetool-lsp.disable_rule".to_string(),
                    Some(vec![serde_json::to_value(data.rule_id).unwrap()]),
                )),
                ..Default::default()
            };
            let diable_category_action = CodeAction {
                title: format!("Disable category in workspace '{}'", data.rule_category),
                kind: Some(CodeActionKind::QUICKFIX),
                command: Some(Command::new(
                    "Disable Cateogiry".to_string(),
                    "typst-languagetool-lsp.disable_category".to_string(),
                    Some(vec![serde_json::to_value(data.rule_category).unwrap()]),
                )),
                ..Default::default()
            };

            actions.push(CodeActionOrCommand::CodeAction(ignore_action));
            actions.push(CodeActionOrCommand::CodeAction(diable_rule_action));
            actions.push(CodeActionOrCommand::CodeAction(diable_category_action));
        }

        Ok(Some(actions))
    }
}

fn new_configuration_item(section: &str) -> ConfigurationItem {
    ConfigurationItem {
        section: Some(format!("typst-languagetool-lsp.{}", section)),
        scope_uri: None,
    }
}

#[derive(Debug, Error)]
enum GetConfigurationError {
    #[error("Failed to fetch configuration items: {0}")]
    Fetch(#[from] jsonrpc::Error),

    #[error("Failed to parse configuration items")]
    Parse,
}

impl Backend {
    async fn get_configuration(&self) -> Result<Settings, GetConfigurationError> {
        let configuration = self
            .client
            .configuration(vec![
                new_configuration_item("host"),
                new_configuration_item("port"),
                new_configuration_item("language"),
                new_configuration_item("disabled_rules"),
                new_configuration_item("disabled_categories"),
                new_configuration_item("ignore_words"),
            ])
            .await?;

        let host = match configuration[0].as_str() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        };
        let port = match configuration[1].as_str() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        };
        let language = match configuration[2].as_str() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        };
        let disabled_rules = match configuration[3].as_array() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        }
        .into_iter()
        .map(|item| item.as_str().unwrap().to_string())
        .collect();
        let disabled_categories = match configuration[4].as_array() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        }
        .into_iter()
        .map(|item| item.as_str().unwrap().to_string())
        .collect();
        let ignore_words = match configuration[5].as_array() {
            Some(v) => v,
            None => return Err(GetConfigurationError::Parse),
        }
        .into_iter()
        .map(|item| item.as_str().unwrap().to_string())
        .collect();

        Ok(Settings {
            host: host.to_string(),
            port: port.to_string(),
            language: language.to_string(),
            disabled_categories: disabled_categories,
            disabled_rules: disabled_rules,
            ignore_words: ignore_words,
        })
    }

    async fn check_file(&self, uri: Url, text: String, version: Option<i32>) {
        let configuration = match self.get_configuration().await {
            Ok(v) => v,
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, e.to_string())
                    .await;
                return;
            }
        };

        self.client
            .log_message(
                MessageType::INFO,
                format!("Checking with client config: {:#?}", configuration),
            )
            .await;

        let problems = check_file(
            &configuration.host,
            &configuration.port,
            uri.as_str(),
            text,
            configuration.language,
            Some(configuration.disabled_rules),
            Some(configuration.disabled_categories),
            Some(configuration.ignore_words),
        )
        .await;

        let problems = match problems {
            Ok(v) => v,
            Err(e) => {
                self.client
                    .show_message(
                        MessageType::ERROR,
                        format!("Failed to check file with languagetool: {}", e.to_string()),
                    )
                    .await;
                return;
            }
        };

        let mut diagnostics = vec![];

        for problem in problems {
            let diagnostic_data = DiagnosticData {
                rule_id: problem.rule_id,
                rule_category: problem.rule_category.clone(),
                match_string: problem.match_string,
                replacements: problem.replacements,
            };
            let data = match serde_json::to_value(diagnostic_data) {
                Ok(v) => v,
                Err(e) => {
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            format!("Failed to serialze diagnostic data: {}", e.to_string()),
                        )
                        .await;
                    continue;
                }
            };

            let diagnostic = Diagnostic {
                range: Range::new(
                    Position {
                        line: problem.range_start.line as u32,
                        character: problem.range_start.column as u32,
                    },
                    Position {
                        line: problem.range_end.line as u32,
                        character: problem.range_end.column as u32,
                    },
                ),
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String(
                    format!("{}", problem.rule_category,),
                )),
                source: Some(String::from("typst-languagetool")),
                message: problem.message,
                data: Some(data),
                ..Default::default()
            };

            diagnostics.push(diagnostic)
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, version)
            .await;
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
struct DiagnosticData {
    pub rule_id: String,
    pub rule_category: String,
    pub match_string: String,
    pub replacements: Vec<String>,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
struct Settings {
    pub host: String,
    pub port: String,
    pub language: String,
    pub disabled_rules: Vec<String>,
    pub disabled_categories: Vec<String>,
    pub ignore_words: Vec<String>,
}
