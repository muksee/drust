//! 窗口事件监视器

use iced::{
    alignment,
    event,
    executor,
    widget::{
        button,
        checkbox,
        container,
        text,
        Column,
    },
    window,
    Alignment,
    Application,
    Command,
    Element,
    Event,
    Length,
    Settings,
    Theme,
};

fn main() -> iced::Result {
    Events::run(Settings {
        window: window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Events {
    last: Vec<Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Toogled(bool),
    Exit,
}

impl Application for Events {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last
                    .push(event);
                if self
                    .last
                    .len()
                    > 20
                {
                    let _ = self
                        .last
                        .remove(0);
                }
                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(id, window::Event::CloseRequested) = event {
                    window::close(id)
                } else {
                    Command::none()
                }
            }
            Message::Toogled(enabled) => {
                self.enabled = enabled;
                Command::none()
            }
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn view(
        &self,
    ) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let events = Column::with_children(
            self.last
                .iter()
                .map(|event| text(format!("{event:?}")).size(20))
                .map(Element::from),
        );

        let toggle = checkbox("Listen to runtime events", self.enabled)
            .on_toggle(Message::Toogled);

        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
