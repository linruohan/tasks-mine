use std::path::PathBuf;

use gpui::{Action, App, SharedString};
use gpui_component::{Theme, ThemeMode, ThemeRegistry};

#[derive(Action, Clone, Copy, PartialEq, Eq)]
#[action(namespace = theme, no_json)]
pub struct SwitchThemeMode(pub ThemeMode);

#[derive(Action, Clone, PartialEq, Eq)]
#[action(namespace = theme, no_json)]
pub struct SwitchTheme(pub SharedString);

pub fn init(cx: &mut App) {
    // 尝试加载 ./themes 目录的自定义主题（忽略失败即可）
    let _ = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, |_cx| {});

    // 监听主题切换动作
    cx.on_action(|switch: &SwitchThemeMode, cx: &mut App| {
        Theme::change(switch.0, None, cx);
        cx.refresh_windows();
    });

    cx.on_action(|switch: &SwitchTheme, cx: &mut App| {
        if let Some(theme_cfg) = ThemeRegistry::global(cx).themes().get(&switch.0).cloned() {
            Theme::global_mut(cx).apply_config(&theme_cfg);
            cx.refresh_windows();
        }
    });
}
