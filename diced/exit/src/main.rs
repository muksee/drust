use iced::{
    executor,
    widget::{
        button,
        container,
        Column,
    },
    window,
    Alignment,
    Application,
    Command,
    Length,
    Settings,
    Theme,
};

fn main() -> iced::Result {
    Exit::run(Settings::default())
}

#[derive(Default)]
struct Exit {
    show_confirm: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Confirm,
    Exit,
}

impl Application for Exit {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Exit = Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Confirm => window::close(window::Id::MAIN),
            Message::Exit => {
                self.show_confirm = true;
                Command::none()
            }
        }
    }

    fn view(
        &self,
    ) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let content = if self.show_confirm {
            Column::new()
                .push("Are you sure you want exit?")
                .push(
                    button("Yes, exit now!")
                        .padding([10, 20])
                        .on_press(Message::Confirm),
                )
        } else {
            Column::new()
                .push("Click the button to exit")
                .push(
                    button("Exit")
                        .padding([10, 20])
                        .on_press(Message::Exit),
                )
        }
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
