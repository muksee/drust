use std::{fs, path::PathBuf};

extern crate pulldown_cmark;
extern crate structopt;

use pulldown_cmark::{html, Parser};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mdp", about = "Markdown to HTML render CLI")]
pub struct Options {
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

pub fn render_markdown(markdown: String) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(&markdown[..]);
    html::push_html(&mut html_buf, parser);
    html_buf
}

fn main() {
    let option = Options::from_args();
    let contents =
        fs::read_to_string(option.filename).expect("Something went wrong reading the file");

    let result = render_markdown(contents);
    println!("{}", result);
}
