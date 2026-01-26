use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::*, h_flex, v_flex, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub name: String,
    pub status: String,
    pub error_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualEnv {
    pub id: String,
    pub name: String,
    pub status: String, // 空闲、占用、部署中
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub status: String,
    pub progress: f32,
}

pub struct HiveView {
    active_tab: usize,
    failed_cases: Vec<TestCase>,
    virtual_envs: Vec<VirtualEnv>,
    jobs: Vec<Job>,
}

impl HiveView {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            failed_cases: vec![],
            virtual_envs: vec![],
            jobs: vec![],
        }
    }
}

impl Render for HiveView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _active_tab = self.active_tab;

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .child(
                // 标题区
                h_flex().w_full().items_center().child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Icon::new(IconName::Server).size_6())
                        .child(
                            div()
                                .text_2xl()
                                .font_bold()
                                .text_color(cx.theme().foreground)
                                .child("Hive 平台管理"),
                        ),
                ),
            )
            .child(
                // 内容区
                // 简化实现，只保留失败用例分析功能
                self.render_failed_cases(window, cx),
            )
    }
}

impl HiveView {
    fn render_failed_cases(&self, _window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .w_full()
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        div()
                            .w(px(300.0))
                            .h(px(30.0))
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_md()
                            .p_2()
                            .child("搜索失败用例..."),
                    )
                    .child(
                        Button::new("analyze-btn")
                            .primary()
                            .label("分析失败用例")
                            .icon(Icon::new(IconName::Search)),
                    )
                    .child(
                        Button::new("export-analysis-btn")
                            .ghost()
                            .label("导出分析报告")
                            .icon(Icon::new(IconName::Download)),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_md()
                    .p_4()
                    .child(if self.failed_cases.is_empty() {
                        v_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::CheckCircle2)
                                    .size_16()
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("暂无失败用例"),
                            )
                    } else {
                        div().child(self.render_case_list(cx))
                    }),
            )
    }

    fn render_virtual_envs(&self, _window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .w_full()
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        Button::new("occupy-env-btn")
                            .primary()
                            .label("占用环境")
                            .icon(Icon::new(IconName::Lock)),
                    )
                    .child(
                        Button::new("release-env-btn")
                            .ghost()
                            .label("释放环境")
                            .icon(Icon::new(IconName::Unlock)),
                    )
                    .child(
                        Button::new("deploy-env-btn")
                            .ghost()
                            .label("部署环境")
                            .icon(Icon::new(IconName::Rocket)),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .child(if self.virtual_envs.is_empty() {
                        v_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::CheckCircle2)
                                    .size_16()
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("暂无虚拟环境"),
                            )
                    } else {
                        div().child(self.render_env_grid(cx))
                    }),
            )
    }

    fn render_jobs(&self, _window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .w_full()
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        Button::new("start-job-btn")
                            .primary()
                            .label("拉起 Job")
                            .icon(Icon::new(IconName::Play)),
                    )
                    .child(
                        Button::new("continue-job-btn")
                            .ghost()
                            .label("续跑 Job")
                            .icon(Icon::new(IconName::RotateCw)),
                    )
                    .child(
                        Button::new("analyze-failures-btn")
                            .ghost()
                            .label("组织失败分析")
                            .icon(Icon::new(IconName::FileText)),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_md()
                    .p_4()
                    .child(if self.jobs.is_empty() {
                        v_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Clock)
                                    .size_16()
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(
                                div()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("暂无运行中的 Job"),
                            )
                    } else {
                        div().child(self.render_job_list(cx))
                    }),
            )
    }

    fn render_case_list(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .children(self.failed_cases.iter().map(|case| {
                h_flex()
                    .w_full()
                    .p_3()
                    .gap_3()
                    .bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_md()
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_1()
                            .child(div().font_semibold().child(case.name.clone()))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(format!("ID: {}", case.id)),
                            )
                            .when_some(case.error_msg.as_ref(), |this, msg| {
                                this.child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(0xef4444))
                                        .child(msg.to_string()),
                                )
                            }),
                    )
                    .child(
                        Button::new(format!("view-case-{}", case.id))
                            .small()
                            .ghost()
                            .label("查看详情"),
                    )
            }))
    }

    fn render_env_grid(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .grid()
            .grid_cols(3)
            .gap_4()
            .children(self.virtual_envs.iter().map(|env| {
                v_flex()
                    .p_4()
                    .gap_2()
                    .bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_lg()
                    .child(div().font_semibold().child(env.name.clone()))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("ID: {}", env.id)),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(
                                match env.status.as_str() {
                                    "空闲" => rgb(0x22c55e),
                                    "占用" => rgb(0xf59e0b),
                                    _ => rgb(0x3b82f6),
                                },
                            ))
                            .child(div().text_sm().child(env.status.clone())),
                    )
                    .when_some(env.owner.as_ref(), |this, owner| {
                        this.child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(format!("使用者: {}", owner)),
                        )
                    })
            }))
    }

    fn render_job_list(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .children(self.jobs.iter().map(|job| {
                v_flex()
                    .w_full()
                    .p_3()
                    .gap_2()
                    .bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_md()
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(div().font_semibold().child(job.name.clone()))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(job.status.clone()),
                            ),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .flex_1()
                                    .h(px(8.0))
                                    .bg(cx.theme().border)
                                    .rounded_full()
                                    .child(
                                        div()
                                            .w(relative(job.progress))
                                            .h_full()
                                            .bg(rgb(0x3b82f6))
                                            .rounded_full(),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .child(format!("{:.0}%", job.progress * 100.0)),
                            ),
                    )
            }))
    }
}
