use bevy::{math::vec2, prelude::*, window::WindowResolution, transform::commands, sprite::MaterialMesh2dBundle};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugLines};

mod forces_components;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPRITE_W_HALF: f32 = 16.0;
const SPRITE_H_HALF: f32 = 32.0;
const LINES_Z: f32 = 2.0;

pub fn lerp(start: f32, end: f32, ratio: f32) -> f32 {
    start * (1.0 - ratio) + end * ratio
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    Restart,
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
        mouse_input,
        gravity.before(motion_movable),
        wind.before(motion_movable),
        friction.before(motion_movable),
        drag.before(motion_movable),
        motion_movable,
        edges_bounce_movable.before(motion_movable),
        reset_acceleration
    ).in_set(OnUpdate(GameState::Playing)))
    .add_system(despawn::<Player>.in_schedule(OnExit(GameState::Playing)))
    .add_system(start_playing.in_schedule(OnEnter(GameState::Restart)));

    #[cfg(debug_assertions)]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(DebugLinesPlugin::default());
        app.add_system(debug_line_movable.before(reset_acceleration).in_set(OnUpdate(GameState::Playing)));
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

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_resources: Res<GameResources>
) {
    commands
        .spawn((
            Player,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::new(SPRITE_W_HALF * 2.0, SPRITE_H_HALF * 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 1.0),
                ..default()
            },
            // Velocity(vec2(0.0, 0.0)),
            // Acceleration(vec2(0.0, 0.0)),
            // Mass(1.0),
            Movable::new(2.0),
        ));
}

fn despawn<T: Component>(
    mut commands: Commands,
    components_q: Query<Entity, With<T>>
) {
    for entity in components_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn start_playing(
    mut app_state: ResMut<NextState<GameState>>,
) {
    app_state.set(GameState::Playing);
}

const FRICTION_COEFICIENT: f32 = 1.0;
const DRAG_COEFICITENT: f32 = 0.1;
const ACCELERATION_MAGNITUE: f32 = 10.0;
const VELOCITY_LIMIT: f32 = 300.0;
const GRAVITY: Vec2 = vec2(0.0, -10.0);
const WIND: Vec2 = vec2(5.0, 0.0);

fn mouse_input(
    mut commands: Commands,
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
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<Entity, With<Player>>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let Ok(mut transform) = player_q.get_single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::R) {
        app_state.set(GameState::Restart);
    }

    let mut velocity = Vec2::ZERO;
    // if keyboard_input.pressed(KeyCode::W) {

    // }
    // if keyboard_input.pressed(KeyCode::S) {

    // }
    // if keyboard_input.pressed(KeyCode::A) {

    // }
    // if keyboard_input.pressed(KeyCode::D) {

    // }

    if velocity != Vec2::ZERO {

    }
}

#[derive(Component)]
struct Movable {
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Movable {
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }
}

#[derive(Component)]
struct Player;

fn gravity(
    mut q: Query<&mut Movable>
) {
    for mut movable in q.iter_mut() {
        let gravity = GRAVITY * movable.mass;
        movable.apply_force(gravity);
    }
}

fn wind(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<&mut Movable>
) {
    for mut movable in q.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            movable.apply_force(WIND);
        }
    }
}

fn friction(
    mut q: Query<(&mut Movable, &Transform)>
) {
    for (mut movable, transform) in q.iter_mut() {
        if transform.translation.y - SPRITE_H_HALF + SCREEN_HEIGHT / 2.0 < 1.0 {
            let friction = movable.velocity.normalize_or_zero() * -1.0 * FRICTION_COEFICIENT * movable.mass;
            movable.apply_force(friction);
        }
    }
}

fn drag(
    mut q: Query<&mut Movable>
) {
    for mut movable in q.iter_mut() {
        let drag = movable.velocity.normalize_or_zero() * -1.0 * DRAG_COEFICITENT * movable.velocity.length_squared();
        movable.apply_force(drag);
    }
}

fn reset_acceleration(
    mut q: Query<&mut Movable>
) {
    for mut movable in q.iter_mut() {
        movable.acceleration.x = 0.0;
        movable.acceleration.y = 0.0;

    }
}

fn motion_movable(
    time: Res<Time>,
    mut q: Query<(&mut Movable, &mut Transform)>
) {
    let dt = time.delta_seconds();
    for (mut movable, mut transform) in q.iter_mut() {
        movable.velocity = movable.velocity + movable.acceleration * dt;
        transform.translation.x += movable.velocity.x;
        transform.translation.y += movable.velocity.y;
    }
}


fn debug_line_movable(
    mut q: Query<(&Movable, &Transform), With<Player>>,
    mut lines: ResMut<DebugLines>,
) {
    let Ok((movable, transform)) = q.get_single_mut() else {
        return;
    };

    let start = transform.translation.truncate().extend(LINES_Z) * 100.0;
    let end = start + movable.velocity.extend(LINES_Z);
    lines.line_colored(start, end, 0.0, Color::WHITE);
}


fn edges_bounce_movable(
    mut q: Query<(&mut Movable, &mut Transform)>
) {
    for (mut movable, mut transform) in q.iter_mut() {
        if transform.translation.y - SPRITE_H_HALF <= -SCREEN_HEIGHT / 2.0 {
            transform.translation.y = -SCREEN_HEIGHT / 2.0 + SPRITE_H_HALF;
            movable.velocity.y *= -1.0;
        }
        if transform.translation.x + SPRITE_W_HALF >= SCREEN_WIDTH / 2.0 {
            transform.translation.x = SCREEN_WIDTH / 2.0 - SPRITE_W_HALF;
            movable.velocity.x *= -1.0;
        }
        if transform.translation.x - SPRITE_W_HALF <= -SCREEN_WIDTH / 2.0 {
            transform.translation.y = -SCREEN_WIDTH / 2.0 + SPRITE_W_HALF;
            movable.velocity.x *= -1.0;
        }
    }
}