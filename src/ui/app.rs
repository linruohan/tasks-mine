use gpui::{
    div, prelude::FluentBuilder, px, AnyView, AppContext, Context, Entity, InteractiveElement,
    IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled, Subscription, Window,
};
use gpui_component::{
    h_flex,
    input::{Input, InputEvent, InputState},
    resizable::{h_resizable, resizable_panel},
    sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
    v_flex, ActiveTheme, Icon, IconName,
};

use super::view::{CodeHubView, DtsView, ExcelView, HiveView, RequirementView};

struct StoryItem {
    name: &'static str,
    description: &'static str,
    icon: IconName,
    view: AnyView,
}

impl StoryItem {
    fn new(name: &'static str, description: &'static str, icon: IconName, view: AnyView) -> Self {
        Self { name, description, icon, view }
    }
}

pub struct TasksApp {
    stories: Vec<StoryItem>,
    active_index: Option<usize>,
    collapsed: bool,
    search_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl TasksApp {
    pub fn new(init_story: Option<&str>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("搜索功能..."));
        let stories = Self::build_stories(window, cx);
        let mut this = Self {
            stories,
            active_index: Some(0),
            collapsed: false,
            search_input,
            _subscriptions: vec![],
        };

        if let Some(init_story) = init_story {
            this.set_active_story(init_story);
        }

        let subscription = cx.subscribe(&this.search_input, |this, _, event, cx| {
            if matches!(event, InputEvent::Change) {
                this.active_index = Some(0);
                cx.notify();
            }
        });
        this._subscriptions.push(subscription);

        this
    }

    pub fn view(init_story: Option<&str>, window: &mut Window, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|cx| Self::new(init_story, window, cx))
    }

    fn build_stories(_window: &mut Window, cx: &mut Context<Self>) -> Vec<StoryItem> {
        vec![
            StoryItem::new(
                "CodeHub",
                "MR 管理与统计",
                IconName::GitMerge,
                cx.new(|_| CodeHubView::new()).into(),
            ),
            StoryItem::new(
                "DTS",
                "问题单管理与统计",
                IconName::Bug,
                cx.new(|_| DtsView::new()).into(),
            ),
            StoryItem::new(
                "Excel",
                "Excel 读写工具",
                IconName::FileSpreadsheet,
                cx.new(|_| ExcelView::new()).into(),
            ),
            StoryItem::new(
                "Hive",
                "Hive 平台快速入口",
                IconName::Server,
                cx.new(|_| HiveView::new()).into(),
            ),
            StoryItem::new(
                "Requirement",
                "需求管理",
                IconName::ListChecks,
                cx.new(|_| RequirementView::new()).into(),
            ),
        ]
    }

    fn set_active_story(&mut self, name: &str) {
        if let Some(index) =
            self.stories.iter().position(|story| story.name.eq_ignore_ascii_case(name))
        {
            self.active_index = Some(index);
        }
    }
}

impl Render for TasksApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query = self.search_input.read(cx).value().trim().to_lowercase();
        let filtered: Vec<(usize, &StoryItem)> = self
            .stories
            .iter()
            .enumerate()
            .filter(|(_, story)| story.name.to_lowercase().contains(&query))
            .collect();

        let active_idx = self.active_index.unwrap_or(0);
        let active_story =
            filtered.iter().find(|(idx, _)| *idx == active_idx).or_else(|| filtered.first());

        let story_name = active_story.map(|(_, story)| story.name).unwrap_or("");
        let story_desc = active_story.map(|(_, story)| story.description).unwrap_or("");

        v_flex().size_full().bg(cx.theme().background).child(
            h_resizable("app-shell")
                .child(
                    resizable_panel().size(px(255.)).size_range(px(200.)..px(320.)).child(
                        Sidebar::new("app-sidebar")
                            .w_full()
                            .border_0()
                            .collapsed(self.collapsed)
                            .header(
                                v_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        SidebarHeader::new()
                                            .w_full()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .rounded(cx.theme().radius)
                                                    .bg(cx.theme().primary)
                                                    .text_color(cx.theme().primary_foreground)
                                                    .size_8()
                                                    .flex_shrink_0()
                                                    .child(Icon::new(IconName::GalleryVerticalEnd)),
                                            )
                                            .when(!self.collapsed, |this| {
                                                this.child(
                                                    v_flex()
                                                        .gap_0()
                                                        .text_sm()
                                                        .flex_1()
                                                        .line_height(px(20.0))
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child("Tasks Mine")
                                                        .child(
                                                            div()
                                                                .text_color(
                                                                    cx.theme().muted_foreground,
                                                                )
                                                                .child("功能导航")
                                                                .text_xs(),
                                                        ),
                                                )
                                            }),
                                    )
                                    .child(
                                        div()
                                            .bg(cx.theme().sidebar_accent)
                                            .rounded_full()
                                            .px_1()
                                            .flex_1()
                                            .mx_1()
                                            .child(
                                                Input::new(&self.search_input)
                                                    .appearance(false)
                                                    .cleanable(true),
                                            ),
                                    ),
                            )
                            .children(filtered.iter().map(|(idx, story)| {
                                let idx = *idx;
                                SidebarGroup::new(story.name).child(
                                    SidebarMenu::new().child(
                                        SidebarMenuItem::new(story.name)
                                            .active(self.active_index == Some(idx))
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.active_index = Some(idx);
                                                cx.notify();
                                            })),
                                    ),
                                )
                            })),
                    ),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .h_full()
                        .overflow_x_hidden()
                        .child(
                            h_flex()
                                .id("header")
                                .p_4()
                                .border_b_1()
                                .border_color(cx.theme().border)
                                .justify_between()
                                .items_start()
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .child(div().text_xl().child(story_name))
                                        .child(
                                            div()
                                                .text_color(cx.theme().muted_foreground)
                                                .child(story_desc),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .id("story")
                                .flex_1()
                                .overflow_y_scroll()
                                .when_some(active_story, |this, (_, story)| {
                                    this.child(story.view.clone())
                                }),
                        )
                        .into_any_element(),
                ),
        )
    }
}
