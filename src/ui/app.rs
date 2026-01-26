use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    button::*, h_flex, scroll::ScrollableElement, v_flex, ActiveTheme, Icon, IconName,
    StyledExt,
};

use crate::tools::{
    codehub::CodeHubView, dts::DtsView, excel::ExcelView, hive::HiveView,
    requirement::RequirementView,
};

pub struct App {
    active_tab: usize,
}

impl App {
    pub fn new() -> Self {
        Self { active_tab: 0 }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_tab = self.active_tab;

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                // 顶部导航栏
                h_flex()
                    .w_full()
                    .h(px(60.0))
                    .bg(cx.theme().title_bar)
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .items_center()
                    .px_4()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Icon::new(IconName::Workflow).size_6())
                            .child(
                                div()
                                    .text_xl()
                                    .font_bold()
                                    .text_color(cx.theme().foreground)
                                    .child("Tasks Mine"),
                            ),
                    ),
            )
            .child(
                // 主内容区域
                h_flex()
                    .flex_1()
                    .w_full()
                    .child(
                        // 左侧导航
                        v_flex()
                            .w(px(200.0))
                            .h_full()
                            .bg(cx.theme().background)
                            .border_r_1()
                            .border_color(cx.theme().border)
                            .p_2()
                            .gap_1()
                            .child(self.nav_button("CodeHub", IconName::GitMerge, 0, window, cx))
                            .child(self.nav_button("DTS", IconName::Bug, 1, window, cx))
                            .child(self.nav_button(
                                "Excel",
                                IconName::FileSpreadsheet,
                                2,
                                window,
                                cx,
                            ))
                            .child(self.nav_button("Hive", IconName::Server, 3, window, cx))
                            .child(self.nav_button(
                                "Requirement",
                                IconName::ListChecks,
                                4,
                                window,
                                cx,
                            )),
                    )
                    .child(
                        // 右侧内容区
                        div()
                            .flex_1()
                            .h_full()
                            .bg(cx.theme().background)
                            .overflow_y_scrollbar()
                            .child(self.render_content(active_tab, window, cx)),
                    ),
            )
    }
}

impl App {
    fn nav_button(
        &self,
        label: &str,
        icon: IconName,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.active_tab == index;
        let label = label.to_string();

        Button::new(format!("nav-{}", index))
            .justify_start()
            .gap_2()
            .when(is_active, |this: Button| this.primary())
            .icon(Icon::new(icon))
            .label(label)
            .on_click(cx.listener(move |view, _, _, cx| {
                view.active_tab = index;
                cx.notify();
            }))
    }

    fn render_content(
        &self,
        active_tab: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match active_tab {
            0 => div().child(cx.new(|_| CodeHubView::new())),
            1 => div().child(cx.new(|_| DtsView::new())),
            2 => div().child(cx.new(|_| ExcelView::new())),
            3 => div().child(cx.new(|_| HiveView::new())),
            4 => div().child(cx.new(|_| RequirementView::new())),
            _ => div().child("Unknown view"),
        }
    }
}
