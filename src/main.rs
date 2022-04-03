use bevy::app::App;
use bevy::core::FixedTimestep;
use bevy::DefaultPlugins;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_event::<EatEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_input_system)
                .with_system(eat_system))
        .add_system(score_system)
        .add_system(respawn_food_system)
        .run()
}

#[derive(Default)]
struct Game {
    points: u32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Speed(f32);

struct EatEvent(Entity);


fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Speed, &mut Transform)>,
) {
    let (_, speed, mut transform) = query.single_mut();
    let mut y_dir: f32 = 0.0;
    let mut x_dir: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::A) { x_dir -= 1.0; }
    if keyboard_input.pressed(KeyCode::D) { x_dir += 1.0; }
    if keyboard_input.pressed(KeyCode::S) { y_dir -= 1.0; }
    if keyboard_input.pressed(KeyCode::W) { y_dir += 1.0; }

    let translation: &mut Vec3 = &mut transform.translation;
    translation.x += x_dir * speed.0 * TIME_STEP;
    translation.x = translation.x.min(400.0).max(-400.0);

    translation.y += y_dir * speed.0 * TIME_STEP;
    translation.y = translation.y.min(300.0).max(-300.0);
}

fn eat_system(
    mut eat_event_writer: EventWriter<EatEvent>,
    food_query: Query<(Entity, &mut Food, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
) {
    let result = food_query.get_single();
    if let Ok((food_entity, _, food_transform)) = result {
        let (_, player_transform): (_, &Transform) = player_query.single();

        if is_collision(&food_transform, &player_transform) {
            eat_event_writer.send(EatEvent(food_entity))
        }
    }
}

fn respawn_food_system(mut eat_event: EventReader<EatEvent>, mut commands: Commands) {
    for event in eat_event.iter() {
        let food_entity: Entity = event.0;
        commands.entity(food_entity).despawn_recursive();
        spawn_food(&mut commands);
    }
}

fn score_system(
    mut eat_event_reader: EventReader<EatEvent>,
    mut game: ResMut<Game>,
) {
    for _ in eat_event_reader.iter() {
        game.points += 1;
        println!("{}", game.points);
    }
}

fn is_collision(transform_a: &Transform, transform_b: &Transform) -> bool {
    if (transform_a.translation.x - transform_b.translation.x).abs() < 10.0
        && (transform_a.translation.y - transform_b.translation.y).abs() < 10.0 {
        true
    } else { false }
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    game.points = 0;
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(500.0));

    spawn_food(&mut commands);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


fn spawn_food(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-400.0..400.0);
    let y: f32 = rng.gen_range(-300.0..300.0);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::new(10.0, 10.0, 0.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Food);
}