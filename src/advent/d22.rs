use std::cmp::Ordering;
use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::{sequence::separated_pair, IResult};

use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Block {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

#[derive(Resource)]
struct Plane {
    blocks: Vec<Block>,
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
            id: rem.len(),
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

    // Create a top-side view
    let zmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.2 > s { b.end.2 } else { s });
    let ymax = blocks
        .iter()
        .fold(0, |s, b| if b.end.1 > s { b.end.1 } else { s });
    let xmax = blocks
        .iter()
        .fold(0, |s, b| if b.end.0 > s { b.end.0 } else { s });

    let mut grid = vec![vec![vec![0, xmax]; ymax]; zmax];

    for (idx, block) in blocks.iter().enumerate() {
        for x in block.start.0..block.end.0 {
            for y in block.start.1..block.end.1 {
                for z in block.start.2..block.end.2 {
                    grid[x][y][z] = idx + 1;
                }
            }
        }
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(Plane { blocks })
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
        mesh: meshes.add(shape::Circle::new(1000.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    for block in &plane.blocks {
        println!("{:?}", block);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                (block.end.0 - block.start.0 + 1) as f32,
                (block.end.2 - block.start.2 + 1) as f32,
                (block.end.1 - block.start.1 + 1) as f32,
            ))),
            material: materials.add(
                Color::rgb_u8(
                    ((block.id * 7) % 256) as u8,
                    ((block.id * 5) % 256) as u8,
                    ((block.id * 3) % 256) as u8,
                )
                .into(),
            ),
            transform: Transform::from_xyz(
                block.start.0 as f32,
                block.start.2 as f32,
                block.start.1 as f32,
            ),
            ..default()
        });
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
