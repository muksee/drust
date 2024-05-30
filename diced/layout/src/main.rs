use iced::{
    color,
    executor,
    theme,
    widget::{
        button,
        canvas,
        checkbox,
        column,
        container,
        horizontal_space,
        pick_list,
        row,
        scrollable,
        text,
        vertical_rule,
    },
    Alignment,
    Application,
    Color,
    Command,
    Element,
    Font,
    Length,
    Point,
    Renderer,
    Settings,
    Theme,
};

fn main() -> iced::Result {
    Layout::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    ExpalinToggled(bool),
    ThemeSlected(Theme),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Example {
    title: &'static str,
    view: fn() -> Element<'static, Message>,
}

impl Example {
    const LIST: &'static [Self] = &[
        Self {
            title: "Centered",
            view: centered,
        },
        Self {
            title: "Column",
            view: column_,
        },
        Self {
            title: "Row",
            view: row_,
        },
        Self {
            title: "Space",
            view: space,
        },
        Self {
            title: "Application",
            view: application,
        },
        Self {
            title: "Nested Quotes",
            view: nested_quotes,
        },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) = Self::LIST
            .iter()
            .position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) = Self::LIST
            .iter()
            .position(|&example| example == self)
        else {
            return self;
        };

        Self::LIST
            .get(index + 1)
            .copied()
            .unwrap_or(self)
    }
}

impl Default for Example {
    fn default() -> Self {
        Example::LIST[0]
    }
}

fn centered<'a>() -> Element<'a, Message> {
    container(text("I am centered!").size(50))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically",
        square(50),
        square(50),
        square(50),
        "The amount of space between",
        "elements can be configured",
    ]
    .spacing(40)
    .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![
        "A row works like a column...",
        square(50),
        square(50),
        square(50),
        "but lays out widgets horizontally!",
    ]
    .spacing(40)
    .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_items(Alignment::Center),
    )
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();
        container::Appearance::default().with_border(
            palette
                .background
                .strong
                .color,
            1,
        )
    });

    let sidebar = container(
        column!["Siderbar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
    )
    .style(theme::Container::Box)
    .height(Length::Fill)
    .center_y();

    let content =
        container(
            scrollable(
                column![
                    "Content!",
                    square(400),
                    square(200),
                    square(400),
                    "The end",
                ]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

    column![header, row![sidebar, content]].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();
        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: iced::Rectangle,
            _cursor: iced::mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());
            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette
                    .background
                    .strong
                    .color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square)
        .width(size)
        .height(size)
        .into()
}

fn nested_quotes<'a>() -> Element<'a, Message> {
    (1..5)
        .fold(column![text("Original text")].padding(10), |quotes, i| {
            column![
                container(row![vertical_rule(2), quotes].height(Length::Shrink))
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        container::Appearance::default().with_background(
                            if palette.is_dark {
                                Color {
                                    a: 0.01,
                                    ..Color::WHITE
                                }
                            } else {
                                Color {
                                    a: 0.08,
                                    ..Color::BLACK
                                }
                            },
                        )
                    }),
                text(format!("Reply {i}"))
            ]
            .padding(10)
            .padding(10)
        })
        .into()
}

#[derive(Debug)]
struct Layout {
    example: Example,
    explain: bool,
    theme: Theme,
}

impl Application for Layout {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                example: Example::default(),
                explain: false,
                theme: Theme::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "{} - Layout - Iced",
            self.example
                .title
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme
            .clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Next => {
                self.example = self
                    .example
                    .next();
            }
            Message::Previous => {
                self.example = self
                    .example
                    .previous();
            }
            Message::ExpalinToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSlected(theme) => {
                self.theme = theme;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced::{
            keyboard,
            keyboard::key,
        };

        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => {
                Some(Message::Previous)
            }
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let header = row![
            text(
                self.example
                    .title
            )
            .size(20)
            .font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExpalinToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSlected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let example = container(if self.explain {
            (self
                .example
                .view)()
            .explain(color!(0x0000ff))
        } else {
            (self
                .example
                .view)()
        })
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Appearance::default().with_border(
                palette
                    .background
                    .strong
                    .color,
                4.0,
            )
        })
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        let controls = row([
            (!self
                .example
                .is_first())
            .then_some(
                button("<- Previous")
                    .padding([5, 10])
                    .on_press(Message::Previous)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self
                .example
                .is_last())
            .then_some(
                button("Next ->")
                    .padding([5, 10])
                    .on_press(Message::Next)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        column![header, example, controls,]
            .spacing(10)
            .padding(20)
            .into()
    }
}
