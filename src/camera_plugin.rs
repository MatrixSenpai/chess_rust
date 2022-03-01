use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera)
            .add_system(handle_cursor)
            .add_system(translate_to_hover);
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Default)]
struct MousePos(f32, f32);
#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct HoveredCell(pub u8, pub u8);
#[derive(Default)]
pub struct SelectedCell(u8, u8);

fn setup_camera(mut commands: Commands) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn handle_cursor(mut commands: Commands, windows: Res<Windows>, camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get(camera.window).unwrap();

    if let Some(screen_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_position: Vec2 = world_position.truncate();

        commands.insert_resource(MousePos(world_position.x, world_position.y));
    } else {
        commands.remove_resource::<MousePos>();
    }
}

fn translate_to_hover(mut commands: Commands, mouse_position: Option<Res<MousePos>>) {
    if let Some(mouse_position) = mouse_position {
        let normalized = normalize_cursor(Vec2::new(mouse_position.0, mouse_position.1));
        if let Some((x, y)) = normalized {
            commands.insert_resource(HoveredCell(x, y));
        }
    }
}

fn normalize_cursor(world_position: Vec2) -> Option<(u8, u8)> {
    if (-310.0_f32..310.0_f32).contains(&world_position.x) && (-310.0_f32..310.0_f32).contains(&world_position.y) {
        let x_pos = ((world_position.x + 310.0) / 77.5).round() - 1.0;
        let y_pos = ((world_position.y + 310.0) / 77.5).round() - 1.0;

        Some((x_pos as u8, y_pos as u8))
    } else {
        None
    }
}