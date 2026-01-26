mod ui;

use gpui::Application;
use gpui_component_assets::Assets;
use tasks_mine::app::TasksApp;

fn main() {
    let app = Application::new().with_assets(Assets);

    // Parse `cargo run -- <story_name>`
    let name = std::env::args().nth(1);

    app.run(move |cx| {
        tasks_mine::init(cx);
        cx.activate(true);

        tasks_mine::create_new_window(
            "GPUI Component",
            move |window, cx| TasksApp::view(name.as_deref(), window, cx),
            cx,
        );
    });
}
