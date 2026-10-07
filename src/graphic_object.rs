//! Графический объект: позиция, угол поворота и ссылка на материал;
//! создание сущности на сцене.

use bevy::prelude::*;

/// Графический объект. Позиция и угол принадлежат объекту (композиция),
/// а материал хранится по ссылке Handle и может быть общим (агрегация).
#[derive(Component, Debug, Clone)]
pub struct GraphicObject {
    /// Позиция объекта в глобальной системе координат.
    pub position: Vec3,

    /// Угол поворота вокруг оси Oy, в градусах.
    pub angle_deg: f32,

    /// Разделяемая ссылка на материал объекта.
    pub material: Handle<StandardMaterial>,
}

impl GraphicObject {
    /// Конструктор.
    pub fn new(position: Vec3, angle_deg: f32, material: Handle<StandardMaterial>) -> Self {
        Self {
            position,
            angle_deg,
            material,
        }
    }

    /// Матрица модели: перенос в позицию и поворот вокруг Oy по часовой стрелке.
    pub fn to_transform(&self) -> Transform {
        let angle_rad = -self.angle_deg.to_radians();
        Transform::from_translation(self.position).with_rotation(Quat::from_rotation_y(angle_rad))
    }
}

/// Создание сущности: объект получает ссылку на общий меш и передаёт движку
/// ссылки на меш и материал. Расчёт освещения выполняет шейдер материала.
pub fn spawn_graphic_object(commands: &mut Commands, mesh: Handle<Mesh>, object: GraphicObject) {
    let transform = object.to_transform();
    let material = object.material.clone();

    println!(
        "Создан объект: позиция {:?}, угол {}°",
        object.position, object.angle_deg
    );

    commands.spawn((object, Mesh3d(mesh), MeshMaterial3d(material), transform));
}
