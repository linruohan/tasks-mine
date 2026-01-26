mod ui;

use gpui::*;
use gpui_component::{Root, TitleBar};
use ui::app::App;

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

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
                |window, _cx| {
                    let app_entity = _cx.new(|_| App::new());
                    _cx.new(|cx| Root::new(app_entity, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
