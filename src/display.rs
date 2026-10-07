//! Вывод FPS в заголовок окна. Кадры считает встроенный плагин Bevy
//! FrameTimeDiagnosticsPlugin, система только читает его значение.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Обновляет заголовок окна, только если округлённое значение FPS изменилось;
/// прошлое значение хранится в Local.
pub fn update_window_title(
    diagnostics: Res<DiagnosticsStore>,
    mut windows: Query<&mut Window>,
    mut last_fps: Local<u32>,
) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    else {
        return;
    };
    let fps = fps.round().max(0.0) as u32;

    if fps != *last_fps {
        *last_fps = fps;

        for mut window in &mut windows {
            window.title = format!("Lab05 [{fps} FPS]");
        }
    }
}
