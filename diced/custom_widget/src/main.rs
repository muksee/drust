use iced::{
    advanced::{
        layout,
        renderer,
        widget,
        Widget,
    },
    widget::{
        column,
        container,
        slider,
        text,
    },
    Alignment,
    Border,
    Color,
    Element,
    Length,
    Sandbox,
    Settings,
    Size,
};
fn main() -> iced::Result {
    Example::run(Settings::default())
}

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }
}

pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::with_radius(self.radius),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Circle>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Circle) -> Self {
        Self::new(circle)
    }
}

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example { radius: 50.0 }
    }
    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::RadiusChanged(value) => self.radius = value,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = column![
            circle(self.radius),
            text(format!("Radius: {:.2}", self.radius)),
            slider(1.0..=100.0, self.radius, Message::RadiusChanged).step(0.01),
        ]
        .padding(20)
        .spacing(20)
        // .max_width(500)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
