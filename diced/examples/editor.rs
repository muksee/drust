use std::path::PathBuf;

use iced::{
    highlighter,
    widget::text_editor,
};

struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
}

fn main() {}
