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
pub struct Issue {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub assignee: String,
}

pub struct DtsView {
    issues: Vec<Issue>,
    filter_status: String,
    search_input: Entity<InputState>,
    search_value: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl DtsView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索问题单..."));

        // 添加测试数据
        let issues = vec![
            Issue {
                id: "DTS001".to_string(),
                title: "登录页面显示异常".to_string(),
                severity: "严重".to_string(),
                status: "提交".to_string(),
                created_at: Utc::now(),
                resolved_at: None,
                assignee: "张三".to_string(),
            },
            Issue {
                id: "DTS002".to_string(),
                title: "数据库连接超时".to_string(),
                severity: "严重".to_string(),
                status: "提交".to_string(),
                created_at: Utc::now(),
                resolved_at: None,
                assignee: "李四".to_string(),
            },
            Issue {
                id: "DTS003".to_string(),
                title: "按钮样式不对齐".to_string(),
                severity: "一般".to_string(),
                status: "提交".to_string(),
                created_at: Utc::now(),
                resolved_at: Some(Utc::now()),
                assignee: "王五".to_string(),
            },
            Issue {
                id: "DTS004".to_string(),
                title: "文本输入框光标位置错误".to_string(),
                severity: "轻微".to_string(),
                status: "提交".to_string(),
                created_at: Utc::now(),
                resolved_at: None,
                assignee: "赵六".to_string(),
            },
        ];

        let subscriptions = vec![cx.subscribe_in(&search_input, window, Self::on_input_event)];

        Self {
            issues,
            filter_status: "all".to_string(),
            search_input,
            search_value: None,
            _subscriptions: subscriptions,
        }
    }

    pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(_window, cx))
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

    fn calculate_stats(&self) -> (usize, usize, usize) {
        let submitted = self.issues.iter().filter(|i| i.status == "提交").count();
        let resolved = self.issues.iter().filter(|i| i.resolved_at.is_some()).count();
        let total = self.issues.len();
        (submitted, resolved, total)
    }
}

impl Render for DtsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().trim().to_lowercase();

        // 根据搜索查询过滤问题单
        let filtered_issues: Vec<&Issue> = self
            .issues
            .iter()
            .filter(|issue: &&Issue| {
                search_query.is_empty()
                    || issue.title.to_lowercase().contains(&search_query)
                    || issue.assignee.to_lowercase().contains(&search_query)
                    || issue.id.to_lowercase().contains(&search_query)
            })
            .collect();

        let (submitted, resolved, total) = self.calculate_stats_filtered(&filtered_issues);

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(
                // 标题区
                h_flex().w_full().items_center().justify_between().child(
                    h_flex().gap_2().items_center().child(Icon::new(IconName::Bug).size_6()).child(
                        div()
                            .text_2xl()
                            .font_bold()
                            .text_color(cx.theme().foreground)
                            .child("DTS - 问题单管理"),
                    ),
                ),
            )
            .child(
                // 统计卡片区
                h_flex()
                    .w_full()
                    .gap_4()
                    .child(self.stat_card("总计问题单", &total.to_string(), cx))
                    .child(self.stat_card("已提交", &submitted.to_string(), cx))
                    .child(self.stat_card("已回归", &resolved.to_string(), cx))
                    .child(self.stat_card(
                        "回归率",
                        &format!(
                            "{:.1}%",
                            if total > 0 { (resolved as f64 / total as f64) * 100.0 } else { 0.0 }
                        ),
                        cx,
                    )),
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
                        Button::new("add-issue-btn")
                            .primary()
                            .label("添加问题单")
                            .icon(Icon::new(IconName::Plus)),
                    )
                    .child(Button::new("refresh-btn").ghost().icon(Icon::new(IconName::RefreshCw))),
            )
            .child(
                // 问题单列表
                v_flex()
                    .flex_1()
                    .w_full()
                    .gap_2()
                    .child(div().text_lg().font_semibold().child("问题单列表"))
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_4()
                            .child(if self.issues.is_empty() {
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
                                            .child("暂无问题单数据"),
                                    )
                            } else {
                                div().child(self.render_issue_list(filtered_issues, cx))
                            }),
                    ),
            )
    }
}

impl DtsView {
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

    fn calculate_stats_filtered(&self, issues: &[&Issue]) -> (usize, usize, usize) {
        let submitted = issues.iter().filter(|i| i.status == "提交").count();
        let resolved = issues.iter().filter(|i| i.resolved_at.is_some()).count();
        let total = issues.len();
        (submitted, resolved, total)
    }

    fn render_issue_list(&self, issues: Vec<&Issue>, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .children(issues.iter().map(|issue| self.render_issue_item(issue, cx)))
    }

    fn render_issue_item(&self, issue: &Issue, cx: &Context<Self>) -> impl IntoElement {
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
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .font_semibold()
                                    .text_color(cx.theme().foreground)
                                    .child(issue.title.clone()),
                            )
                            .child(self.severity_badge(&issue.severity, cx)),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("#{}", issue.id))
                            .child("•")
                            .child(issue.assignee.clone())
                            .child("•")
                            .child(issue.created_at.format("%Y-%m-%d").to_string()),
                    ),
            )
            .child(
                div()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .bg(if issue.resolved_at.is_some() { rgb(0x22c55e) } else { rgb(0xf59e0b) })
                    .text_color(if issue.resolved_at.is_some() {
                        rgb(0x22c55e)
                    } else {
                        rgb(0xf59e0b)
                    })
                    .text_sm()
                    .child(issue.status.clone()),
            )
            .child(
                Button::new(format!("view-{}", issue.id))
                    .small()
                    .ghost()
                    .icon(Icon::new(IconName::ExternalLink)),
            )
    }

    fn severity_badge(&self, severity: &str, _cx: &Context<Self>) -> impl IntoElement {
        let (bg_color, text_color) = match severity {
            "严重" => (rgb(0xef4444), rgb(0xef4444)),
            "一般" => (rgb(0xf59e0b), rgb(0xf59e0b)),
            "轻微" => (rgb(0x3b82f6), rgb(0x3b82f6)),
            _ => (rgb(0x6b7280), rgb(0x6b7280)),
        };
        let severity = severity.to_string();

        div()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(bg_color)
            .text_color(text_color)
            .text_xs()
            .child(severity)
    }
}
