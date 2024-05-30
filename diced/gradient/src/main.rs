use iced::{
    application,
    gradient,
    theme,
    widget::{
        checkbox,
        column,
        container,
        horizontal_space,
        row,
        slider,
        text,
        Container,
    },
    window,
    Background,
    Color,
    Element,
    Length,
    Radians,
    Sandbox,
    Settings,
    Theme,
};

fn main() -> iced::Result {
    Gradient::run(Settings {
        window: window::Settings {
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
struct Gradient {
    start: Color,
    end: Color,
    angle: Radians,
    transparent: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    StartChanged(Color),
    EndChanged(Color),
    AngleChanged(Radians),
    TransparentToggled(bool),
}

impl Sandbox for Gradient {
    type Message = Message;

    fn new() -> Self {
        Self {
            start: Color::WHITE,
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: Radians(0.0),
            transparent: false,
        }
    }

    fn title(&self) -> String {
        String::from("Gradient - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::StartChanged(color) => self.start = color,
            Message::EndChanged(color) => self.end = color,
            Message::AngleChanged(angle) => self.angle = angle,
            Message::TransparentToggled(transparent) => {
                self.transparent = transparent;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let Self {
            start,
            end,
            angle,
            transparent,
        } = *self;

        // 部件
        let gradient_box = container(horizontal_space())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_: &_| {
                // 创建一个渐变对象
                let gradient = gradient::Linear::new(angle)
                    .add_stop(0.0, start)
                    .add_stop(1.0, end)
                    .into();

                // 创建一个容器外观,将背景设置为渐变对象.
                container::Appearance {
                    background: Some(Background::Gradient(gradient)),
                    ..Default::default()
                }
            });

        // 部件
        let angle_picker = row![
            text("Angle").width(64),
            slider(Radians::RANGE, self.angle, Message::AngleChanged).step(0.01),
        ]
        .spacing(0)
        .padding(8)
        .align_items(iced::Alignment::Center);

        // 部件
        let transparent_toggle = Container::new(
            checkbox("Transparent window", transparent)
                .on_toggle(Message::TransparentToggled),
        )
        .padding(8);

        // 部件
        let display_panel = row![
            text(format!("Start: {:?}", self.start)),
            text(format!("End: {:?}", self.end)),
            text(format!("Angle: {:?}", self.angle))
        ]
        .padding(8)
        .spacing(8);

        // 部件整合
        column![
            color_picker("Start", self.start).map(Message::StartChanged),
            color_picker("End", self.end).map(Message::EndChanged),
            angle_picker,
            display_panel,
            transparent_toggle,
            gradient_box,
        ]
        .into()
    }

    fn style(&self) -> iced::theme::Application {
        if self.transparent {
            theme::Application::custom(|theme: &Theme| application::Appearance {
                background_color: Color::TRANSPARENT,
                text_color: theme
                    .palette()
                    .text,
            })
        } else {
            theme::Application::Default
        }
    }
}

fn color_picker(label: &str, color: Color) -> Element<'_, Color> {
    row![
        text(label).width(64),
        slider(0.0..=1.0, color.r, move |r| { Color { r, ..color } }).step(0.01),
        slider(0.0..=1.0, color.g, move |g| { Color { g, ..color } }).step(0.01),
        slider(0.0..=1.0, color.b, move |b| { Color { b, ..color } }).step(0.01),
        slider(0.0..=1.0, color.a, move |a| { Color { a, ..color } }).step(0.01),
    ]
    .spacing(0)
    .padding(8)
    .align_items(iced::Alignment::Center)
    .into()
}
