use std::fmt::Display;

use iced::{
    widget::{
        column,
        pick_list,
        row,
        scrollable,
        text,
        vertical_space,
    },
    Alignment,
    Length,
    Sandbox,
    Settings,
};

fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    LanguageSelected(Language),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    Elm,
    Ruby,
    Haskell,
    C,
    Javascript,
    Other,
}

impl Language {
    const ALL: [Language; 7] = [
        Language::C,
        Language::Elm,
        Language::Ruby,
        Language::Haskell,
        Language::Rust,
        Language::Javascript,
        Language::Other,
    ];
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Haskell => "Haskell",
                Language::C => "C",
                Language::Javascript => "Javascript",
                Language::Other => "Some other language",
            }
        )
    }
}

#[derive(Default)]
struct Example {
    selected_language: Option<Language>,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Picklist - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = Some(language);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let pick_list = pick_list(
            &Language::ALL[..],
            self.selected_language,
            Message::LanguageSelected,
        )
        .placeholder("Choose a language...");

        let content = column![
            vertical_space().height(200),
            "which is your favourate language?",
            row![
                pick_list,
                text(format!(
                    "  Favourate:{}",
                    self.selected_language
                        .unwrap_or_default()
                )),
            ],
            vertical_space().height(200),
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        scrollable(content).into()
    }
}
