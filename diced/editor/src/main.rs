use std::{
    ffi,
    io,
    path::{
        Path,
        PathBuf,
    },
    sync::Arc,
};

use iced::{
    executor,
    highlighter::{
        self,
        Highlighter,
    },
    theme,
    widget::{
        button,
        container,
        horizontal_space,
        pick_list,
        text,
        text_editor,
        tooltip,
        Column,
        Row,
    },
    Alignment,
    Application,
    Command,
    Element,
    Font,
    Length,
    Settings,
    Theme,
};
pub fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf")
            .as_slice()
            .into()],
        default_font: Font::MONOSPACE,
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

struct Editor {
    file: Option<PathBuf>,         // 文件路径
    content: text_editor::Content, // 编辑器内容
    theme: highlighter::Theme,     // 主题

    /// 标志是否正在执行某种后台操作,比如打开文件,保存文件等.
    /// 1.在提交任何后台任务之前,需要将此状态设置为true
    /// 2.通过判断此状态是否为true,来确定当前是否有后台任务执行
    is_loading: bool,

    /// 标志文件是否有未保存的更改
    is_dirty: bool,
}


#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action), // 编辑器消息
    ThemeSelected(highlighter::Theme),    // 主题选择
    NewFile,                              // 新建
    OpenFile,                             // 打开
    FileOpened(Result<(PathBuf, Arc<String>), Error>), // 打开完成
    SaveFile,                             // 保存
    FileSaved(Result<PathBuf, Error>),    // 保存完成
}

impl Application for Editor {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                file: None,
                content: text_editor::Content::new(),
                theme: highlighter::Theme::SolarizedDark,
                is_loading: true,
                is_dirty: false,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("Editor - Iced")
    }

    fn theme(&self) -> Self::Theme {
        if self
            .theme
            .is_dark()
        {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // 编辑器消息
            Message::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content
                    .perform(action);

                Command::none()
            }
            // 主题选择消息
            Message::ThemeSelected(theme) => {
                self.theme = theme;
                Command::none()
            }
            // 新建文件消息
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Command::none()
            }
            // 打开文件消息 -> 提交异步打开文件任务
            Message::OpenFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;
                    Command::perform(open_file(), Message::FileOpened)
                }
            }
            // 打开完成消息
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Command::none()
            }
            // 保存文件消息 -> 提交异步保存任务
            Message::SaveFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;
                    Command::perform(
                        save_file(
                            self.file
                                .clone(),
                            self.content
                                .text(),
                        ),
                        Message::FileSaved,
                    )
                }
            }
            // 保存完成消息
            Message::FileSaved(result) => {
                self.is_loading = false;
                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                };

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        // 工具栏: 三个操作按钮(打开,新建,保存), 主题选择菜单
        let controls = Row::new()
            .push(action(new_icon(), "New File", Some(Message::NewFile)))
            .push(action(
                open_icon(),
                "Open File",
                (!self.is_loading).then_some(Message::OpenFile),
            ))
            .push(action(
                save_icon(),
                "Save File",
                self.is_dirty
                    .then_some(Message::SaveFile),
            ))
            .push(horizontal_space())
            .push(
                pick_list(
                    highlighter::Theme::ALL,
                    Some(self.theme),
                    Message::ThemeSelected,
                )
                .text_size(14)
                .padding([5, 10]),
            )
            .spacing(10)
            .align_items(Alignment::Center);

        // 状态栏: 文件全路径,光标在文本的位置
        let status = Row::new()
            .push(text(if let Some(path) = &self.file {
                let path = path
                    .display()
                    .to_string();
                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New File")
            }))
            .push(horizontal_space())
            .push(text({
                let (line, column) = self
                    .content
                    .cursor_position();
                format!("{}:{}", line + 1, column + 1)
            }))
            .spacing(10);

        // 组合: 工具栏,编辑区,状态栏
        Column::new()
            .push(controls)
            .push(
                text_editor(&self.content)
                    .height(Length::Fill)
                    .on_action(Message::ActionPerformed)
                    .highlight::<Highlighter>(
                        highlighter::Settings {
                            theme: self.theme,
                            extension: self
                                .file
                                .as_deref()
                                .and_then(Path::extension)
                                .and_then(ffi::OsStr::to_str)
                                .map(str::to_string)
                                .unwrap_or(String::from("rs")),
                        },
                        |highlight, _theme| highlight.to_format(),
                    ),
            )
            .push(status)
            .spacing(10)
            .padding(10)
            .into()
    }
}

/// 动作按钮
///
/// - 传入: 按钮文本(图标),提示文本,按钮动作
/// - 根据按钮动作是否为None设置按钮风格
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(
        container(content)
            .width(30)
            .center_x(),
    );

    if let Some(on_press) = on_press {
        tooltip(action.on_press(on_press), label, tooltip::Position::FollowCursor)
            .style(theme::Container::Box)
            .into()
    } else {
        action
            .style(theme::Button::Secondary)
            .into()
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
}

/// 新建文件图标
fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

/// 保存文件图标
fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

/// 打开文件图标
fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

/// 加载图标字体并应用到对应的文本部件
fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint)
        .font(ICON_FONT)
        .into()
}

/// 通过异步文件对话框打开文件,并读取文件内容
async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(
        picked_file
            .path()
            .to_owned(),
    )
    .await
}

/// 读取文件内容
async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

/// 保存文件内容
/// 如果是新建文件,则传入Path为None,需要通过异步文件对话框指定目标文件
async fn save_file(
    path: Option<PathBuf>,
    contents: String,
) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}
