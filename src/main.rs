mod camera;

use crate::camera::CameraPlugin;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::CylinderMeshBuilder;

fn main() {
    App::new()
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins) // Adds default plugins like rendering and window management
        .add_systems(Startup, setup) // System to set up the scene
        .add_systems(Update, (rotate_object, jump)) // System to rotate the object
        .run();
}

// This system sets up the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a camera
    commands.spawn((
        PointLight {
            intensity: 0.0,
            range: 300.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(5.0, 5.0, 0.0),
    ));

    commands.spawn((
        Name::new("Cylinder"),
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d::from(meshes.add(CylinderMeshBuilder::new(4.0, 0.1, 32).build())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            metallic: 0.1,
            perceptual_roughness: 0.1,
            ..Default::default()
        })),
        Transform::from_xyz(2.0, 0.0, 0.0),
    ));

    commands.spawn((
        Name::new("Cuboid"),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        LinearVelocity::default(),
        Mesh3d::from(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb_u8(124, 144, 255)))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
}

type WithMesh = (With<Mesh3d>, With<MeshMaterial3d<StandardMaterial>>);

// This system rotates the 3D object
fn rotate_object(time: Res<Time>, mut query: Query<(&Name, &mut Transform), WithMesh>) {
    for (name, mut transform) in query.iter_mut() {
        if name.as_str() == "Cylinder" {
            transform.rotation *= Quat::from_rotation_y(1.0 * time.delta().as_secs_f32());
        }
    }
}

fn jump(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Name, &mut LinearVelocity), WithMesh>,
) {
    if input.pressed(KeyCode::Space) {
        for (name, mut velocity) in query.iter_mut() {
            if name.as_str() == "Cuboid" {
                velocity.y += 10.0 * time.delta_secs();
            }
        }
    }
}
