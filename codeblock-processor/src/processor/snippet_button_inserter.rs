use pulldown_cmark::CowStr;
use snippet_extractor::Snippets;

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
            for value in self.snippets.snippets_for_id(marker) {
                let snippet = &value.content;
                let dedented = textwrap::dedent(snippet);
                *current_block = Some(dedented);

                let url = format!(
                    "'vscode://file/'.concat(make_path('{}:{}:{}'))",
                    value.file.display(),
                    value.line + 1,
                    value.col
                );
                *current_url = Some(url);
                *current_btn_text = Some("Open VSCode".into());
            }
        }
    }
}
