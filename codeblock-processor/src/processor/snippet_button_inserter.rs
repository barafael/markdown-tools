use pulldown_cmark::CowStr;
use snippet::Snippets;

use super::ButtonInserter;

#[derive(Debug)]
pub struct SnippetButtonInserter {
    snippets: Snippets,
}

impl SnippetButtonInserter {
    pub fn with_snippets(snippets: Snippets) -> Self {
        Self { snippets }
    }
}

impl ButtonInserter for SnippetButtonInserter {
    fn handle_codeblock(
        &self,
        fence: &CowStr,
        _code: &CowStr,
        current_block: &mut Option<String>,
        current_url: &mut Option<String>,
        current_btn_text: &mut Option<String>,
    ) {
        let context = fence
            .split_whitespace()
            .filter(|s| s.starts_with("marker:"))
            .collect::<Vec<_>>()
            .pop();
        if let Some(marker) = context {
            let marker = marker.split_once(':').unwrap().1;
            if let Some(value) = self.snippets.get(marker) {
                let snippet = &value.content;
                let dedented = textwrap::dedent(snippet);
                *current_block = Some(dedented);

                let url = if value.file.is_absolute() {
                    format!(
                        "vscode://file{}:{}:{}",
                        value.file.display(),
                        value.line + 1,
                        value.col
                    )
                } else {
                    todo!("handle relative paths using window.location.href");
                };
                *current_url = Some(url);
                *current_btn_text = Some("Open VSCode".into());
            }
        }
    }
}
