use std::collections::HashMap;
use stylist::{style, Style};

pub fn build(css: &str) -> Style {
  let css_str = format!(r#"{}"#, css);
  Style::new(css_str).unwrap_or(style!(r#""#).unwrap())
}
