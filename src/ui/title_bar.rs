use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Context, Entity, IntoElement, ParentElement, Render, SharedString,
    Styled, Window,
};
use gpui_component::{
    badge::Badge,
    button::{Button, ButtonVariants},
    label::Label,
    menu::AppMenuBar,
    ActiveTheme as _, IconName, Sizable, TitleBar, WindowExt,
};

use crate::ui::app_menus;

pub struct AppTitleBar {
    app_menu_bar: Entity<AppMenuBar>,
    child: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
}

impl AppTitleBar {
    pub fn new(title: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let app_menu_bar = app_menus::init(title, cx);
        Self { app_menu_bar, child: Rc::new(|_, _| div().into_any_element()) }
    }

    pub fn child<F, E>(mut self, f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        self.child = Rc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();

        TitleBar::new().child(div().flex().items_center().child(self.app_menu_bar.clone())).child(
            div()
                .flex()
                .items_center()
                .justify_end()
                .px_2()
                .gap_2()
                .child((self.child.clone())(window, cx))
                .child(Label::new("theme:").secondary(cx.theme().theme_name()).text_sm())
                .child(
                    div().relative().child(
                        Badge::new().count(notifications_count).max(99).child(
                            Button::new("bell").small().ghost().compact().icon(IconName::Bell),
                        ),
                    ),
                ),
        )
    }
}
