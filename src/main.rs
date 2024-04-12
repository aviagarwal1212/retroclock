use bevy::prelude::*;
use chrono::Timelike;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, clock_face)
        .run();
}

fn setup(mut commands: Commands, mut gizmo_conf: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2dBundle::default());
    let (config, _) = gizmo_conf.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 20.0;
}

fn clock_face(mut gizmos: Gizmos) {
    let now = chrono::Local::now();

    let hour = now.hour() as f32;
    let minute = now.minute() as f32;
    let second = now.second() as f32;

    let minute_angle = ((360.0 / 60.0) * minute).to_radians();
    let second_angle = ((360.0 / 60.0) * second).to_radians();
    let hour_angle = ((360.0 / 48.0) * hour).to_radians();

    gizmos
        .arc_2d(
            Vec2::ZERO,
            second_angle / 2.0,
            second_angle,
            100.0,
            Color::BISQUE,
        )
        .segments(360 * 3);
    gizmos
        .arc_2d(
            Vec2::ZERO,
            minute_angle / 2.0,
            minute_angle,
            120.0,
            Color::TEAL,
        )
        .segments(360 * 3);
    gizmos
        .arc_2d(
            Vec2::ZERO,
            hour_angle / 2.0,
            hour_angle,
            140.0,
            Color::ORANGE,
        )
        .segments(360 * 3);
}
