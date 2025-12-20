use markdown2pdf::{parse_into_file, config::ConfigSource, fonts::FontConfig};
use std::path::PathBuf;

fn main()
{
// Configure fonts for international document
let font_config = FontConfig {
    custom_paths: vec![PathBuf::from("./fonts")],
    default_font: Some("Noto Sans".to_string()),
    code_font: Some("Fira Code".to_string()),
    fallback_fonts: vec![
        "Arial Unicode MS".to_string(),
        "DejaVu Sans".to_string(),
    ],
    enable_subsetting: true,
};

parse_into_file(
    markdown,
    "output.pdf",
    ConfigSource::Default,
    Some(&font_config),
).expect("parse file failed");
}