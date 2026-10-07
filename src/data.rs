//! Модуль общих данных: начальные позиции камеры и источника света,
//! список файлов материалов и размещение объектов.

use bevy::prelude::*;

use crate::light::SceneLight;

/// Файлы материалов. Пути задаются относительно каталога проекта.
pub const MATERIAL_FILES: [&str; 4] = [
    "assets/materials/red_glossy.txt",
    "assets/materials/blue_matte.txt",
    "assets/materials/green_satin.txt",
    "assets/materials/white_emissive.txt",
];

/// Размещение объекта: вместо цвета — индекс материала в MATERIAL_FILES,
/// поэтому несколько объектов могут использовать один материал.
pub struct ObjectLayout {
    pub position: Vec3,
    pub angle_deg: f32,
    pub material_index: usize,
}

/// Начальная позиция камеры, как в ЛР №3 и №4.
pub fn initial_camera_position() -> Vec3 {
    Vec3::new(10.0, 15.0, 17.5)
}

/// Начальные параметры источника света: над плоскостью и сбоку от центра.
pub fn initial_light() -> SceneLight {
    let mut light = SceneLight::new(Vec3::new(8.0, 10.0, 6.0));
    light.set_color(Color::WHITE);
    light.set_intensity(6_000_000.0);
    light.set_ambient(Color::WHITE, 80.0);
    light
}

/// Размещение четырёх торов, как в ЛР №3; у каждого свой материал.
pub fn scene_layout() -> Vec<ObjectLayout> {
    vec![
        // Красный глянцевый — справа (+X).
        ObjectLayout {
            position: Vec3::new(4.0, 0.0, 0.0),
            angle_deg: 180.0,
            material_index: 0,
        },
        // Синий матовый — слева (-X).
        ObjectLayout {
            position: Vec3::new(-4.0, 0.0, 0.0),
            angle_deg: 0.0,
            material_index: 1,
        },
        // Зелёный полуматовый — сзади (-Z).
        ObjectLayout {
            position: Vec3::new(0.0, 0.0, -4.0),
            angle_deg: 90.0,
            material_index: 2,
        },
        // Белый с самосвечением — спереди (+Z).
        ObjectLayout {
            position: Vec3::new(0.0, 0.0, 4.0),
            angle_deg: 270.0,
            material_index: 3,
        },
    ]
}
