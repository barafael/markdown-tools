use pulldown_cmark::CowStr;

pub mod playground_button_inserter;
pub mod snippet_button_inserter;

pub trait ButtonInserter {
    fn handle_codeblock(
        &self,
        fence: &CowStr,
        code: &CowStr,
        current_block: &mut Option<String>,
        current_url: &mut Option<String>,
        current_btn_text: &mut Option<String>,
    );
}
