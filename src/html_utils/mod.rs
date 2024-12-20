use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = renderMarkdownToHTML)]
    pub fn render_markdown_to_html(markdown: &str) -> String;
}

pub fn make_tag(text: &str, color: &str) -> String {
    let color_class = format!("inline-flex items-center rounded-md px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
    format!(r#"<span class="{}">{}</span>"#, color_class, text)
}
