mod cliio;
use bevy::{
    prelude::*,
    window::{WindowMode, WindowResolution},
};
/*
use mancala::board::{Board, Position, NUM_POCKETS};
use mancala::game::{select, Turn};
use mancala::player::Player;
use std::io;
 */

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(512.0, 288.0), // 16:9
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
        .add_systems(Startup, setup)
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
