use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<RgbCubeMaterial>::default())
        .add_startup_system(setup)
        .add_system(camera_controls)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RgbCubeMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.5, 0.5, 0.5),
        // transform: Transform::from_xyz(1., 1., 1.),
        material: materials.add(RgbCubeMaterial {}),
        ..default()
    });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>
) {
    let mut camera = camera_query.single_mut();
    let rotateAngle = 5.0 * time.delta_seconds();

    if keyboard.pressed(KeyCode::W) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-rotateAngle))
    }
    if keyboard.pressed(KeyCode::S) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_x(rotateAngle))
    }
    if keyboard.pressed(KeyCode::A) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(rotateAngle))
    }
    if keyboard.pressed(KeyCode::D) {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-rotateAngle))
    }
}


// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "383f4b5f-dd43-419a-b0cb-bc1a869f3375"]
pub struct RgbCubeMaterial { }

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for RgbCubeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rgb_cube_material.wgsl".into()
    }
}
