use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy_mod_raycast::prelude::*;

mod board;
mod game;

use game::Game;
use board::{Player};

#[derive(Component)]
struct BoardCell;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct MyRaycastSet;

#[derive(Resource, Default)]
struct SelectedCell {
    pos: Option<(u32, u32)>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, bevy_mod_raycast::deferred::DeferredRaycastingPlugin::<MyRaycastSet>::default()))
        .insert_resource(Game::new())
        .insert_resource(SelectedCell::default())
        .add_systems(Startup, (setup_camera, spawn_board_base))
        .add_systems(Update, (sync_board_with_game, handle_input))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, RaycastSource::<MyRaycastSet>::new()));
}

fn spawn_board_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(10.0, 10.0))),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        transform: Transform::from_xyz(4.5, -0.5, 4.5),
        ..default()
    });

    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 0.1, 1.0)),
                    material: materials.add(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                    ..default()
                },
                RaycastMesh::<MyRaycastSet>::default(),
            ));
        }
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn sync_board_with_game(
    mut commands: Commands,
    game: Res<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_cells: Query<Entity, With<BoardCell>>,
) {
    for entity in board_cells.iter() {
        commands.entity(entity).despawn();
    }

    for (y, row) in game.board.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.height > 0 {
                let player_color = match cell.owner {
                    Some(Player::White) => Color::WHITE,
                    Some(Player::Black) => Color::BLACK,
                    None => Color::Srgba(Srgba::hex("808080").unwrap()),
                };

                for i in 0..cell.height {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(0.9, 0.9, 0.9)),
                            material: materials.add(player_color),
                            transform: Transform::from_xyz(x as f32, i as f32 * 0.9, y as f32),
                            ..default()
                        },
                        BoardCell,
                    ));
                }
            }
        }
    }
}

fn handle_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut game: ResMut<Game>,
    mut selected_cell: ResMut<SelectedCell>,
    mut raycast: Raycast,
    camera_query: Query<&RaycastSource<MyRaycastSet>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(ray_source) = camera_query.iter().next() {
            if let Some(ray) = ray_source.get_ray() {
                if let Some((_entity, intersection)) = raycast.cast_ray(ray, &RaycastSettings::default()).first() {
                    let hit_point = intersection.position();
                    let x = hit_point.x.round() as u32;
                    let y = hit_point.z.round() as u32;
                    let pos = (x, y);

                    if let Some(pos1) = selected_cell.pos {
                        let pos2 = pos;
                        println!("Placing block from {:?} to {:?}", pos1, pos2);
                        let current_turn = game.current_turn;
                        match game.handle_move(current_turn, pos1, pos2) {
                            Ok(_) => {
                                println!("Block placed successfully!");
                                if let Some(winner) = game.check_win_condition() {
                                    println!("Winner is {:?}", winner);
                                }
                            },
                            Err(e) => eprintln!("Error placing block: {:?}", e),
                        }
                        selected_cell.pos = None;
                    } else {
                        println!("Selected cell: {:?}", pos);
                        selected_cell.pos = Some(pos);
                    }
                }
            }
        }
    }
}