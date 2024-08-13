mod cliio;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};
use mancala::board::NUM_POCKETS;
/*
use mancala::board::{Board, Position, NUM_POCKETS};
use mancala::game::{select, Turn};
use mancala::player::Player;
use std::io;
 */

// TODO: add menu plugin and result plugin

// 16:9
const WINDOW_X: f32 = 512.0;
const WINDOW_Y: f32 = 288.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_X, WINDOW_Y),
                resizable: false,
                mode: WindowMode::Windowed,
                // TODO: Restore this for mobile devices
                // mode: WindowMode::BorderlessFullscreen,
                // on iOS, gestures must be enabled.
                // This doesn't work on Android
                recognize_rotation_gesture: true,
                ..default()
            }),
            ..default()
        }))
        /*
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .insert_resource(Score(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        */
        .add_systems(Startup, setup)
        .add_systems(Update, handle_mouse_clicks)
        /*
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
                play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, update_scoreboard)
        */
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    let pocket_size = WINDOW_X / (NUM_POCKETS + 2) as f32;
    let margin = 2.0;
    let color = Color::srgb(0.5, 0.5, 1.0);

    // Stores
    for i in [-1., 1.] {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: color,
                ..default()
            },
            transform: Transform {
                translation: Vec2::new(i * (-WINDOW_X / 2. + pocket_size / 2.), 0.).extend(0.0),
                scale: Vec3::new(pocket_size - margin, 2. * pocket_size - margin, 0.0),
                ..default()
            },
            ..default()
        });
    }

    // pockets
    for i in 0..NUM_POCKETS {
        for j in [-1., 1.] {
            let center_x = -WINDOW_X / 2. + ((i + 1) as f32 * pocket_size) + pocket_size / 2.;
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: color,
                    ..default()
                },
                transform: Transform {
                    translation: Vec2::new(center_x, j * pocket_size / 2.).extend(0.),
                    scale: Vec3::new(pocket_size - margin, pocket_size - margin, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();
    let op_world_cursor_position = window.cursor_position().and_then(|viewport_position| {
        camera.viewport_to_world_2d(camera_transform, viewport_position)
    });

    if mouse_input.just_released(MouseButton::Left) {
        println!("click at {:?}", op_world_cursor_position.unwrap());
    }
}
/*
fn get_input() -> usize {
    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");
    let cmd_int: usize = cmd.trim().parse::<usize>().expect("Please type a number!") - 1;
    if !(0..NUM_POCKETS).contains(&cmd_int) {
        // TODO: Retry
        println!("Please type a number between 1 and {NUM_POCKETS}");
        panic!()
    }
    cmd_int
}

fn main() {
    let mut board = Board::new();
    let mut turn = Turn::InProgress { next: Player::A };

    loop {
        cliio::print(&board);
        match turn {
            Turn::InProgress { next } => {
                let cmd = get_input();
                (turn, board) = select(
                    &board,
                    &Position::Pocket {
                        player: next,
                        idx: cmd,
                    },
                );
            }
            Turn::Finished { winner } => {
                let player_name = match winner {
                    Player::A => "First player",
                    Player::B => "Second player",
                };
                println!("{player_name} won!");
                break;
            }
        }
    }
}
 */
