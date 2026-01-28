use chrono::{DateTime, Utc};
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb,
};
use gpui_component::{ActiveTheme, Icon, IconName, Sizable, StyledExt, button::*, h_flex, v_flex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: String,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub additions: i32,
    pub deletions: i32,
    pub status: String,
}

pub struct CodeHubView {
    mrs: Vec<MergeRequest>,
    filter_start_date: String,
    filter_end_date: String,
}

impl CodeHubView {
    pub fn new() -> Self {
        Self { mrs: vec![], filter_start_date: String::new(), filter_end_date: String::new() }
    }

    fn calculate_stats(&self) -> (i32, i32, usize) {
        let total_additions: i32 = self.mrs.iter().map(|mr| mr.additions).sum();
        let total_deletions: i32 = self.mrs.iter().map(|mr| mr.deletions).sum();
        let total_count = self.mrs.len();
        (total_additions, total_deletions, total_count)
    }
}

impl Render for CodeHubView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (additions, deletions, count) = self.calculate_stats();

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
                        .child(Icon::new(IconName::GitMerge).size_6())
                        .child(
                            div()
                                .text_2xl()
                                .font_bold()
                                .text_color(cx.theme().foreground)
                                .child("CodeHub - MR 管理"),
                        ),
                ),
            )
            .child(
                // 统计卡片区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.stat_card("MR 数量", &count.to_string(), cx))
                    .child(self.stat_card("新增行数", &format!("+{}", additions), cx))
                    .child(self.stat_card("删除行数", &format!("-{}", deletions), cx))
                    .child(self.stat_card("净变化", &format!("{:+}", additions - deletions), cx)),
            )
            .child(
                // 筛选区
                h_flex()
                    .w_full()
                    .gap_4()
                    .items_end()
                    .child(
                        v_flex().gap_1().child(div().text_sm().child("开始日期")).child(
                            div()
                                .w(px(200.0))
                                .h(px(30.0))
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_md()
                                .p_2()
                                .child("YYYY-MM-DD"),
                        ),
                    )
                    .child(
                        v_flex().gap_1().child(div().text_sm().child("结束日期")).child(
                            div()
                                .w(px(200.0))
                                .h(px(30.0))
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_md()
                                .p_2()
                                .child("YYYY-MM-DD"),
                        ),
                    )
                    .child(
                        Button::new("filter-btn")
                            .primary()
                            .label("查询")
                            .icon(Icon::new(IconName::Search)),
                    )
                    .child(
                        Button::new("add-mr-btn")
                            .primary()
                            .label("添加 MR")
                            .icon(Icon::new(IconName::Plus)),
                    ),
            )
            .child(
                // MR 列表
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("MR 列表"))
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(if self.mrs.is_empty() {
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
                                            .child("暂无 MR 数据"),
                                    )
                            } else {
                                div().child(self.render_mr_list(cx))
                            }),
                    ),
            )
    }
}

impl CodeHubView {
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

    fn render_mr_list(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex().w_full().gap_2().children(self.mrs.iter().map(|mr| self.render_mr_item(mr, cx)))
    }

    fn render_mr_item(&self, mr: &MergeRequest, cx: &Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .p_3()
            .gap_3()
            .bg(cx.theme().background)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .hover(|this| this.bg(cx.theme().background))
            .child(
                v_flex()
                    .flex_1()
                    .gap_1()
                    .child(
                        div()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(mr.title.clone()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("#{}", mr.id))
                            .child("•")
                            .child(mr.author.clone())
                            .child("•")
                            .child(mr.created_at.format("%Y-%m-%d %H:%M").to_string()),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x22c55e))
                            .text_color(rgb(0x22c55e))
                            .text_sm()
                            .child(format!("+{}", mr.additions)),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0xef4444))
                            .text_color(rgb(0xef4444))
                            .text_sm()
                            .child(format!("-{}", mr.deletions)),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new(format!("edit-{}", mr.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::PenTool)),
                    )
                    .child(
                        Button::new(format!("delete-{}", mr.id))
                            .small()
                            .ghost()
                            .icon(Icon::new(IconName::Trash)),
                    ),
            )
    }
}
