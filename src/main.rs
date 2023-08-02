use bevy::{
    prelude::*,
    sprite::{Material2dPlugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    window::{PresentMode, WindowTheme},
    app::PluginGroupBuilder,
    utils::Duration,
    asset::ChangeWatcher,
};

use dmxvideo::{setup_config};
use dmxvideo::{setup_network};
use dmxvideo::{setup_shader, update_shader, DmxMaterial};


pub fn window_plugin() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "DMX Video".into(),
            resolution: (500., 300.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }).set(AssetPlugin {
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
        ..Default::default()
    })
}

fn main() {
    App::new()
        .add_plugins((
            window_plugin(),
            Material2dPlugin::<DmxMaterial>::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup_config)
        .add_systems(Startup, setup_shader)
        .add_systems(Startup, setup_network)
        .add_systems(Update, update_shader)
        .run();
}
