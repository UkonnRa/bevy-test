use bevy::prelude::*;

#[derive(Component)]
#[require(Button, BoxShadow(default_shadow))]
pub struct ElevatedButton;

fn default_shadow() -> BoxShadow {
  BoxShadow {
    x_offset: Val::Percent(5.),
    y_offset: Val::Percent(5.),
    blur_radius: Val::Percent(2.5),
    ..Default::default()
  }
}