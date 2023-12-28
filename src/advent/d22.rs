use std::cmp::Ordering;
use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use std::collections::{HashMap, HashSet};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};

use bevy::prelude::*;

#[allow(unused_imports)]
use super::util;

#[derive(Debug, PartialEq, Eq)]
struct Block {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

#[derive(Resource)]
struct Plane {
    grid: Vec<Vec<Vec<usize>>>,
}

fn stack_bricks<'a>(grid: &'a mut Vec<Vec<Vec<usize>>>, blocks: &mut Vec<Block>) {
    blocks.sort_by(|a, b| {
        let zord = a.start.2.cmp(&b.start.2);
        if zord != Ordering::Equal {
            return zord;
        }
        let yord = a.start.1.cmp(&b.start.1);
        if yord != Ordering::Equal {
            return yord;
        }
        a.start.0.cmp(&b.start.0)
    });

    for block in blocks {
        let mut delta_empty = Vec::new();
        let z = block.start.2;
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let mut delta_z = 0;
                for test_z in (0..z).rev() {
                    if grid[x][y][test_z] != 0 {
                        break;
                    }
                    delta_z = z - test_z;
                }
                delta_empty.push(delta_z);
            }
        }
        let delta;
        if let Some(&x) = delta_empty.iter().min() {
            delta = x;
        } else {
            continue;
        }
        for z in block.start.2..=block.end.2 {
            for x in block.start.0..=block.end.0 {
                for y in block.start.1..=block.end.1 {
                    grid[x][y][z - delta] = grid[x][y][z];
                    grid[x][y][z] = 0;
                }
            }
        }
        block.start.2 -= delta;
        block.end.2 -= delta;
    }
}

fn parse_block<'a>(input: &'a str) -> IResult<&'a str, Block> {
    let (rem, (start, end)) = separated_pair(
        separated_list1(tag(","), u32),
        tag("~"),
        separated_list1(tag(","), u32),
    )(input)?;
    Ok((
        rem,
        Block {
            id: rem.len() + 1,
            start: (start[0] as usize, start[1] as usize, start[2] as usize),
            end: (end[0] as usize, end[1] as usize, end[2] as usize),
        },
    ))
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Vec<Block>> {
    separated_list1(tag("\n"), parse_block)(input)
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string(&path)?.trim().parse()?;

    let (rem, mut blocks) = parse_input(&input).unwrap();
    if !rem.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Input not fully consumed",
        )));
    }

    let zmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.2 > s { b.end.2 } else { s });
    let ymax = blocks
        .iter()
        .fold(0, |s, b| if b.end.1 > s { b.end.1 } else { s });
    let xmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.0 > s { b.end.0 } else { s });

    let mut plane = Plane{ grid: vec![vec![vec![0; zmax + 2]; ymax + 1]; xmax + 1] };

    for block in &blocks {
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                for z in block.start.2..=block.end.2 {
                    plane.grid[x][y][z] = block.id;
                }
            }
        }
    }

    stack_bricks(&mut plane.grid, &mut blocks);

    // construct block deps map: id -> (amount blocks we stand on, if is empty above)
    let mut block_stands_on: HashMap<usize, usize> = HashMap::new();
    let mut block_is_empty_above: HashMap<usize, bool> = HashMap::new();

    let mut removable = 0;

    for block in &blocks {
        let mut empty_above = true;
        let mut stands_on: HashSet<usize> = HashSet::new();
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let z = block.end.2;
                if plane.grid[x][y][z+1] != 0 {
                    empty_above = false;
                }
                if z > 0 && plane.grid[x][y][z-1] != 0 {
                    stands_on.insert(plane.grid[x][y][z-1]);
                }
            }
        }
        block_is_empty_above.insert(block.id, empty_above);
        block_stands_on.insert(block.id, stands_on.len());
    }

    for block in &blocks {
        let mut safe = true;
        for x in block.start.0..=block.end.0 {
            for y in block.start.1..=block.end.1 {
                let z = block.end.2;
                let block_above = plane.grid[x][y][z+1];
                if block_above != 0 && block_stands_on[&block_above] == 1 {
                    safe = false;
                }
            }
        }
        if safe {
            removable += 1;
        }
    }

    println!("Removable blocks: {}", removable);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(plane)
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

pub fn pt2(_path: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    plane: Res<Plane>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(50.0).into()),
        material: materials.add(Color::YELLOW_GREEN.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    for x in 0..plane.grid.len() {
        for y in 0..plane.grid[0].len() {
            for z in 0..plane.grid[0][0].len() {
                if plane.grid[x][y][z] == 0 {
                    continue;
                }
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(
                        Color::rgb_u8(
                            ((plane.grid[x][y][z] * 7) % 256) as u8,
                            ((plane.grid[x][y][z] * 5) % 256) as u8,
                            ((plane.grid[x][y][z] * 3) % 256) as u8,
                        )
                        .into(),
                    ),
                    transform: Transform::from_xyz(x as f32, z as f32, y as f32),
                    ..default()
                });
            }
        }
    }
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
