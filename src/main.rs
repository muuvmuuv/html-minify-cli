use minify_html::{minify, Cfg};
use pretty_bytes::converter::convert;
use std::fs;

fn main() {
    let file_path = std::env::args().nth(1).expect("No HTML file given");
    let out_file = std::env::args().nth(2).expect("No out file given");

    let contents = fs::read(file_path).expect("Something went wrong reading the file");
    println!("File size: {}", convert(contents.len() as f64));
    let contents: &[u8] = &contents;

    let mut cfg = Cfg::new();
    cfg.keep_html_and_head_opening_tags = true;
    cfg.remove_bangs = true;
    cfg.remove_processing_instructions = true;
    cfg.minify_css = true;
    cfg.minify_js = true;

    let minified = minify(&contents, &cfg);
    println!("Minified size: {}", convert(minified.len() as f64));

    let html = String::from_utf8(minified.clone()).unwrap();
    fs::write(out_file, html).expect("Unable to write file");
}
