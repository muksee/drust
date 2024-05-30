use iced::{
    widget::{
        container,
        progress_bar,
        slider,
        text,
        Column,
    },
    Length,
    Sandbox,
    Settings,
};

fn main() -> iced::Result {
    Progress::run(Settings::default())
}

#[derive(Default)]
struct Progress {
    value: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
}

impl Sandbox for Progress {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Progress bar - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SliderChanged(value) => self.value = value,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
            .push(text(format!("Progress: {:.2}%", self.value)))
            .push(
                slider(0.0..=100.0, self.value, Message::SliderChanged)
                    .step(0.01),
            )
            .push(progress_bar(0.0..=100.0, self.value))
            .padding(10)
            .spacing(50);

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
