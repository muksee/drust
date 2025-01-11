use std::{
    f32::consts::PI,
    time::Instant,
};

use iced::{
    application,
    executor,
    mouse::Cursor,
    theme,
    widget::{
        canvas,
        canvas::{
            gradient,
            stroke,
            Path,
            Stroke,
        },
    },
    window,
    Application,
    Color,
    Command,
    Element,
    Length,
    Point,
    Renderer,
    Settings,
    Size,
    Subscription,
    Theme,
    Vector,
};

fn main() -> iced::Result {
    SolarSystem::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct SolarSystem {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
}

impl Application for SolarSystem {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            SolarSystem {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Solar system - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(instant) => {
                self.state
                    .update(instant);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        canvas(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::custom(dark_background)
    }

    fn subscription(&self) -> Subscription<Message> {
        // 根据窗口刷新周期发送携带当前时间的消息
        // 由此顶球触发太阳系图的刷新,实现动画效果
        window::frames().map(Message::Tick)
    }
}

#[derive(Debug)]
struct State {
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    start: Instant,
    now: Instant,
    stars: Vec<(Point, f32)>,
}

impl State {
    // 太阳半径
    const SUN_RADIUS: f32 = 70.0;
    // 地球公转轨道半径
    const ORBIT_RADIUS: f32 = 150.0;
    // 地球半径
    const EARTH_RADIUS: f32 = 12.0;
    // 月球半径
    const MOON_RADIUS: f32 = 4.0;
    // 地月距离
    const MOON_DISTANCE: f32 = 28.0;

    pub fn new() -> State {
        let now = Instant::now();
        let size = window::Settings::default().size;

        State {
            space_cache: canvas::Cache::new(),
            system_cache: canvas::Cache::default(),
            start: now,
            now,
            stars: Self::genrate_stars(size.width, size.height),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        // 要先清除此帧,因为每次的帧内容不同
        self.system_cache
            .clear();
    }

    pub fn genrate_stars(width: f32, height: f32) -> Vec<(Point, f32)> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        // 在画布中随即生成100个背景星星的坐标点,这个坐标是以太阳为原点的.因此坐标有负有正
        // 星星实际渲染为正方形,边长随机
        (0..100)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range((-width / 2.0)..(width / 2.0)),
                        rng.gen_range((-height / 2.0)..(height / 2.0)),
                    ),
                    rng.gen_range(0.5..1.5),
                )
            })
            .collect()
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<<Renderer as canvas::Renderer>::Geometry> {
        // 以帧中心为原点,绘制宇宙背景帧
        let background = self
            .space_cache
            .draw(renderer, bounds.size(), |frame| {
                let stars = Path::new(|path| {
                    for (p, size) in &self.stars {
                        path.rectangle(*p, Size::new(*size, *size));
                    }
                });

                frame.translate(frame.center() - Point::ORIGIN);
                frame.fill(&stars, Color::WHITE);
            });

        // 绘制太阳系帧
        let system = self
            .system_cache
            .draw(renderer, bounds.size(), |frame| {
                // 太阳系中心坐标
                let center = frame.center();

                // 太阳
                let sun = Path::circle(center, Self::SUN_RADIUS);
                // 地球轨道
                let orbit = Path::circle(center, Self::ORBIT_RADIUS);

                // 绘制太阳
                frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));
                // 绘制地球轨道
                frame.stroke(
                    &orbit,
                    Stroke {
                        style: stroke::Style::Solid(Color::from_rgba8(
                            0, 153, 255, 0.1,
                        )),
                        width: 1.0,
                        line_dash: canvas::LineDash {
                            offset: 0,
                            segments: &[3.0, 6.0],
                        },
                        ..Stroke::default()
                    },
                );

                // 流逝时间
                let elapsed = self.now - self.start;
                // 根据流逝时间计算坐标旋转弧度,分别计算秒和毫秒两部分相加,1秒6度1毫秒0.006度
                let rotation = (2.0 * PI / 60.0) * elapsed.as_secs() as f32
                    + (2.0 * PI / 60_000.0) * elapsed.subsec_millis() as f32;

                frame.with_save(|frame| {
                    // 原点移动到帧中心
                    frame.translate(Vector::new(center.x, center.y));
                    // 旋转坐标系
                    frame.rotate(rotation);
                    // 原点沿x轴移动到地球轨道上
                    frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.0));

                    // 地球
                    let earth = Path::circle(Point::ORIGIN, Self::EARTH_RADIUS);

                    // 绘制地球,从面向太阳的一侧到背对太阳一侧进行渐变填充
                    let earth_fill = gradient::Linear::new(
                        Point::new(-Self::EARTH_RADIUS, 0.0),
                        Point::new(Self::EARTH_RADIUS, 0.0),
                    )
                    .add_stop(0.2, Color::from_rgb(0.15, 0.50, 1.0))
                    .add_stop(0.8, Color::from_rgb(0.0, 0.20, 0.47));
                    frame.fill(&earth, earth_fill);

                    frame.with_save(|frame| {
                        // 月球公转角速度是地球公转角速度的约10倍,因此同个周期转过的角度是地球的约10倍
                        frame.rotate(rotation * 10.0);
                        // 原点沿Y轴移动到月球轨道上
                        frame.translate(Vector::new(0.0, Self::MOON_DISTANCE));

                        // 月球
                        let moon = Path::circle(Point::ORIGIN, Self::MOON_RADIUS);
                        // 绘制月球
                        frame.fill(&moon, Color::WHITE);
                    });
                });
            });

        // 返回帧组
        vec![background, system]
    }
}
