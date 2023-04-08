use bevy::{math::vec2, prelude::*, window::WindowResolution};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_prototype_debug_lines::DebugLinesPlugin;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPRITE_SIZE: f32 = 16.0;

pub fn lerp(start: f32, end: f32, ratio: f32) -> f32 {
    start * (1.0 - ratio) + end * ratio
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_state::<GameState>()
    .add_startup_system(setup)
    .add_system(setup_world.in_schedule(OnEnter(GameState::Playing)))
    .add_systems((
        input,
        mouse_input
    ).in_set(OnUpdate(GameState::Playing)));

    #[cfg(debug_assertions)]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(DebugLinesPlugin::default());
    }

    app.run();
}

#[derive(Resource)]
pub struct GameResources {
    image_handle: Handle<Image>,
    font_handle: Handle<Font>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("sprites.png");
    let font_handle = asset_server.load("QuinqueFive.ttf");

    commands.insert_resource(GameResources {
        image_handle,
        font_handle,
    });
    commands.spawn(Camera2dBundle::default());
}

fn setup_world(mut commands: Commands, game_resources: Res<GameResources>) {
    let player_entity = commands
        .spawn((
            Group(0),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(
                        0.0 * SPRITE_SIZE,
                        0.,
                        1.0 * SPRITE_SIZE,
                        SPRITE_SIZE,
                    )),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(-200., 0., CHARACTER_Z_INDEX),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        Group(1),
        SteerAi::new(0.2),
        AvoidObstacles(SteerConfiguration::new(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE * 3.0)),
        ChaseTargets(SteerConfiguration::new(SPRITE_DRAW_SIZE * 4.0, 400.0), vec![player_entity]),
        StrafeAround(SteerConfiguration::new(SPRITE_DRAW_SIZE * 2.0, SPRITE_DRAW_SIZE * 4.5), player_entity, vec2(0.1, 0.98)),
        AvoidGroup(SteerConfiguration::new(0.0, SPRITE_DRAW_SIZE * 2.5), 0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                color: Color::Blue,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., CHARACTER_Z_INDEX),
            ..default()
        },
    ));
}


fn mouse_input(
    mut commands: Commands,
    game_resources: Res<GameResources>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };
    let Some(mouse_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) else
    {
        return;
    };

    if mouse_button_input.just_pressed(MouseButton::Right) {

    }
}

fn input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Transform, With<PlayerControlled>>,
) {
    let Ok(mut transform) = player_q.get_single_mut() else {
        return;
    };

    let mut velocity = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {

    }
    if keyboard_input.pressed(KeyCode::S) {

    }
    if keyboard_input.pressed(KeyCode::A) {

    }
    if keyboard_input.pressed(KeyCode::D) {

    }

    if velocity != Vec2::ZERO {

    }
}
