use iced::{
    alignment::Horizontal,
    executor,
    font,
    theme,
    widget::{
        self,
        button,
        checkbox,
        container,
        keyed_column,
        scrollable,
        text,
        text_input,
        Column,
        Row,
        Text,
    },
    window,
    Alignment,
    Application,
    Color,
    Command,
    Element,
    Font,
    Length,
    Settings,
    Size,
    Theme,
};
use once_cell::sync::Lazy;
use serde::{
    Deserialize,
    Serialize,
};
use std::path::PathBuf;
use uuid::Uuid;

fn main() -> iced::Result {
    Todos::run(Settings {
        window: window::Settings {
            size: Size::new(500.0, 800.0),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug)]
enum Todos {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    FontLoaded(Result<(), font::Error>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessageBridge(usize, TaskMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl Application for Todos {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // 初始化数据的任务
        (
            Todos::Loading,
            Command::batch(vec![
                font::load(include_bytes!("../fonts/icons.ttf").as_slice())
                    .map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Todos::Loading => false,
            Todos::Loaded(state) => state.dirty,
        };
        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced::{
            keyboard,
            keyboard::key,
        };

        // 监视键盘事件
        // 全屏: shift + up
        // 恢复: shift + down
        // ....
        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                _ => None,
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Todos::Loading => {
                match message {
                    // 数据加载成功
                    Message::Loaded(Ok(state)) => {
                        *self = Todos::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            tasks: state.tasks,
                            ..State::default()
                        });
                    }
                    // 数据加载失败
                    Message::Loaded(Err(_)) => {
                        *self = Todos::Loaded(State::default());
                    }
                    _ => {}
                }

                // 异步任务: 焦点定位到输入框
                text_input::focus(INPUT_ID.clone())
            }
            Todos::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    // 输入事件
                    Message::InputChanged(value) => {
                        state.input_value = value;
                        Command::none()
                    }
                    // 创建事件
                    Message::CreateTask => {
                        if !state
                            .input_value
                            .is_empty()
                        {
                            // 插入任务
                            state
                                .tasks
                                .push(Task::new(
                                    state
                                        .input_value
                                        .clone(),
                                ));
                            // 清空输入框
                            state
                                .input_value
                                .clear();
                        }
                        Command::none()
                    }
                    // 切换过滤器
                    Message::FilterChanged(filter) => {
                        state.filter = filter;
                        Command::none()
                    }
                    // 任务控件消息 <- (任务索引,删除消息)
                    Message::TaskMessageBridge(i, TaskMessage::Delete) => {
                        state
                            .tasks
                            .remove(i);
                        Command::none()
                    }
                    // 任务控件消息 <- (任务索引,其它消息)
                    Message::TaskMessageBridge(i, task_message) => {
                        if let Some(task) = state
                            .tasks
                            .get_mut(i)
                        {
                            // 是否为"编辑"指令
                            let should_focus =
                                matches!(task_message, TaskMessage::Edit);

                            // 透传更新子控件状态
                            task.update(task_message);

                            // 如果是编辑指令
                            if should_focus {
                                // 根据任务索引获取输入框ID
                                let id = Task::text_input_id(i);
                                // 任务: 聚焦编辑框,全选中编辑框文本
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    // 保存任务执行完毕
                    Message::Saved(_result) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    }
                    // 响应Tab消息
                    Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    // 响应全屏消息
                    Message::ToggleFullscreen(mode) => {
                        window::change_mode(window::Id::MAIN, mode)
                    }
                    _ => Command::none(),
                };

                // 当收到的消息不是"Saved"消息,则说明需要保存,设置状态为"脏数据"
                if !saved {
                    state.dirty = true;
                }

                // 构建保存任务,条件: 1.非Saved消息 2.没有其它保存操作正在执行
                //
                // state.saving
                // 是一个保存任务的互斥量,一旦有保存任务开始执行,设为true,
                // 保存任务执行完毕设为false*即收到了保存操作执行完毕后发送的Saved消息)
                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            input_value: state
                                .input_value
                                .clone(),
                            filter: state.filter,
                            tasks: state
                                .tasks
                                .clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                // 提交两个异步任务: 普通任务,保存任务
                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match self {
            Todos::Loading => loading_message(),
            Todos::Loaded(State {
                input_value,
                filter,
                tasks,
                ..
            }) => {
                // 标题
                let title = text("todos")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(Horizontal::Center);

                // 新增框
                let input = text_input("What needs to be down?", input_value)
                    .id(INPUT_ID.clone())
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask)
                    .padding(15)
                    .size(30);

                // 控制栏
                let controls = view_controls(tasks, *filter);
                let filtered_tasks = tasks
                    .iter()
                    .filter(|task| filter.matches(task));

                // 任务列表
                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    keyed_column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                (
                                    task.id,
                                    task.view(i)
                                        .map(move |message| {
                                            Message::TaskMessageBridge(i, message)
                                        }),
                                )
                            }),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        Filter::ALL => "You have not crated a task yet",
                        Filter::Active => "All you tasks are done! :D",
                        Filter::Completed => {
                            "You have not completed a task yet..."
                        }
                    })
                };

                // 子元素整合
                let content = Column::new()
                    .push(title)
                    .push(input)
                    .push(controls)
                    .push(tasks)
                    .spacing(20)
                    .width(800);

                // 滚动条
                scrollable(
                    container(content)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    ALL,
    Active,
    Completed,
}

impl Filter {
    fn matches(self, task: &Task) -> bool {
        match self {
            Filter::ALL => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

/// 任务条目 / 任务条目控件
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    description: String,
    completed: bool,

    #[serde(skip)]
    state: TaskState,
}

/// 任务条目控件状态:
/// 1.非编辑状态 - 默认
/// 2.编辑状态
#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Idle
    }
}

/// 任务条目控件的消息
#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

impl Task {
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    fn new(description: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::FinishEdition => {
                if !self
                    .description
                    .is_empty()
                {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    // 根据任务条目控件的状态是否是编辑状态,进行控件渲染

    fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed)
                    .on_toggle(TaskMessage::Completed)
                    .width(Length::Fill)
                    .text_shaping(text::Shaping::Advanced);

                Row::new()
                    .push(checkbox)
                    .push(
                        button(edit_icon())
                            .on_press(TaskMessage::Edit)
                            .padding(10)
                            .style(theme::Button::Text),
                    )
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into()
            }
            TaskState::Editing => {
                let text_input =
                    text_input("Describe your task...", &self.description)
                        .id(Self::text_input_id(i))
                        .on_input(TaskMessage::DescriptionEdited)
                        .on_submit(TaskMessage::FinishEdition)
                        .padding(10);

                Row::new()
                    .push(text_input)
                    .push(
                        button(
                            Row::new()
                                .push(delete_icon())
                                .push("Delete"),
                        )
                        .on_press(TaskMessage::Delete)
                        .padding(10)
                        .style(theme::Button::Destructive),
                    )
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into()
            }
        }
    }
}

/// 子元素: 图标
fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(Horizontal::Center)
}

/// 子元素: 控制栏[任务完成摘要,筛选按钮列表]
fn view_controls(tasks: &[Task], current_filter: Filter) -> Element<Message> {
    // 未完成项目统计
    let tasks_left = tasks
        .iter()
        .filter(|task| !task.completed)
        .count();

    // 三个筛选按钮,根据当前是否被激活来确定渲染颜色
    let filter_button = |label, filter, current_filter| {
        let label = text(label);
        let button = button(label).style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button
            .on_press(Message::FilterChanged(filter))
            .padding(8)
    };

    Row::new()
        .push(text(format!(
            "{tasks_left} {} left",
            if tasks_left == 1 { "task" } else { "tasks" }
        )))
        .push(
            Row::new()
                .push(filter_button("ALL", Filter::ALL, current_filter))
                .push(filter_button("Active", Filter::Active, current_filter))
                .push(filter_button(
                    "Completed",
                    Filter::Completed,
                    current_filter,
                ))
                .width(Length::Shrink)
                .spacing(10),
        )
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
}

/// 子元素: 加载界面
fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("loading....")
            .horizontal_alignment(Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

/// 子元素: 列表为空界面
fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .height(200)
    .center_y()
    .into()
}

/// 全量数据内存映像,和硬盘数据对应
/// 包含:
/// 过滤条件 / 正在输入内容 / 已创建条目列表
///
/// 操作:
/// 状态文件路径 / 读取状态 / 保存状态
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}

/// 状态加载错误
#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

/// 状态保存错误
#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    /// 获取文件路径
    fn path() -> PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("rs", "Iced", "Todos")
        {
            project_dirs
                .data_dir()
                .into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("todos.json");
        path
    }

    /// 从文件读取状态
    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    /// 向文件保存状态
    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json =
            serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl SavedState {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;

        window
            .local_storage()
            .ok()?
    }

    async fn load() -> Result<SavedState, LoadError> {
        let storage = Self::storage().ok_or(LoadError::File)?;

        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::File)?
            .ok_or(LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::File)?;

        let json =
            serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::Write)?;

        let _ = wasm_timer::Delay::new(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

/// 图标字体
const ICONS: Font = Font::with_name("Iced-Todos-Icons");

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
