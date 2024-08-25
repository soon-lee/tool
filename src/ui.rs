use crate::i18n;
use crate::i18n::I18n;
use iced::widget::{column, row, Button, Column, Image, PickList, Row, Text, TextInput};
use iced::window::Icon;
use iced::{executor, Alignment, Application, Command, Element, Font, Length, Settings, Size, Theme};
use std::borrow::Cow;
use std::sync::Arc;
use iced::alignment::{Horizontal, Vertical};

static JETBRAINS: &[u8] = include_bytes!("../assets/fonts/JetBrainsMonoNL-Medium.ttf");
static FANGSONG: &[u8] = include_bytes!("../assets/fonts/fangsong_gb2312.ttf");

static FONT_JETBRAINS: Font = Font::External {
    name: "JetBrains",
    bytes: JETBRAINS,
};
static FONT_FANGSONG: Font = Font::External {
    name: "FangSong",
    bytes: FANGSONG,
};
pub(crate) struct App {
    i18n: I18n,
    locale: Option<String>,
    theme: Option<String>,
    fruit: String,
    vegetable: String,
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
    Export,
    ChangeLocale(String),
    ChangeTheme(String),
    InputFruit(String),
    SubmitFruit,
    InputVegetable(String),
    SubmitVegetable,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut i18n = I18n::new();
        match i18n.change_locale("zh-CN") {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
        let locale = Some(i18n.translate("ui.tool.locale.zh-CN", "简体中文（中国大陆）"));
        let theme = Some(i18n.translate("ui.tool.theme.dark", "暗黑主题"));
        (
            App {
                i18n,
                locale,
                theme,
                fruit: String::new(),
                vegetable: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        self.i18n.translate("ui.title.text", "张三蔬菜水果店")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Export => Command::none(),
            Message::ChangeLocale(locale) => match locale.as_str() {
                "简体中文（中国大陆）" | "Simplified Chinese (China)" => {
                    match self.i18n.change_locale("zh-CN") {
                        Ok(_) => {
                            self.locale = Some(
                                self.i18n
                                    .translate("ui.tool.locale.zh-CN", "简体中文（中国大陆）"),
                            );
                            match &self.theme {
                                Some(theme) => match theme.as_str() {
                                    "明亮主题" | "Light Theme" => {
                                        self.theme = Some(
                                            self.i18n.translate("ui.tool.theme.light", "明亮主题"),
                                        );
                                    }
                                    "暗黑主题" | "Dark Theme" => {
                                        self.theme = Some(
                                            self.i18n.translate("ui.tool.theme.dark", "暗黑主题"),
                                        );
                                    }
                                    _ => {}
                                },
                                _ => {}
                            };
                            Command::none()
                        }
                        Err(err) => {
                            println!("{}", err);
                            Command::none()
                        }
                    }
                }
                "美式英语" | "American English" => match self.i18n.change_locale("en-US") {
                    Ok(_) => {
                        self.locale = Some(
                            self.i18n
                                .translate("ui.tool.locale.en-US", "American English"),
                        );
                        match &self.theme {
                            Some(theme) => match theme.as_str() {
                                "明亮主题" | "Light Theme" => {
                                    self.theme = Some(
                                        self.i18n.translate("ui.tool.theme.light", "Light Theme"),
                                    );
                                }
                                "暗黑主题" | "Dark Theme" => {
                                    self.theme = Some(
                                        self.i18n.translate("ui.tool.theme.dark", "Dark Theme"),
                                    );
                                }
                                _ => {}
                            },
                            _ => {}
                        };
                        Command::none()
                    }
                    Err(err) => {
                        println!("{}", err);
                        Command::none()
                    }
                },
                _ => Command::none(),
            },
            Message::ChangeTheme(theme) => {
                match theme.as_str() {
                    "明亮主题" | "Light Theme" => {
                        self.theme = Some(self.i18n.translate("ui.tool.theme.light", "明亮主题"));
                    }
                    "暗黑主题" | "Dark Theme" => {
                        self.theme = Some(self.i18n.translate("ui.tool.theme.dark", "暗黑主题"));
                    }
                    _ => {
                        self.theme = Some(self.i18n.translate("ui.tool.theme.dark", "暗黑主题"));
                    }
                }
                Command::none()
            }
            Message::InputFruit(in_str) => {
                self.fruit = in_str;
                Command::none()
            }
            Message::SubmitFruit => Command::none(),
            Message::InputVegetable(in_str) => {
                self.vegetable = in_str;
                Command::none()
            }
            Message::SubmitVegetable => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let toolbar = row![
            row![Button::new(Text::new(self.i18n.translate("ui.tool.file.text", "文件")))
                .on_press(Message::Export).width(50)],
            row![PickList::new(
                vec![
                    self.i18n
                        .translate("ui.tool.locale.zh-CN", "简体中文（中国大陆）"),
                    self.i18n.translate("ui.tool.locale.en-US", "美式英语"),
                ],
                self.locale.clone(),
                |picked| Message::ChangeLocale(picked),
            ).width(150),
            PickList::new(
                vec![
                    self.i18n.translate("ui.tool.theme.light", "明亮主题"),
                    self.i18n.translate("ui.tool.theme.dark", "暗黑主题"),
                ],
                self.theme.clone(),
                |picked| Message::ChangeTheme(picked),
            ).width(90)].spacing(2)
        ].spacing(210).padding(5);

        let fruit = row![
            Text::new(self.i18n.translate("ui.container.fruit.text", "水果")).width(50).height(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
            TextInput::new(
                &self
                    .i18n
                    .translate("ui.container.fruit.placeholder", "请输入水果"),
                &self.fruit,
            )
            .on_input(|in_str| Message::InputFruit(in_str)),
            Text::new(self.i18n.translate("ui.container.status.none", "无状态"),).width(50).height(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
            Button::new(Text::new(
                self.i18n.translate("ui.container.button.submit", "提交"),
            ))
            .on_press(Message::SubmitFruit),
        ].spacing(5).padding(5);

        let vegetable = row![
            Text::new(self.i18n.translate("ui.container.vegetable.text", "蔬菜")).width(50).height(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
            TextInput::new(
                &self
                    .i18n
                    .translate("ui.container.vegetable.placeholder", "请输入蔬菜"),
                &self.vegetable,
            )
            .on_input(|in_str| Message::InputVegetable(in_str)),
            Text::new(self.i18n.translate("ui.container.status.none", "无状态")).width(50).height(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
            Button::new(Text::new(
                self.i18n.translate("ui.container.button.submit", "提交"),
            ))
            .on_press(Message::SubmitVegetable),
        ].spacing(5).padding(5);

        let container = column![fruit, vegetable,];

        let statusbar = column![
            Text::new("1 Fruit"),
            Text::new("2 Vegetable"),
            Text::new("3"),
            Text::new("4"),
            Text::new("5"),
        ];

        column![toolbar, container, statusbar,].spacing(5).into()
    }

    fn theme(&self) -> Self::Theme {
        match &self.theme {
            Some(theme) => match theme.as_str() {
                "明亮主题" | "Light Theme" => Theme::Light,
                "暗黑主题" | "Dark Theme" => Theme::Dark,
                _ => Theme::Dark,
            },
            _ => Theme::Dark,
        }
    }
}
impl App {
    pub(crate) fn work() {
        let icon = match iced::window::icon::from_file("assets/icon.png") {
            Ok(icon) => icon,
            Err(err) => panic!("{}", err),
        };

        let settings = Settings {
            window: iced::window::Settings {
                size: (500, 300),
                icon: Some(Icon::from(icon)),
                ..Default::default()
            },
            default_font: Some(FANGSONG),
            default_text_size: 15f32,
            ..Default::default()
        };

        match App::run(settings) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
    }
}
