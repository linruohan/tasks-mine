use chrono::{DateTime, Utc};
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, div, px, rgb,
};
use gpui_component::{ActiveTheme, Icon, IconName, Sizable, StyledExt, button::*, h_flex, v_flex};
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
}

impl RequirementView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { requirements: vec![] }
    }
}

impl Render for RequirementView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(self.stat_card("总需求", &self.requirements.len().to_string(), cx))
                    .child(
                        self.stat_card(
                            "进行中",
                            &self
                                .requirements
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
                            &self
                                .requirements
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
                            &self
                                .requirements
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
                        div()
                            .w(px(300.0))
                            .h(px(30.0))
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_2()
                            .child("搜索需求..."),
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
                            .child(if self.requirements.is_empty() {
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
                                    .child(
                                        div()
                                            .text_color(cx.theme().muted_foreground)
                                            .child("暂无需求数据"),
                                    )
                            } else {
                                div().child(self.render_requirement_list(cx))
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

    fn render_requirement_list(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .children(self.requirements.iter().map(|req| self.render_requirement_item(req, cx)))
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
