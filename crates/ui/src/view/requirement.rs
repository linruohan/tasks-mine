use chrono::{DateTime, Utc};
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Subscription, Window, div, rgb,
};
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt,
    button::*,
    h_flex,
    input::{Input, InputEvent, InputState},
    v_flex,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub title: String,
    pub version: String,
    pub test_cycle: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub owner: String,
}

pub struct RequirementView {
    requirements: Vec<Requirement>,
    search_input: Entity<InputState>,
    search_value: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl RequirementView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索需求..."));

        // 添加测试数据
        let requirements = vec![
            Requirement {
                id: "REQ001".to_string(),
                title: "用户认证系统".to_string(),
                version: "v1.0".to_string(),
                test_cycle: "2周".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                status: "进行中".to_string(),
                owner: "张三".to_string(),
            },
            Requirement {
                id: "REQ002".to_string(),
                title: "支付功能模块".to_string(),
                version: "v2.0".to_string(),
                test_cycle: "3周".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                status: "进行中".to_string(),
                owner: "李四".to_string(),
            },
            Requirement {
                id: "REQ003".to_string(),
                title: "数据分析平台".to_string(),
                version: "v1.5".to_string(),
                test_cycle: "4周".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                status: "已完成".to_string(),
                owner: "王五".to_string(),
            },
            Requirement {
                id: "REQ004".to_string(),
                title: "移动端适配".to_string(),
                version: "v1.0".to_string(),
                test_cycle: "2周".to_string(),
                start_date: Utc::now(),
                end_date: Utc::now(),
                status: "延期".to_string(),
                owner: "赵六".to_string(),
            },
        ];

        let subscriptions = vec![cx.subscribe_in(&search_input, window, Self::on_input_event)];

        Self { requirements, search_input, search_value: None, _subscriptions: subscriptions }
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if state == &self.search_input {
                    self.search_value = Some(text.into());
                }
            },
            _ => {},
        };
    }
}

impl Render for RequirementView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().trim().to_lowercase();

        // 根据搜索查询过滤需求
        let filtered_requirements: Vec<&Requirement> = self
            .requirements
            .iter()
            .filter(|req: &&Requirement| {
                search_query.is_empty()
                    || req.title.to_lowercase().contains(&search_query)
                    || req.owner.to_lowercase().contains(&search_query)
                    || req.id.to_lowercase().contains(&search_query)
                    || req.version.to_lowercase().contains(&search_query)
            })
            .collect();

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(
                // 标题区
                h_flex().w_full().items_center().justify_between().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::ListChecks).size_6())
                        .child(
                            div()
                                .text_2xl()
                                .font_bold()
                                .text_color(cx.theme().foreground)
                                .child("需求管理"),
                        ),
                ),
            )
            .child(
                // 统计卡片区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.stat_card("总需求", &filtered_requirements.len().to_string(), cx))
                    .child(
                        self.stat_card(
                            "进行中",
                            &filtered_requirements
                                .iter()
                                .filter(|r| r.status == "进行中")
                                .count()
                                .to_string(),
                            cx,
                        ),
                    )
                    .child(
                        self.stat_card(
                            "已完成",
                            &filtered_requirements
                                .iter()
                                .filter(|r| r.status == "已完成")
                                .count()
                                .to_string(),
                            cx,
                        ),
                    )
                    .child(
                        self.stat_card(
                            "延期",
                            &filtered_requirements
                                .iter()
                                .filter(|r| r.status == "延期")
                                .count()
                                .to_string(),
                            cx,
                        ),
                    ),
            )
            .child(
                // 操作区
                h_flex()
                    .w_full()
                    .gap_4()
                    .items_center()
                    .child(
                        div().flex_1().child(
                            Input::new(&self.search_input)
                                .cleanable(true)
                                .prefix(Icon::new(IconName::Search).size_4()),
                        ),
                    )
                    .child(
                        Button::new("add-req-btn")
                            .primary()
                            .label("添加需求")
                            .icon(Icon::new(IconName::Plus)),
                    )
                    .child(
                        Button::new("import-req-btn")
                            .ghost()
                            .label("导入需求")
                            .icon(Icon::new(IconName::Upload)),
                    ),
            )
            .child(
                // 需求列表
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("需求列表"))
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(if filtered_requirements.is_empty() {
                                v_flex()
                                    .size_full()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .child(
                                        Icon::new(IconName::Inbox)
                                            .size_16()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(div().text_color(cx.theme().muted_foreground).child(
                                        if search_query.is_empty() {
                                            "暂无需求数据"
                                        } else {
                                            "没有找到匹配的需求"
                                        },
                                    ))
                            } else {
                                div().child(self.render_requirement_list(filtered_requirements, cx))
                            }),
                    ),
            )
    }
}

impl RequirementView {
    fn stat_card(&self, label: &str, value: &str, cx: &Context<Self>) -> impl IntoElement {
        let label = label.to_string();
        let value = value.to_string();
        v_flex()
            .flex_1()
            .p_4()
            .gap_2()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_lg()
            .child(div().text_sm().text_color(cx.theme().muted_foreground).child(label))
            .child(div().text_2xl().font_bold().text_color(cx.theme().foreground).child(value))
    }

    fn render_requirement_list(
        &self,
        requirements: Vec<&Requirement>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .children(requirements.iter().map(|req| self.render_requirement_item(req, cx)))
    }

    fn render_requirement_item(&self, req: &Requirement, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .p_4()
            .gap_3()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .hover(|this| this.bg(cx.theme().background))
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .text_lg()
                                    .font_semibold()
                                    .text_color(cx.theme().foreground)
                                    .child(req.title.clone()),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(rgb(0x3b82f6))
                                    .text_color(rgb(0x3b82f6))
                                    .text_xs()
                                    .child(req.version.clone()),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(match req.status.as_str() {
                                "进行中" => rgb(0x3b82f6),
                                "已完成" => rgb(0x22c55e),
                                "延期" => rgb(0xef4444),
                                _ => rgb(0x6b7280),
                            })
                            .text_color(match req.status.as_str() {
                                "进行中" => rgb(0x3b82f6),
                                "已完成" => rgb(0x22c55e),
                                "延期" => rgb(0xef4444),
                                _ => rgb(0x6b7280),
                            })
                            .text_sm()
                            .child(req.status.clone()),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_4()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(Icon::new(IconName::Hash).size_4())
                            .child(format!("ID: {}", req.id)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(Icon::new(IconName::RotateCw).size_4())
                            .child(format!("测试周期: {}", req.test_cycle)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(Icon::new(IconName::User).size_4())
                            .child(format!("负责人: {}", req.owner)),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(Icon::new(IconName::Calendar).size_4())
                            .child(format!(
                                "{} ~ {}",
                                req.start_date.format("%Y-%m-%d"),
                                req.end_date.format("%Y-%m-%d")
                            )),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Button::new(format!("edit-req-{}", req.id))
                                    .small()
                                    .ghost()
                                    .icon(Icon::new(IconName::PenTool)),
                            )
                            .child(
                                Button::new(format!("view-req-{}", req.id))
                                    .small()
                                    .ghost()
                                    .icon(Icon::new(IconName::ExternalLink)),
                            )
                            .child(
                                Button::new(format!("delete-req-{}", req.id))
                                    .small()
                                    .ghost()
                                    .icon(Icon::new(IconName::Trash)),
                            ),
                    ),
            )
    }
}
