import { type ExtensionContext, workspace, window, WorkspaceConfiguration, commands } from "vscode";
import * as path from "path";
import * as child_process from "child_process";

import {
	LanguageClient,
	type LanguageClientOptions,
	type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined = undefined;

export function activate(context: ExtensionContext): Promise<void> {
	return startClient(context).catch((e) => {
		void window.showErrorMessage(`Failed to activate typst-languagetool-lsp: ${e}`);
		throw e;
	});
}

async function startClient(context: ExtensionContext): Promise<void> {
	const config = workspace.getConfiguration("typst-languagetool-lsp");
	const serverCommand = getServer(config);
	const run = {
		command: serverCommand,
		options: { env: Object.assign({}, process.env, { RUST_BACKTRACE: "1" }) },
	};
	const serverOptions: ServerOptions = {
		run,
		debug: run,
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "typst" }],
		initializationOptions: config,
	};

	client = new LanguageClient(
		"typst-languagetool-lsp",
		"Typst Languagetool Language Server",
		serverOptions,
		clientOptions
	);

	context.subscriptions.push(
		commands.registerCommand("typst-languagetool-lsp.ignore_word", ignore_word)
	);
	context.subscriptions.push(
		commands.registerCommand("typst-languagetool-lsp.disable_rule", disable_rule)
	);
	context.subscriptions.push(
		commands.registerCommand("typst-languagetool-lsp.disable_category", disable_category)
	);

	return client.start();
}

function ignore_word(word: string) {
	let configuration = workspace.getConfiguration("typst-languagetool-lsp");

	let ignore_words: string[] | undefined = configuration.get("ignore_words");
	if (ignore_words === undefined) {
		ignore_words = [];
	}

	ignore_words.push(word);

	configuration.update("ignore_words", ignore_words, false);
}

function disable_category(category: string) {
	let configuration = workspace.getConfiguration("typst-languagetool-lsp");

	let disabled_categories: string[] | undefined = configuration.get("disabled_categories");
	if (disabled_categories === undefined) {
		disabled_categories = [];
	}

	disabled_categories.push(category);

	configuration.update("disabled_categories", disabled_categories, false);
}

function disable_rule(rule: string) {
	let configuration = workspace.getConfiguration("typst-languagetool-lsp");

	let disabled_rules: string[] | undefined = configuration.get("disabled_rules");
	if (disabled_rules === undefined) {
		disabled_rules = [];
	}

	disabled_rules.push(rule);

	configuration.update("disabled_rules", disabled_rules, false);
}

export function deactivate(): Promise<void> | undefined {
	return client?.stop();
}

function getServer(conf: WorkspaceConfiguration): string {
	const pathInConfig = conf.get<string | null>("serverPath");
	if (pathInConfig !== undefined && pathInConfig !== null && pathInConfig !== "") {
		const validation = validateServer(pathInConfig);
		if (!validation.valid) {
			throw new Error(
				`\`typst-languagetool-lsp.serverPath\` (${pathInConfig}) does not point to a valid typst-languagetool-lsp binary:\n${validation.message}`
			);
		}
		return pathInConfig;
	}
	const windows = process.platform === "win32";
	const suffix = windows ? ".exe" : "";
	const binaryName = "typst-languagetool-lsp" + suffix;

	const bundledPath = path.resolve(__dirname, binaryName);

	const bundledValidation = validateServer(bundledPath);
	if (bundledValidation.valid) {
		return bundledPath;
	}

	const binaryValidation = validateServer(binaryName);
	if (binaryValidation.valid) {
		return binaryName;
	}

	throw new Error(
		`Could not find a valid typst-languagetool-lsp binary.\nBundled: ${bundledValidation.message}\nIn PATH: ${binaryValidation.message}`
	);
}

function validateServer(path: string): { valid: true } | { valid: false; message: string } {
	try {
		const result = child_process.spawnSync(path);
		if (result.status === 0) {
			return { valid: true };
		} else {
			const statusMessage = result.status !== null ? [`return status: ${result.status}`] : [];
			const errorMessage =
				result.error?.message !== undefined ? [`error: ${result.error.message}`] : [];
			const messages = [statusMessage, errorMessage];
			const messageSuffix =
				messages.length !== 0 ? `:\n\t${messages.flat().join("\n\t")}` : "";
			const message = `Failed to launch '${path}'${messageSuffix}`;
			return { valid: false, message };
		}
	} catch (e) {
		if (e instanceof Error) {
			return { valid: false, message: `Failed to launch '${path}': ${e.message}` };
		} else {
			return { valid: false, message: `Failed to launch '${path}': ${JSON.stringify(e)}` };
		}
	}
}
