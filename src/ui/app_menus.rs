use gpui::{App, Entity, Menu, MenuItem, SharedString};
use gpui_component::{menu::AppMenuBar, ActiveTheme as _, Theme, ThemeMode, ThemeRegistry};

use crate::ui::theme::{SwitchTheme, SwitchThemeMode};

pub fn init(title: impl Into<SharedString>, cx: &mut App) -> Entity<AppMenuBar> {
    let app_menu_bar = AppMenuBar::new(cx);
    let title: SharedString = title.into();
    update_app_menu(title.clone(), app_menu_bar.clone(), cx);

    // 主题变化时刷新菜单勾选状态
    cx.observe_global::<Theme>({
        let title = title.clone();
        let app_menu_bar = app_menu_bar.clone();
        move |cx| {
            update_app_menu(title.clone(), app_menu_bar.clone(), cx);
        }
    })
    .detach();

    app_menu_bar
}

fn update_app_menu(title: impl Into<SharedString>, app_menu_bar: Entity<AppMenuBar>, cx: &mut App) {
    let mode = cx.theme().mode;
    cx.set_menus(vec![Menu {
        name: title.into(),
        items: vec![
            MenuItem::Submenu(Menu {
                name: "Appearance".into(),
                items: vec![
                    MenuItem::action("Light", SwitchThemeMode(ThemeMode::Light))
                        .checked(!mode.is_dark()),
                    MenuItem::action("Dark", SwitchThemeMode(ThemeMode::Dark))
                        .checked(mode.is_dark()),
                ],
            }),
            theme_menu(cx),
        ],
    }]);

    app_menu_bar.update(cx, |menu_bar, cx| {
        menu_bar.reload(cx);
    })
}

fn theme_menu(cx: &App) -> MenuItem {
    let themes = ThemeRegistry::global(cx).sorted_themes();
    let current_name = cx.theme().theme_name();
    MenuItem::Submenu(Menu {
        name: "Theme".into(),
        items: themes
            .iter()
            .map(|theme| {
                let checked = current_name == &theme.name;
                MenuItem::action(theme.name.clone(), SwitchTheme(theme.name.clone()))
                    .checked(checked)
            })
            .collect(),
    })
}
