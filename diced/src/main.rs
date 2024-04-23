use iced::widget::{
    button,
    column,
    text,
    text_input,
};

use iced::{
    Alignment,
    Element,
    Sandbox,
    Settings,
};

fn main() -> iced::Result {
    println!("Hello, world!");

    Counter::run(Settings::default())
}

struct Counter {
    value: i64,
    name: &'static str,
}

impl Sandbox for Counter {
    type Message = Message;

    // 构造入口
    fn new() -> Self {
        Self { value: 0, name: "" }
    }


    // 窗口标题
    fn title(&self) -> String {
        String::from("Counter - iced")
    }

    // 视图
    // 与视图交互触发各种消息
    fn view(&self) -> Element<Self::Message> {
        column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            text_input("input something", self.name),
            button("-").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    // 收到消息后,根据消息内容更新状态,影响视图
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}
