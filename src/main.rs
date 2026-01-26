mod ui;

use gpui::*;
use gpui_component::{Root, TitleBar};
use ui::{theme, App};

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point { x: px(100.0), y: px(100.0) },
                        size: Size { width: px(1400.0), height: px(900.0) },
                    })),
                    titlebar: Some(TitleBar::title_bar_options()),
                    window_min_size: Some(Size { width: px(800.0), height: px(600.0) }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|_| App::new());
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
