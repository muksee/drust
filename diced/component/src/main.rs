use iced::{
    widget::container,
    Length,
    Sandbox,
    Settings,
};
use numeric_input::numeric_input;

fn main() -> iced::Result {
    Component::run(Settings::default())
}

#[derive(Default)]
struct Component {
    value: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NumericInputChanged(Option<u32>),
}

impl Sandbox for Component {
    type Message = Message;

    fn new() -> Self {
        Component::default()
    }

    fn title(&self) -> String {
        String::from("Component - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NumericInputChanged(value) => {
                self.value = value;
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        container(
            numeric_input(self.value).on_change(Message::NumericInputChanged),
        )
        .padding(20)
        .height(Length::Fill)
        .center_y()
        .into()
    }
}

mod numeric_input {
    use iced::{
        alignment::{
            Horizontal,
            Vertical,
        },
        widget::{
            button,
            component,
            text,
            text_input,
            Component,
            Row,
        },
        Alignment,
        Element,
        Length,
        Size,
    };

    /// 组件本体
    pub struct NumericInput<Message> {
        value: Option<u32>,
        on_change: Option<Box<dyn Fn(Option<u32>) -> Message>>,
    }

    /// 便捷函数
    ///
    /// 提供给组件使用者,以函数方式创建组件实例
    pub fn numeric_input<Message>(value: Option<u32>) -> NumericInput<Message> {
        NumericInput::new(value)
    }

    impl<Message> NumericInput<Message> {
        pub fn new(value: Option<u32>) -> Self {
            Self {
                value,
                on_change: None,
            }
        }

        /// 如何为组件自定义类似on_xxx的函数,方便用户传入事件处理函数(钩子)
        ///
        /// 声明一个on_xxx方法,用于对组件持有的事件处理函数进行更新.
        /// 事件处理函数可在组件触发内部消息后更新内部状态(update)时进行调用
        pub fn on_change(
            mut self,
            f: impl Fn(Option<u32>) -> Message + 'static,
        ) -> Self {
            self.on_change = Some(Box::new(f));
            self
        }
    }

    /// 组件内部消息
    ///
    /// 组件的流程:
    /// 1.首次渲染组件视图
    /// 1.视图交互,产生内部消息
    /// 2.内部消息更新内部状态,并向上抛出外部消息
    /// 3.根据内部状态刷新视图
    #[derive(Debug, Clone)]
    pub enum Event {
        InputChanged(String),
        IncrementPressed,
        DecrementPressed,
    }

    /// 组件接口实现
    impl<Message> Component<Message> for NumericInput<Message> {
        type State = ();
        type Event = Event;

        /// 根据内部消息更新内部状态,并转为(返回)外部消息
        ///
        /// 问题:
        /// 外部消息类型未知,如何在视图内部产生能外部消息实现解耦呢?
        /// 在组件中登记产生外部消息的函数,这些函数由用户在使用组件时提供给组件,以此实现解耦.
        fn update(
            &mut self,
            _state: &mut Self::State,
            event: Self::Event,
        ) -> Option<Message> {
            match event {
                Event::IncrementPressed => match self.on_change {
                    Some(ref f) => Some(f(Some(
                        self.value
                            .unwrap_or_default()
                            .saturating_add(1),
                    ))),
                    None => None,
                },
                Event::DecrementPressed => match self.on_change {
                    Some(ref f) => Some(f(Some(
                        self.value
                            .unwrap_or_default()
                            .saturating_sub(1),
                    ))),
                    None => None,
                },
                Event::InputChanged(value) => match self.on_change {
                    Some(ref f) => {
                        if value.is_empty() {
                            Some(f(None))
                        } else {
                            value
                                .parse()
                                .ok()
                                .map(Some)
                                .map(f.as_ref())
                        }
                    }
                    None => None,
                },
            }
        }

        /// 根据内部状体渲染组件视图
        fn view(&self, _state: &Self::State) -> Element<Self::Event> {
            let button = |label, on_press| {
                button(
                    text(label)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .width(40)
                .height(40)
                .on_press(on_press)
            };

            Row::new()
                .push(button("-", Event::DecrementPressed))
                .push(
                    text_input(
                        "Type a number",
                        self.value
                            .as_ref()
                            .map(u32::to_string)
                            .as_deref()
                            .unwrap_or(""),
                    )
                    .on_input(Event::InputChanged)
                    .padding(10),
                )
                .push(button("+", Event::IncrementPressed))
                .align_items(Alignment::Center)
                .spacing(10)
                .into()
        }

        fn size_hint(&self) -> iced::Size<iced::Length> {
            Size {
                width: Length::Fill,
                height: Length::Shrink,
            }
        }
    }

    impl<'a, Message> From<NumericInput<Message>> for Element<'a, Message>
    where
        Message: 'a,
    {
        fn from(numeric_input: NumericInput<Message>) -> Self {
            component(numeric_input)
        }
    }
}
