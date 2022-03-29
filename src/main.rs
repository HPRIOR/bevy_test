use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::DefaultPlugins;
use bevy::ecs::component::Component;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_input)
        .run()
}

#[derive(Default)]
struct Game {
    points: u32,
    player: Option<Entity>,
}

#[derive(Component)]
struct Player;

fn player_input(game: ResMut<Game>, keyboard_input: Res<Input<KeyCode>>, mut transforms: Query<&Transform>) {
    if keyboard_input.just_pressed(KeyCode::W){
        println!("pressed W");
        if let Some(player) = game.player{
            let mut player_transform  = *transforms.get_mut(player).unwrap();
            player_transform = Transform{
                translation: Vec3::new(
                    10.0,
                    1.9,
                    2.9
                ),
                ..Default::default()
            };
            let player_transform = transforms.get(player).unwrap();
            println!("{:?}", player_transform)

        }
    }
    if keyboard_input.just_pressed(KeyCode::A){

    }
    if keyboard_input.just_pressed(KeyCode::S){

    }
    if keyboard_input.just_pressed(KeyCode::D){

    }
}

fn setup(mut commands: Commands, mut game: ResMut<Game>, asset_server: Res<AssetServer>) {
    game.points = 0;
    game.player = Some(commands
        .spawn()
        .insert(Player)
        .insert(Transform::default()).with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        });
    })
        .id());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}