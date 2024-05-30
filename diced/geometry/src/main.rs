use iced::{
    widget::{
        column,
        container,
        scrollable,
    },
    Element,
    Length,
    Sandbox,
    Settings,
};
use rainbow::rainbow;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example;

impl Sandbox for Example {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Custom 2D geometry - Iced")
    }

    fn update(&mut self, _: ()) {}

    fn view(&self) -> Element<()> {
        let content = column![
            rainbow(),
            "In this example we draw a custom widget Rainbow, using \
                 the Mesh2D primitive. This primitive supplies a list of \
                 triangles, expressed as vertices and indices.",
            "Move your cursor over it, and see the center vertex \
                 follow you!",
            "Every Vertex2D defines its own color. You could use the \
                 Mesh2D primitive to render virtually any two-dimensional \
                 geometry for your widget.",
        ]
        .padding(20)
        .spacing(20)
        .max_width(500);

        let scrollable = scrollable(
            container(content)
                .width(Length::Fill)
                .center_x(),
        );

        container(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

mod rainbow {
    use iced::{
        advanced::{
            graphics::{
                color,
                mesh::{
                    self,
                    SolidVertex2D,
                },
                Mesh,
            },
            layout,
            Renderer as _,
            Widget,
        },
        Element,
        Length,
        Renderer,
        Size,
        Theme,
        Vector,
    };

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Rainbow;

    pub fn rainbow() -> Rainbow {
        Rainbow
    }

    impl<Message> Widget<Message, Theme, Renderer> for Rainbow {
        fn size(&self) -> Size<iced::Length> {
            Size {
                width: Length::Fill,
                height: Length::Fill,
            }
        }

        fn layout(
            &self,
            _tree: &mut iced::advanced::widget::Tree,
            _renderer: &Renderer,
            limits: &iced::advanced::layout::Limits,
        ) -> layout::Node {
            let width = limits
                .max()
                .width;

            layout::Node::new(Size::new(width, width))
        }

        fn draw(
            &self,
            _tree: &iced::advanced::widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &iced::advanced::renderer::Style,
            layout: layout::Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            _viewport: &iced::Rectangle,
        ) {
            // 获取部件边界,一般是一个长方形Rectangle
            // 通过边界的长宽来定义渲染图形的画布大小
            // 图像定义时,是使用的画布坐标系,与窗口坐标系无关.
            let bounds = layout.bounds();

            // 画布四条边上的8个顶点以及中心顶点对应的颜色
            // R O Y G B I V C
            let color_r = [1.0, 0.0, 0.0, 1.0];
            let color_o = [1.0, 0.5, 0.0, 1.0];
            let color_y = [1.0, 1.0, 0.0, 1.0];
            let color_g = [0.0, 1.0, 0.0, 1.0];
            let color_gb = [0.0, 1.0, 0.5, 1.0];
            let color_b = [0.0, 0.2, 1.0, 1.0];
            let color_i = [0.5, 0.0, 1.0, 1.0];
            let color_v = [0.75, 0.0, 0.5, 1.0];
            let color_c = [1.0, 1.0, 1.0, 1.0];

            // 中心顶点在画布中的坐标
            // 1.如果鼠标在部件内,则取鼠标在部件中的"相对"坐标(即鼠标在画布坐标系中的坐标)
            // 2.如果鼠标在部件外,则根取画布中心
            let posn_center = {
                if let Some(cursor_position) = cursor.position_in(bounds) {
                    [cursor_position.x, cursor_position.y]
                } else {
                    [bounds.width / 2.0, bounds.height / 2.0]
                }
            };

            // 边上顶点位置
            let posn_tl = [0.0, 0.0]; // 上左
            let posn_t = [bounds.width / 2.0, 0.0]; // 上中
            let posn_tr = [bounds.width, 0.0]; // 上右
            let posn_r = [bounds.width, bounds.height / 2.0]; // 右中
            let posn_br = [bounds.width, bounds.height]; // 下右
            let posn_b = [(bounds.width / 2.0), bounds.height]; // 下中
            let posn_bl = [0.0, bounds.height]; // 下左
            let posn_l = [0.0, bounds.height / 2.0]; // 左中

            // 定义单色网格渲染图像
            let mesh = Mesh::Solid {
                // 画布尺寸
                size: bounds.size(),
                buffers: mesh::Indexed {
                    // 带色彩的二维顶点的列表9个
                    vertices: vec![
                        SolidVertex2D {
                            position: posn_center,
                            color: color::pack(color_c),
                        },
                        SolidVertex2D {
                            position: posn_tl,
                            color: color::pack(color_r),
                        },
                        SolidVertex2D {
                            position: posn_t,
                            color: color::pack(color_o),
                        },
                        SolidVertex2D {
                            position: posn_tr,
                            color: color::pack(color_y),
                        },
                        SolidVertex2D {
                            position: posn_r,
                            color: color::pack(color_g),
                        },
                        SolidVertex2D {
                            position: posn_br,
                            color: color::pack(color_gb),
                        },
                        SolidVertex2D {
                            position: posn_b,
                            color: color::pack(color_b),
                        },
                        SolidVertex2D {
                            position: posn_bl,
                            color: color::pack(color_i),
                        },
                        SolidVertex2D {
                            position: posn_l,
                            color: color::pack(color_v),
                        },
                    ],
                    // 由顶点组合成的一系列三角形8个
                    indices: vec![
                        0, 1, 2, // 上左三角
                        0, 2, 3, // 上中三角
                        0, 3, 4, // 上右三角
                        0, 4, 5, // 右中三角
                        0, 5, 6, // 下右三角
                        0, 6, 7, // 下中三角
                        0, 7, 8, // 下左三角
                        0, 8, 1, // 左中三角
                    ],
                },
            };

            // 将闭包中渲染的图像平移转换到窗口的指定坐标位置
            // 说明:
            // 1.原始图像定义时用的坐标是基于自己的画布,与窗口无关
            // 2.定义好的原始图像需要通过一些转换操作(比如平移),才能显示在窗口指定的位置
            renderer.with_translation(
                Vector::new(bounds.x, bounds.y),
                |renderer| {
                    renderer.draw_mesh(mesh);
                },
            )
        }
    }

    impl<'a, Message> From<Rainbow> for Element<'a, Message> {
        fn from(rainbow: Rainbow) -> Self {
            Self::new(rainbow)
        }
    }
}
