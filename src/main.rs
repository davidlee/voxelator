// use std::f32::consts::PI;

use bevy::prelude::*;

mod uv_texture;

#[derive(Component, Debug)]
struct CameraMarker;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component, Debug)]
struct Shape {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_systems(Startup, setup_camera)
        .add_systems(Startup, add_shapes)
        .add_systems(Update, rotate)
        .add_systems(Update, flicker)
        .run();
}

const Z_SIZE: i32 = 3;
const Y_SIZE: i32 = 10;
const X_SIZE: i32 = 10;

type Matrix3d = Vec<Vec<Vec<i32>>>;
// make a simple 9x9 voxel cube matrix
fn mk_voxel_cube() -> Matrix3d {
    let mut cube_sq: Matrix3d = vec![];
    for _z in 0..=Z_SIZE {
        let mut plane = vec![];
        for _y in 0..=Y_SIZE {
            let mut row = vec![];
            for x in 0..=X_SIZE {
                row.push(x);
            }
            plane.push(row);
        }
        cube_sq.push(plane);
    }
    cube_sq
}

const IMAGE_PATH: &str = "smeil.png";

#[derive(Component, Debug)]
struct Mother;

fn add_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load(IMAGE_PATH);
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });

    let shape = meshes.add(shape::Cube::default().into());

    let c = mk_voxel_cube();

    commands
        .spawn((
            Mother,
            TransformBundle::default(),
            Visibility::Inherited,
            InheritedVisibility::default(),
        ))
        .with_children(|ch| {
            for (z, plane) in c.iter().enumerate() {
                for (y, row) in plane.iter().enumerate() {
                    for (x, _n) in row.iter().enumerate() {
                        let [x, y, z] = [x as f32, y as f32, z as f32];

                        ch.spawn((
                            PbrBundle {
                                mesh: shape.clone(),
                                material: debug_material.clone(),
                                transform: Transform::from_xyz(
                                    x - (X_SIZE as f32 / 2.0),
                                    y + 1.0,
                                    z,
                                ),
                                ..default()
                            },
                            Shape { x, y, z },
                        ));
                    }
                }
            }
        });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 9000.0,
    //         range: 100.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 16.0, 8.0),
    //     ..default()
    // });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            range: 100.,
            shadows_enabled: true,
            color: Color::RED,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 24.0, -4.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            range: 100.,
            shadows_enabled: true,
            color: Color::GREEN,
            ..default()
        },
        transform: Transform::from_xyz(15.0, 8.0, 14.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            range: 100.,
            shadows_enabled: true,
            color: Color::BLUE,
            ..default()
        },
        transform: Transform::from_xyz(-2., 8., -16.),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(16.0, 20.0, -10.0)
            .looking_at(Vec3::new(0., 5., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Mother>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 1.);
    }
}
fn flicker(
    mut commands: Commands,
    mut query: Query<(Entity, &Visibility, &Shape)>,
    time: Res<Time>,
) {
    for (entity, mut visibility, shape) in &mut query {
        let Shape { x, y, z } = shape;
        let f = f32::floor(time.elapsed_seconds() * 15.0 + x + y + z) as i32
            % (X_SIZE * Y_SIZE / 5)
            == 0;
        if f {
            visibility = &Visibility::Hidden;
        } else {
            visibility = &Visibility::Visible;
        }
        commands.entity(entity).insert(*visibility);
    }
}
