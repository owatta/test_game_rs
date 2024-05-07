use bevy::{prelude::*, input::keyboard::KeyboardInput};

fn main() {
    App::new()
	.add_plugins(DefaultPlugins)
	.add_systems(Startup, setup)
	.add_systems(Update, sprite_movement)
	.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
	commands.spawn(
 SpriteBundle {
			texture: asset_server.load("nullboy.png"),
			transform: Transform::from_xyz(0., 0., 0.),
			..default()
		});
}

fn sprite_movement (
	mut key_events: EventReader<KeyboardInput>,
	time: Res<Time>, 
	mut sprite_pos: Query<&mut Transform>
) {
	use bevy::input::ButtonState;
	
	let mut sprite_pos = sprite_pos.get_single_mut().expect("Error: Could not get sprite position :3");

	for event in key_events.read() {
		if let ButtonState::Pressed = event.state {
			match event.key_code {
				KeyCode::KeyW => sprite_pos.translation.y += 150. * time.delta_seconds(),
				KeyCode::KeyA => sprite_pos.translation.x -= 150. * time.delta_seconds(),
				KeyCode::KeyS => sprite_pos.translation.x += 150. * time.delta_seconds(),
				KeyCode::KeyD => sprite_pos.translation.y -= 150. * time.delta_seconds(),
				_ => (),
			}
		}
	}
}
