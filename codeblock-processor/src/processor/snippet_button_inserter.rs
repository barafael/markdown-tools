use itertools::Itertools;
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
        let hide_other_markers = fence
            .split_whitespace()
            .any(|token| token == "hide_other_markers");

        if let Some(marker) = context {
            let marker = marker.split_once(':').unwrap().1;
            for value in self.snippets.snippets_for_id(marker) {
                let snippet = &value.content;
                let snippet = if hide_other_markers {
                    snippet
                        .lines()
                        .filter(|line| !line.trim().starts_with("// marker-"))
                        .filter(|line| !line.trim().starts_with("# marker-"))
                        .join("\n")
                } else {
                    snippet.clone()
                };
                let dedented = textwrap::dedent(&snippet);
                *current_block = Some(dedented);

                let url = format!(
                    "'vscode://file/'.concat(make_path('{}:{}:{}'))",
                    value.file,
                    value.line + 1,
                    value.col
                );
                *current_url = Some(url);
                *current_btn_text = Some("Open VSCode".into());
            }
        }
    }
}
