//! Источник света сцены. Источник один, поэтому его параметры хранятся
//! в ресурсе SceneLight, а система apply_light переносит их в сущность
//! источника и в ресурс фонового освещения.

use bevy::prelude::*;

/// Параметры источника света сцены.
#[derive(Resource)]
pub struct SceneLight {
    /// Позиция источника в глобальной системе координат.
    position: Vec3,

    /// Цвет диффузной и зеркальной составляющих.
    color: Color,

    /// Световой поток, люмены.
    intensity: f32,

    /// Радиус действия источника.
    range: f32,

    /// Цвет фоновой составляющей.
    ambient_color: Color,

    /// Яркость фоновой составляющей, кд/м².
    ambient_brightness: f32,
}

/// Маркер сущности источника, которой управляет ресурс SceneLight.
#[derive(Component)]
pub struct SceneLightMarker;

impl SceneLight {
    /// Конструктор: белый источник в указанной позиции.
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            color: Color::WHITE,
            intensity: 6_000_000.0,
            range: 50.0,
            ambient_color: Color::WHITE,
            ambient_brightness: 80.0,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn set_ambient(&mut self, color: Color, brightness: f32) {
        self.ambient_color = color;
        self.ambient_brightness = brightness;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Компонент точечного источника; остальные поля — по умолчанию.
    pub fn point_light(&self) -> PointLight {
        PointLight {
            color: self.color,
            intensity: self.intensity,
            range: self.range,
            ..default()
        }
    }

    /// Позиция источника для его сущности.
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position)
    }

    /// Ресурс фонового освещения сцены.
    pub fn ambient_light(&self) -> GlobalAmbientLight {
        GlobalAmbientLight {
            color: self.ambient_color,
            brightness: self.ambient_brightness,
            ..default()
        }
    }
}

/// Создаёт сущность точечного источника при запуске.
pub fn spawn_scene_light(commands: &mut Commands, light: &SceneLight) {
    commands.spawn((light.point_light(), light.transform(), SceneLightMarker));
}

/// Переносит параметры ресурса в сущность источника и в фоновое освещение.
/// is_changed() пропускает работу, если ресурс не менялся.
pub fn apply_light(
    light: Res<SceneLight>,
    mut query: Query<(&mut PointLight, &mut Transform), With<SceneLightMarker>>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    if !light.is_changed() {
        return;
    }

    for (mut point_light, mut transform) in &mut query {
        *point_light = light.point_light();
        *transform = light.transform();
    }

    *ambient = light.ambient_light();
}
