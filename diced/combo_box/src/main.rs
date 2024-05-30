use std::fmt::Display;

use iced::{
    widget::{
        column,
        combo_box,
        container,
        text,
        vertical_space,
    },
    Length,
    Sandbox,
    Settings,
};

fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    languages: combo_box::State<Language>,
    selected_language: Option<Language>,
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Selected(Language),
    OptionHoverd(Language),
    Closed,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            languages: combo_box::State::new(Language::ALL.to_vec()),
            selected_language: None,
            text: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Combox_box - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Selected(language) => {
                self.selected_language = Some(language);
                self.text = language
                    .hello()
                    .to_string();
            }
            Message::OptionHoverd(language) => {
                self.text = language
                    .hello()
                    .to_string();
            }
            Message::Closed => {
                self.text = self
                    .selected_language
                    .map(|langage| {
                        langage
                            .hello()
                            .to_string()
                    })
                    .unwrap_or_default();
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let combox_box = combo_box(
            &self.languages,
            "type a language",
            self.selected_language
                .as_ref(),
            Message::Selected,
        )
        .on_option_hovered(Message::OptionHoverd)
        .on_close(Message::Closed)
        .width(250);

        let content = column![
            text(&self.text),
            "what is your language?",
            combox_box,
            vertical_space().height(150),
        ]
        .width(Length::Fill)
        .align_items(iced::Alignment::Center)
        .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    Danish,
    #[default]
    English,
    French,
    German,
    Italian,
    Portuguese,
    Spanish,
    Other,
}

impl Language {
    const ALL: [Language; 8] = [
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Portuguese,
        Language::Spanish,
        Language::Other,
    ];

    fn hello(&self) -> &str {
        match self {
            Language::Danish => "Halloy!",
            Language::English => "Hello!",
            Language::French => "Salut!",
            Language::German => "Hallo!",
            Language::Italian => "Ciao!",
            Language::Portuguese => "Olá!",
            Language::Spanish => "¡Hola!",
            Language::Other => "... hello?",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Danish => "Danish",
                Language::English => "English",
                Language::French => "French",
                Language::German => "German",
                Language::Italian => "Italian",
                Language::Portuguese => "Portuguese",
                Language::Spanish => "Spanish",
                Language::Other => "Some other language",
            }
        )
    }
}
