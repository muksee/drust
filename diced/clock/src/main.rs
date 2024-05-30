use std::f32::consts::PI;

use iced::{
    executor,
    mouse,
    widget::{
        canvas,
        canvas::{
            stroke,
            Cache,
            Geometry,
            LineCap,
            Path,
            Stroke,
        },
        container,
    },
    Application,
    Color,
    Command,
    Element,
    Length,
    Point,
    Rectangle,
    Renderer,
    Settings,
    Subscription,
    Theme,
    Vector,
};
use time::OffsetDateTime;

pub fn main() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    now: time::OffsetDateTime,
    clock: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                clock: Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                    self.clock
                        .clear();
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::Tick(time::OffsetDateTime::now_local().unwrap_or_else(|e| {
                println!("local time failed, fallback to utc: {e:?}");
                time::OffsetDateTime::now_utc()
            }))
        })
    }
}

impl<Message> canvas::Program<Message> for Clock {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self
            .clock
            .draw(renderer, bounds.size(), |frame| {
                // 获取画布中心坐标,作为时钟圆心
                let center = frame.center();
                // 计算圆的半径,使圆刚好填满画布
                let radius = frame
                    .width()
                    .min(frame.height())
                    / 2.0;

                // 声明圆形
                let background = Path::circle(center, radius);
                // 填充圆形
                frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));

                // 声明线段,表示时针
                let short_hand =
                    Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

                // 声明线段,表示分针和秒针
                let long_hand =
                    Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

                // 秒针宽度设为圆半径的百分之一,时针和分针设为秒针的3倍
                let width = radius / 100.0;

                // 秒针风格:
                // 宽度/线型/末端形状
                let thin_stroke = || -> Stroke {
                    Stroke {
                        width,
                        style: stroke::Style::Solid(Color::WHITE),
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    }
                };

                // 时针和分针风格
                let wide_stroke = || -> Stroke {
                    Stroke {
                        width: width * 3.0,
                        style: stroke::Style::Solid(Color::WHITE),
                        line_cap: LineCap::Round,
                        ..Stroke::default()
                    }
                };

                // 绘图坐标系原点移动到圆心,在此坐标原点绘制表针
                frame.translate(Vector::new(center.x, center.y));

                // 绘制时针:
                // 1.根据时间计算出时针的弧度
                // 2.将绘图坐标系顺时针旋转对应弧度
                // 3.使用线段风格wide_stoke绘制时针线段
                // 4.恢复绘图状态: 取消第2步的旋转
                frame.with_save(|frame| {
                    frame.rotate(hand_rotation_hour(self.now));
                    frame.stroke(&short_hand, wide_stroke());
                });

                // 绘制分针
                frame.with_save(|frame| {
                    frame.rotate(hand_rotation_minute(self.now));
                    frame.stroke(&long_hand, wide_stroke());
                });

                // 绘制秒针
                frame.with_save(|frame| {
                    frame.rotate(hand_rotation_second(self.now));
                    frame.stroke(&long_hand, thin_stroke());
                });
            });

        vec![clock]
    }
}

/// 计算表针的弧度
// fn hand_rotation(n: u8, total: u8) -> f32 {
//     let turns = n as f32 / total as f32;
//     2.0 * std::f32::consts::PI * turns
// }

fn hand_rotation_second(t: OffsetDateTime) -> f32 {
    let s = t.second();
    (s as f32 / 60f32) * 2f32 * PI
}

fn hand_rotation_hour(t: OffsetDateTime) -> f32 {
    let (h, m, s) = (t.hour(), t.minute(), t.second());

    println!("{h},{m},{s}");

    ((h as u32 * 3600 + m as u32 * 60 + s as u32) as f32 / 43200f32) * 2f32 * PI
}

fn hand_rotation_minute(t: OffsetDateTime) -> f32 {
    let (m, s) = (t.minute(), t.second());
    ((m as u32 * 60 + s as u32) as f32 / 3600f32) * 2f32 * PI
}
