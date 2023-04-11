use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{GameState, Player, VELOCITY_LIMIT, DRAG_COEFICITENT, SCREEN_WIDTH, SCREEN_HEIGHT, LINES_Z, FRICTION_COEFICIENT, WIND, GRAVITY, SPRITE_H_HALF, SPRITE_W_HALF};

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Acceleration(Vec2);

#[derive(Component)]
struct Mass(f32);


pub struct ForceComponentsPlugin;

impl Plugin for ForceComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (
                    apply_gravity.before(motion).before(edges_bounce),
                    apply_wind.before(motion).before(edges_bounce),
                    apply_friction.before(motion).before(edges_bounce),
                    apply_drag.before(motion).before(edges_bounce),
                    edges_bounce.before(motion),
                    motion,
                ).in_set(OnUpdate(GameState::Playing)));
    }
}

fn motion(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &mut Acceleration, &mut Transform), With<Player>>
) {
    let dt = time.delta_seconds();
    for (mut velocity, mut acceleration, mut transform) in q.iter_mut() {
        velocity.0 = (velocity.0 + acceleration.0).clamp_length_max(VELOCITY_LIMIT)  * dt;
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
        acceleration.0.x = 0.0;
        acceleration.0.y = 0.0;
    }
}

fn apply_gravity(
    mut q: Query<&mut Acceleration>
) {
    for mut acceleration in q.iter_mut() {
        acceleration.0 += GRAVITY;
    }
}

fn apply_wind(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<(&mut Acceleration, &Mass)>
) {
    if keyboard_input.pressed(KeyCode::W) {
        for (mut acceleration, mass) in q.iter_mut() {
            acceleration.0 += WIND / mass.0;
        }
    }
}

fn apply_friction(
    mut q: Query<(&Transform, &mut Acceleration, &Velocity)>
) {
    for (transform, mut acceleration, velocity) in q.iter_mut() {
        if transform.translation.y - SPRITE_H_HALF + SCREEN_HEIGHT / 2.0 < 1.0 {
            acceleration.0 += velocity.0.normalize() * -1.0 * FRICTION_COEFICIENT;
        }
    }
}

fn apply_drag(
    mut q: Query<(&mut Acceleration, &Velocity, &Mass)>
) {
    for (mut acceleration, velocity, mass) in q.iter_mut() {
        let drag = velocity.0.normalize_or_zero() * -1.0 * DRAG_COEFICITENT * velocity.0.length_squared();
        acceleration.0 += drag / mass.0;
    }
}


fn edges_bounce(
    mut q: Query<(&mut Velocity, &mut Transform)>
) {
    for (mut velocity, mut transform) in q.iter_mut() {
        if transform.translation.y - SPRITE_H_HALF <= -SCREEN_HEIGHT / 2.0 {
            transform.translation.y = -SCREEN_HEIGHT / 2.0 + SPRITE_H_HALF;
            velocity.0.y *= -1.0;
        }
        if transform.translation.x + SPRITE_W_HALF >= SCREEN_WIDTH / 2.0 {
            transform.translation.x = SCREEN_WIDTH / 2.0 - SPRITE_W_HALF;
            velocity.0.x *= -1.0;
        }
        if transform.translation.x - SPRITE_W_HALF <= -SCREEN_WIDTH / 2.0 {
            transform.translation.y = -SCREEN_WIDTH / 2.0 + SPRITE_W_HALF;
            velocity.0.x *= -1.0;
        }
    }
}


fn debug_line(
    mut q: Query<(&Velocity, &Acceleration, &Transform), With<Player>>,
    mut lines: ResMut<DebugLines>,
) {
    let Ok((velocity, acceleration, transform)) = q.get_single_mut() else {
        return;
    };

    let start = transform.translation.truncate().extend(LINES_Z);
    let end = start + velocity.0.extend(LINES_Z);
    lines.line_colored(start, end, 0.0, Color::WHITE);
    let end = start + acceleration.0.extend(LINES_Z) * 10.0;
    lines.line_colored(start, end, 0.0, Color::YELLOW);
}
