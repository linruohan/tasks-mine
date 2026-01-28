pub mod app;
mod app_menus;
mod themes;
mod title_bar;
mod utils;
pub mod view;
use gpui::{
    Action, AnyElement, AnyView, App, AppContext, Bounds, Context, Div, Entity, FocusHandle,
    Focusable, Global, InteractiveElement, IntoElement, KeyBinding, ParentElement, Pixels, Render,
    RenderOnce, SharedString, Size, StyleRefinement, Styled, Window, WindowBounds, WindowKind,
    WindowOptions, actions, div, px, rems, size,
};
use gpui_component::{
    ActiveTheme, Root, TitleBar, WindowExt,
    dock::Panel,
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    notification::Notification,
    scroll::ScrollbarShow,
    v_flex,
};
use serde::Deserialize;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::title_bar::AppTitleBar;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectScrollbarShow(ScrollbarShow);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectLocale(SharedString);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectFont(usize);

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct SelectRadius(usize);

actions!(ui, [
    About,
    Open,
    Quit,
    ToggleSearch,
    TestAction,
    Tab,
    TabPrev,
    ShowPanelInfo,
    ToggleListActiveHighlight
]);

pub struct AppState {
    pub invisible_panels: Entity<Vec<SharedString>>,
}
impl AppState {
    fn init(cx: &mut App) {
        let state = Self { invisible_panels: cx.new(|_| Vec::new()) };
        cx.set_global::<AppState>(state);
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

pub fn create_new_window<F, E>(title: &str, crate_view_fn: F, cx: &mut App)
where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    create_new_window_with_size(title, None, crate_view_fn, cx);
}

pub fn create_new_window_with_size<F, E>(
    title: &str,
    window_size: Option<Size<Pixels>>,
    crate_view_fn: F,
    cx: &mut App,
) where
    E: Into<AnyView>,
    F: FnOnce(&mut Window, &mut App) -> E + Send + 'static,
{
    let mut window_size = window_size.unwrap_or(size(px(1600.0), px(1200.0)));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = window_size.width.min(display_size.width * 0.85);
        window_size.height = window_size.height.min(display_size.height * 0.85);
    }
    let window_bounds = Bounds::centered(None, window_size, cx);
    let title = SharedString::from(title.to_string());

    cx.spawn(async move |cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(gpui::Size { width: px(480.), height: px(320.) }),
            kind: WindowKind::Normal,
            #[cfg(target_os = "linux")]
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(gpui::WindowDecorations::Client),
            ..Default::default()
        };

        let window = cx
            .open_window(options, |window, cx| {
                let view = crate_view_fn(window, cx);
                let story_root = cx.new(|cx| StoryRoot::new(title.clone(), view, window, cx));

                // Set focus to the StoryRoot to enable it's actions.
                let focus_handle = story_root.focus_handle(cx);
                window.defer(cx, move |window, cx| {
                    focus_handle.focus(window, cx);
                });

                cx.new(|cx| Root::new(story_root, window, cx))
            })
            .expect("failed to open window");

        window
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title(&title);
            })
            .expect("failed to update window");

        Ok::<_, anyhow::Error>(())
    })
    .detach();
}

impl Global for AppState {}

pub fn init(cx: &mut App) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("tasks-mine=trace".parse().unwrap()),
        )
        .init();

    gpui_component::init(cx);
    AppState::init(cx);
    themes::init(cx);

    cx.bind_keys([
        KeyBinding::new("/", ToggleSearch, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-o", Open, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-o", Open, None),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-q", Quit, None),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("alt-f4", Quit, None),
    ]);

    cx.on_action(|_: &Quit, cx: &mut App| {
        cx.quit();
    });

    cx.on_action(|_: &About, cx: &mut App| {
        if let Some(window) = cx.active_window().and_then(|w| w.downcast::<Root>()) {
            cx.defer(move |cx| {
                window
                    .update(cx, |root, window, cx| {
                        root.push_notification(
                            "GPUI Component Storybook\nVersion 0.1.0",
                            window,
                            cx,
                        );
                    })
                    .unwrap();
            });
        }
    });

    cx.activate(true);
}

#[derive(IntoElement)]
struct StorySection {
    base: Div,
    title: SharedString,
    sub_title: Vec<AnyElement>,
    children: Vec<AnyElement>,
}

impl StorySection {
    pub fn sub_title(mut self, sub_title: impl IntoElement) -> Self {
        self.sub_title.push(sub_title.into_any_element());
        self
    }

    #[allow(unused)]
    fn max_w_md(mut self) -> Self {
        self.base = self.base.max_w(rems(48.));
        self
    }

    #[allow(unused)]
    fn max_w_lg(mut self) -> Self {
        self.base = self.base.max_w(rems(64.));
        self
    }

    #[allow(unused)]
    fn max_w_xl(mut self) -> Self {
        self.base = self.base.max_w(rems(80.));
        self
    }

    #[allow(unused)]
    fn max_w_2xl(mut self) -> Self {
        self.base = self.base.max_w(rems(96.));
        self
    }
}

impl ParentElement for StorySection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for StorySection {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for StorySection {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        GroupBox::new()
            .id(self.title.clone())
            .outline()
            .title(
                h_flex()
                    .justify_between()
                    .w_full()
                    .gap_4()
                    .child(self.title)
                    .children(self.sub_title),
            )
            .content_style(
                StyleRefinement::default()
                    .rounded(cx.theme().radius_lg)
                    .overflow_x_hidden()
                    .items_center()
                    .justify_center(),
            )
            .child(self.base.children(self.children))
    }
}

pub(crate) fn section(title: impl Into<SharedString>) -> StorySection {
    StorySection {
        title: title.into(),
        sub_title: vec![],
        base: h_flex().flex_wrap().justify_center().items_center().w_full().gap_4(),
        children: vec![],
    }
}

struct StoryRoot {
    focus_handle: FocusHandle,
    title_bar: Entity<AppTitleBar>,
    view: AnyView,
}

impl StoryRoot {
    pub fn new(
        title: impl Into<SharedString>,
        view: impl Into<AnyView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let title_bar = cx.new(|cx| AppTitleBar::new(title, window, cx));
        Self { focus_handle: cx.focus_handle(), title_bar, view: view.into() }
    }

    fn on_action_panel_info(
        &mut self,
        _: &ShowPanelInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        struct Info;
        let note = Notification::new().message("You have clicked panel info.").id::<Info>();
        window.push_notification(note, cx);
    }

    fn on_action_toggle_search(
        &mut self,
        _: &ToggleSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.propagate();
        if window.has_focused_input(cx) {
            return;
        }

        struct Search;
        let note = Notification::new().message("You have toggled search.").id::<Search>();
        window.push_notification(note, cx);
    }
}

impl Focusable for StoryRoot {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StoryRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .id("story-root")
            .on_action(cx.listener(Self::on_action_panel_info))
            .on_action(cx.listener(Self::on_action_toggle_search))
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(self.title_bar.clone())
                    .child(
                        div()
                            .track_focus(&self.focus_handle)
                            .flex_1()
                            .overflow_hidden()
                            .child(self.view.clone()),
                    )
                    .children(sheet_layer)
                    .children(dialog_layer)
                    .children(notification_layer),
            )
    }
}
