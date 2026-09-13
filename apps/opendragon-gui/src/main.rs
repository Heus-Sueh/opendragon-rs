use gpui::prelude::*;
use gpui::*;
use gpui_kit::component::Root;

pub mod app;
pub mod components;
pub mod services;
pub mod state;
pub mod theme;
pub mod views;

use app::OpenDragonApp;
use services::devices::create_backend;
use state::AppState;

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting OpenDragon-rs GUI");

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);

            let backend = create_backend();
            let state = cx.new(|_cx| AppState::new(backend));

            // Set global theme from initial state
            let theme = state.read(cx).theme;
            cx.set_global(theme);

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::default(),
                    size: size(px(1080.0), px(720.0)),
                })),
                titlebar: Some(TitlebarOptions {
                    title: Some("OpenDragon-rs".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| OpenDragonApp::new(state, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        });
}
