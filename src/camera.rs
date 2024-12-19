use bevy::input::keyboard::KeyboardInput;
use bevy::pbr::ShadowFilteringMethod;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyE, KeyQ, KeyS, KeyW};
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct CameraControlData {
    pub speed: f32,
    pub sensitivity: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraControlData {
            speed: 10.0,
            sensitivity: 1.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ShadowFilteringMethod::Hardware2x2,
    ));
}

fn update_camera(
    mut event_reader: EventReader<KeyboardInput>,
    data: Res<CameraControlData>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut x_spd = 0.0;
    let mut y_spd = 0.0;
    let mut z_spd = 0.0;
    for event in event_reader.read() {
        info!("Key Event: {:?}", event);

        if event.state.is_pressed() {
            if event.key_code == KeyA {
                x_spd -= 1.0;
            } else if event.key_code == KeyD {
                x_spd += 1.0;
            }

            if event.key_code == KeyW {
                z_spd -= 1.0;
            } else if event.key_code == KeyS {
                z_spd += 1.0;
            }

            if event.key_code == KeyQ {
                y_spd -= 1.0;
            } else if event.key_code == KeyE {
                y_spd += 1.0;
            }
        }
    }

    for mut transform in query.iter_mut() {
        let dir = Vec3::new(x_spd, y_spd, z_spd).normalize_or_zero();
        let right = transform.right();
        let up = transform.up();
        let forward = transform.forward();
        let delta = time.delta_secs();
        transform.translation +=
            (dir.x * right + dir.y * up - dir.z * forward) * data.speed * delta;
    }
}
