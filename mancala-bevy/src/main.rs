use bevy::math::bounding::BoundingVolume;
use bevy::{
    math::bounding::Aabb2d,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
    window::{PrimaryWindow, WindowMode, WindowResolution},
};
use mancala_logic::board::{Board, Position, NUM_POCKETS};
use mancala_logic::game::{select, Turn};
use mancala_logic::player::Player;
use std::collections::HashMap;

// 16:9 for mobile
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
        .add_systems(Update, update_label)
        .add_systems(Update, handle_game_over)
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

#[derive(Resource)]
struct Coordinates {
    buttons: HashMap<Position, Aabb2d>,
}

#[derive(Resource, Debug)]
struct BoardRes(Board);

#[derive(Resource, Debug)]
struct TurnRes(Turn);

#[derive(Component)]
struct PositionComp(Position);

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    let pocket_size = WINDOW_X / (NUM_POCKETS + 2) as f32;
    let margin = 2.0;
    let color = Color::srgb(0.5, 0.5, 1.0);

    let mut buttons = HashMap::<Position, Aabb2d>::new();

    let text_style = TextStyle {
        font_size: 0.3 * pocket_size,
        color: Color::WHITE,
        ..default()
    };

    // Stores
    for i in [-1., 1.] {
        let center = Vec2::new(i * (-WINDOW_X / 2. + pocket_size / 2.), 0.);
        let size = Vec2::new(pocket_size - margin, 2. * pocket_size - margin);
        let position = Position::Store {
            player: if i == -1.0 { Player::A } else { Player::B },
        };
        buttons.insert(position, Aabb2d::new(center, size / 2.0));

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: center.extend(0.0),
                ..default()
            },

            ..default()
        });
    }

    // Pockets
    for i in 0..NUM_POCKETS {
        for j in [-1., 1.] {
            let center_x = -WINDOW_X / 2. + ((i + 1) as f32 * pocket_size) + pocket_size / 2.;
            let center = Vec2::new(j * center_x, -j * pocket_size / 2.);
            let size = Vec2::new(pocket_size - margin, pocket_size - margin);
            let position = Position::Pocket {
                player: if j == -1. { Player::B } else { Player::A },
                idx: i,
            };
            buttons.insert(position, Aabb2d::new(center, size / 2.0));

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        custom_size: Some(size),
                        ..default()
                    },
                    transform: Transform {
                        translation: center.extend(0.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        Text2dBundle {
                            text: Text {
                                sections: vec![TextSection::new("0", text_style.clone())],
                                justify: JustifyText::Left,
                                linebreak_behavior: BreakLineOn::WordBoundary,
                            },
                            text_2d_bounds: Text2dBounds { size: size },
                            transform: Transform::from_translation(Vec3::Z),
                            ..default()
                        },
                        PositionComp(Position::Pocket {
                            player: if j == -1. { Player::B } else { Player::A },
                            idx: i,
                        }),
                    ));
                });
        }
    }

    commands.insert_resource(Coordinates { buttons });
    commands.insert_resource(BoardRes(Board::new()));
    commands.insert_resource(TurnRes(Turn::InProgress { next: Player::A }));
}

fn update_label(
    board: Res<BoardRes>,
    mut query: Query<(&mut Text, &PositionComp)>,
    turn: Res<TurnRes>,
) {
    for (mut text, position) in &mut query {
        *text = Text {
            sections: vec![TextSection::new(
                board.0[position.0].to_string(),
                TextStyle {
                    color: match (turn.0, position.0) {
                        (Turn::InProgress { next }, Position::Pocket { player, .. }) => {
                            if next == player {
                                Color::linear_rgb(0.8, 0.5, 0.2)
                            } else {
                                Color::WHITE
                            }
                        }
                        _ => Color::WHITE,
                    },
                    ..text.sections[0].style.clone()
                },
            )],
            ..*text
        }
    }
}

fn handle_game_over(mut commands: Commands, turn: Res<TurnRes>) {
    match turn.0 {
        Turn::InProgress { .. } => (),
        Turn::Finished { winner } => {
            commands.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Player {winner:?} won!"),
                        TextStyle {
                            font_size: WINDOW_X / 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )],
                    ..default()
                },
                transform: Transform::from_translation(2. * Vec3::Z),
                ..default()
            });
        }
    }
}

fn handle_mouse_clicks(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    coordinates: Res<Coordinates>,
    board: ResMut<BoardRes>,
    turn: ResMut<TurnRes>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();
    let op_world_cursor_position = window.cursor_position().and_then(|viewport_position| {
        camera.viewport_to_world_2d(camera_transform, viewport_position)
    });

    if mouse_input.just_released(MouseButton::Left) {
        let pos = op_world_cursor_position.unwrap();
        for (position, aabb2d) in &coordinates.buttons {
            if aabb2d.contains(&Aabb2d::new(pos, Vec2::new(1e-5, 1e-5))) {
                match (position, turn.0) {
                    (Position::Pocket { player, .. }, Turn::InProgress { next }) => {
                        if *player == next {
                            let (new_turn, new_board) = select(&board.0, &position);
                            commands.insert_resource(BoardRes(new_board));
                            commands.insert_resource(TurnRes(new_turn));
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
