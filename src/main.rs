//! Лабораторная работа №5: освещение и материалы (Rust / Bevy 0.19).
//! Главный файл подключает модули, создаёт окно, регистрирует ресурсы
//! и системы, загружает материалы и создаёт сцену.

mod camera;
mod data;
mod display;
mod graphic_object;
mod light;
mod phong_material;
mod simulation;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use camera::CameraRig;
use data::{MATERIAL_FILES, initial_camera_position, initial_light, scene_layout};
use display::update_window_title;
use graphic_object::{GraphicObject, spawn_graphic_object};
use light::{SceneLight, apply_light, spawn_scene_light};
use phong_material::PhongMaterial;
use simulation::{apply_camera_transform, simulate_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lab05".into(),
                resolution: (1000, 700).into(),
                ..default()
            }),
            ..default()
        }))
        // Встроенный плагин подсчёта FPS.
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // Чёрный фон сцены.
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        // Ресурс камеры из ЛР №4.
        .insert_resource(CameraRig::new(initial_camera_position()))
        // Ресурс источника света.
        .insert_resource(initial_light())
        // Создание сцены при старте приложения.
        .add_systems(Startup, setup_scene)
        // Системы, выполняемые каждый кадр, в строгом порядке.
        .add_systems(
            Update,
            (
                simulate_camera,
                apply_camera_transform,
                apply_light,
                update_window_title,
            )
                .chain(),
        )
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_rig: Res<CameraRig>,
    light: Res<SceneLight>,
) {
    // Один меш тора используется всеми объектами.
    let torus = meshes.add(Torus::new(0.4, 1.0));

    // Загружаем материалы из файлов. Если файл не удалось прочитать,
    // используется материал по умолчанию, а ошибка выводится в консоль.
    let material_handles: Vec<Handle<StandardMaterial>> = MATERIAL_FILES
        .iter()
        .map(|path| {
            let phong = PhongMaterial::load(path).unwrap_or_else(|error| {
                eprintln!("Ошибка загрузки материала: {error}");
                PhongMaterial::default()
            });
            println!("Загружен материал {path}: {phong:?}");
            materials.add(phong.to_standard_material())
        })
        .collect();

    // Создаём графические объекты. Handle клонируется: объект получает
    // ещё одну ссылку на уже существующий материал.
    for layout in scene_layout() {
        let material = material_handles[layout.material_index].clone();
        let object = GraphicObject::new(layout.position, layout.angle_deg, material);
        spawn_graphic_object(&mut commands, torus.clone(), object);
    }

    // Точечный источник света.
    spawn_scene_light(&mut commands, &light);

    // Камера. Её стартовый Transform берётся из ресурса CameraRig.
    commands.spawn((Camera3d::default(), camera_rig.transform()));

    println!(
        "[setup_scene] Сцена создана: {} объекта, источник света в {:?}.",
        scene_layout().len(),
        light.position()
    );
    println!(
        "[setup_scene] Начальная позиция камеры: {:?}",
        camera_rig.position()
    );
}
