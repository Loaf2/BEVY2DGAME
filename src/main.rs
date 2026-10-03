use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, prelude::*};
use rand::RngExt;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub static mut PLAYER_TIMES_CLICKED: f32 = 1.0;
pub static mut NUMBER_OF_BALLS_COLLECTED: i32 = 0;

fn main
()
{
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Update, player_hit_enemy)
		.add_systems(Startup, setup)
		.add_systems(Update, player_movement)
		.run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn setup
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
)
{
	commands.spawn((Camera2d, IsDefaultUiCamera));
	commands.spawn
	((
		Camera2d,
		Camera
		{
			order: 1,
			clear_color: ClearColorConfig::None,
			..default()
		},
		RenderLayers::layer(1),
	));

	commands.spawn((
		Node
		{
			width: percent(100),
			height: percent(100),
			display: Display::Flex,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..default()
		},
		BackgroundColor(Color::srgb(255.0, 0.0, 0.0)),
	));

	commands.spawn
	((
		Sprite
		{
			image: asset_server.load("sprites/CollectBall.png"),
			color: Color::srgb(170.0, 234.0, 0.0),
			custom_size: Some(Vec2::new(32., 32.)),
			..default()
		},
		Enemy {},
		Transform::from_xyz(500.0, 0.0, 0.0),
		RenderLayers::layer(1),
	));

	commands.spawn
	((
		Sprite
		{
			image: asset_server.load("sprites/BlueBall.png"),
			color: Color::srgb(230.0, 234.0, 0.0),
			custom_size: Some(Vec2::new(64., 64.)),
			..default()
		},
		Player {},
		RenderLayers::layer(1),
	));
}

fn player_movement 
(
	keys: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<&mut Transform, With<Player>>,
	time: Res<Time>,
)
{
	if let Ok(mut transform) = player_query.single_mut()
	{
		let mut direction = Vec3::ZERO;

		if keys.pressed(KeyCode::KeyA)
		{
			direction += Vec3::new(-1.0, 0.0, 0.0);
		}

		if keys.pressed(KeyCode::KeyW)
		{
			direction += Vec3::new(0.0, 1.0, 0.0);
		}

		if keys.pressed(KeyCode::KeyS)
		{
			direction += Vec3::new(0.0, -1.0, 0.0);
		}

		if keys.pressed(KeyCode::KeyD)
		{
			direction += Vec3::new(1.0, 0.0, 0.0);
		}

		if direction.length() > 0.0
		{
			direction = direction.normalize();
		}

		transform.translation += direction * PLAYER_SPEED * time.delta_secs();
	}
}

pub fn player_hit_enemy
(
	mut commands: Commands,
	mut player_query: Query<&Transform, With<Player>>,
	mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
	asset_server: Res<AssetServer>,
)
{
	if let Ok(player_transform) = player_query.single_mut()
	{
			if let Ok((enemy_entity, _enemy_transform)) = enemy_query.single_mut()
			{
				for enemy_transform in enemy_query.iter()
				{
					let distance = player_transform
						.translation
						.distance(enemy_transform.1.translation);
				let player_radius = PLAYER_SIZE / 2.0;
				let enemy_radius = ENEMY_SIZE / 2.0;
				if distance < player_radius + enemy_radius
				{
					commands.entity(enemy_entity).despawn();
					unsafe { NUMBER_OF_BALLS_COLLECTED += 1 };
					unsafe { println!("Number of balls collected: {:?}", *&raw const NUMBER_OF_BALLS_COLLECTED )};
					let mut rng = rand::rng();
					let random_location_x: i32 = rng.random_range(-500..500);
					let random_location_y: i32 = rng.random_range(-350..350);
					commands.spawn
					((
						Sprite
						{
							image: asset_server.load("sprites/CollectBall.png"),
							color: Color::srgb(170.0, 234.0, 0.0),
							custom_size: Some(Vec2::new(32., 32.)),
							..default()
						},
						Enemy {},
						Transform::from_xyz(random_location_x as f32, random_location_y as f32, 0.0),
						RenderLayers::layer(1),
					));
				}
			}
		}
	}
}
