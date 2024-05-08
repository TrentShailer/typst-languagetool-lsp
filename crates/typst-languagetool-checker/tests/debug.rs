use tokio::runtime::Runtime;
use typst_languagetool_checker::check_file;

const FILE: &str = r#""#;

#[test]
fn debug() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let _problems = check_file(
            "https://language.trentshailer.com",
            "",
            "",
            FILE.to_string(),
            "en-GB".to_string(),
            None,
            None,
            None,
        )
        .await;
    });
}
