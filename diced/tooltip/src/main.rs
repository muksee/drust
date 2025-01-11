//! # tooltip
//!
//! ## 什么是tooltip:
//! 当鼠标覆盖到某个部件上时,会显示一个提示信息,这个提示信息即为此部件的tooltip
//!
//! ## 创建tooltip
//! 1.将产生tooltip的部件进行包装
//! 2.提供tooltip的显示文本内容
//! 3.提供tooltip的弹出位置: 上下左右,或者跟随鼠标

use iced::{
    theme,
    widget::{
        button,
        container,
        tooltip,
        tooltip::Position,
    },
    Length,
    Sandbox,
    Settings,
};

fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    position: Position,
}

#[derive(Debug, Clone)]
enum Message {
    ChangePosition,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            position: Position::Bottom,
        }
    }

    fn title(&self) -> String {
        String::from("Tooltip - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangePosition => {
                self.position = match &self.position {
                    Position::FollowCursor => Position::Top,
                    Position::Top => Position::Bottom,
                    Position::Bottom => Position::Left,
                    Position::Left => Position::Right,
                    Position::Right => Position::FollowCursor,
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let tooltip = tooltip(
            button("Press to change position").on_press(Message::ChangePosition),
            position_to_text(self.position),
            self.position,
        )
        .gap(10)
        .style(theme::Container::Box);

        container(tooltip)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn position_to_text<'a>(position: Position) -> &'a str {
    match position {
        Position::FollowCursor => "Follow Cursor",
        Position::Top => "Top",
        Position::Bottom => "Botton",
        Position::Left => "Left",
        Position::Right => "Right",
    }
}
