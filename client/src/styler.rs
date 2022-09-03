use std::collections::HashMap;
use stylist::{style, Style};

pub fn build(css: &str) -> Style {
    let css_str = format!(r#"{}"#, css);
    Style::new(css_str).unwrap_or(style!(r#""#).unwrap())
}

lazy_static! {
    pub static ref PALETTE: HashMap<&'static str, &'static str> = HashMap::from([
        ("primary", "hsl(225, 69%, 56%)"),
        ("primary-hover", "hsl(229, 57%, 51%)"),
        ("primary-active", "hsl(230, 51%, 63%)"),
        ("primary-transparent", "hsla(225, 69%, 56%, 0.5)"),
        ("light-base", "hsl(250, 65%, 95%)"),
        ("dark-base", "hsl(250, 65%, 5%)"),
        ("success", "hsl(155, 100%, 45%)"),
        ("error", "hsl(0, 75%, 40%)"),
    ]);
}
