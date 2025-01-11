use std::{
    f32::consts::PI,
    time::Instant,
};

use iced::{
    executor,
    mouse::Cursor,
    widget::{
        canvas::{
            self,
            stroke,
            Cache,
            Path,
            Stroke,
        },
        Canvas,
    },
    Application,
    Command,
    Element,
    Length,
    Point,
    Settings,
    Subscription,
    Theme,
};

pub fn main() -> iced::Result {
    Arc::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Arc {
    start: Instant,
    cache: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl Application for Arc {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Arc {
                start: Instant::now(),
                cache: Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Arc - Iced")
    }

    fn update(&mut self, _: Message) -> Command<Message> {
        self.cache
            .clear();

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Tick)
    }
}

impl<Message> canvas::Program<Message> for Arc {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<<iced::Renderer as canvas::Renderer>::Geometry> {
        let geometry = self
            .cache
            .draw(renderer, bounds.size(), |frame| {
                // 取当前主题调色板
                let palette = theme.palette();

                // 帧中心坐标
                let center = frame.center();

                // 轨道圆的半径
                //
                // 轨道圆:
                // 以帧中心为圆心,以帧尺寸的五分之一为半径
                //
                // 作用:
                // 曲线的起点固定与轨道圆的正上方
                // 曲线的终点以10秒为周期沿着轨道圆运动
                let radius = frame
                    .width()
                    .min(frame.height())
                    / 5.0;

                // 轨道圆
                let orbit = Path::circle(center, radius);
                // 绘制线条
                frame.stroke(
                    &orbit,
                    Stroke {
                        style: stroke::Style::Solid(palette.text),
                        width: 1.0,
                        line_dash: canvas::LineDash {
                            offset: 0,
                            segments: &[3.0, 6.0],
                        },
                        ..Stroke::default()
                    },
                );

                // 曲线起点: 位于轨道圆最高点,即y轴正向与轨道圆的交点
                let start = Point::new(center.x, center.y - radius);

                // 曲线终点运动弧度计算:
                // 由于需要终点沿着轨道运动一周需要10秒,因此
                // 将流逝的时间以10秒划为一个周期,计算当前时间在本次10秒周期中的进度
                let angle = (self
                    .start
                    .elapsed()
                    .as_millis()
                    % 10_000) as f32
                    / 10_000.0
                    * 2.0
                    * PI;

                // 曲线终点: 当前时间对应的弧度在圆周上的点
                let end = Point::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                );

                // 路径声明: 用于绘制起点和终点的小圆圈
                // 1.曲线起点为一个小圆
                // 2.画笔移动到曲线终点
                // 3.曲线终点为一个小圆
                let circles = Path::new(|b| {
                    b.circle(start, 10.0);
                    b.move_to(end);
                    b.circle(end, 10.0);
                });

                // 绘制图形
                frame.fill(&circles, palette.text);

                // 路径声明: 用于绘制连接起点和终点的曲线
                // 1.画笔移动到曲线起点
                // 2.绘制弧线
                // 3.直线到终点
                let path = Path::new(|b| {
                    b.move_to(start);
                    b.arc_to(center, end, 50.0);
                    b.line_to(end);
                });

                // 绘制线条
                frame.stroke(
                    &path,
                    Stroke {
                        style: stroke::Style::Solid(palette.text),
                        width: 10.0,
                        ..Stroke::default()
                    },
                );
            });

        vec![geometry]
    }
}
