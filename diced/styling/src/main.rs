use iced::{
    widget::{
        button,
        checkbox,
        column,
        container,
        horizontal_rule,
        pick_list,
        progress_bar,
        row,
        scrollable,
        slider,
        text,
        text_input,
        toggler,
        vertical_rule,
        vertical_space,
    },
    Alignment,
    Element,
    Length,
    Sandbox,
    Settings,
    Theme,
};

fn main() -> iced::Result {
    Stlying::run(Settings::default())
}

#[derive(Default)]
struct Stlying {
    theme: Theme,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    TogglerToggled(bool),
    Checked(bool),
}

impl Sandbox for Stlying {
    type Message = Message;

    fn new() -> Self {
        Stlying::default()
    }

    fn title(&self) -> String {
        String::from("Stlying - Iced")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::InputChanged(value) => {
                println!("Input cheanged: {value:?}");
                self.input_value = value
            }
            Message::ButtonPressed => {
                println!("Button pressed")
            }
            Message::SliderChanged(value) => {
                println!("Slider changed: {value:?}");
                self.slider_value = value
            }
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::Checked(value) => {
                println!("Checkbox pressed: {value}");
                self.checkbox_value = value
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // theme block
        let choose_theme = column![
            text("theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                .width(Length::Fill),
        ]
        .spacing(10);

        // input
        let text_input = text_input("Typing something...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);

        // button
        let button = button("submit")
            .padding(10)
            .on_press(Message::ButtonPressed);

        // slider and progress
        let slider =
            slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = progress_bar(0.0..=100.0, self.slider_value);

        // scrollable block
        let scrollable = scrollable(column![
            "Scroll me!",
            vertical_space().height(800),
            "You did it!"
        ])
        .width(Length::Fill)
        .height(100);

        // checkbox
        let checkbox = checkbox("Check me!", self.checkbox_value)
            .on_toggle(Message::Checked);

        // toggler
        let toggler = toggler(
            String::from("Toggle me"),
            self.toggler_value,
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10);

        // collect all
        let content = column![
            choose_theme,
            horizontal_rule(38),
            row![text_input, button]
                .spacing(10)
                .align_items(Alignment::Center),
            slider,
            progress_bar,
            row![
                scrollable,
                vertical_rule(38),
                column![checkbox, toggler]
                    .spacing(20)
                    .spacing(2)
            ]
            .spacing(10)
            .height(100)
            .align_items(Alignment::Center),
        ]
        .spacing(10)
        .padding(20)
        .max_width(600);

        // container
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
