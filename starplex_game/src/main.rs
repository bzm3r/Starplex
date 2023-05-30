use bevy::render::{Render, RenderSet};
use vello::kurbo::{Affine, Point, Rect};
use vello::peniko::{Color, Fill, Gradient, Stroke};
use vello::{Renderer, RendererOptions, Scene, SceneBuilder, SceneFragment};

use bevy::{
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        renderer::{RenderDevice, RenderQueue},
        RenderApp,
    },
};

use vevy_bello::fragment::VelloFragment;
use vevy_bello::render_scenes;
use vevy_bello::renderer::VelloRenderer;
use vevy_bello::scene::VelloScene;
use vevy_bello::target::VelloTarget;
use vevy_bello::VelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VelloPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, cube_rotator_system)
        .add_systems(Update, render_fragment)
        .run()
}

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Light
    // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 1.5)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 5.0)),
            ..default()
        },
        MainPassCube,
    ));

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        //VelloFragment(SceneFragment::default()),
        VelloFragment::default(),
        VelloTarget::new(image_handle),
    ));
}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        transform.rotate_y(0.7 * time.delta_seconds());
    }
}

fn render_fragment(mut fragment: Query<&mut VelloFragment>, mut frame: Local<usize>) {
    let mut fragment = fragment.single_mut();
    let mut builder = SceneBuilder::for_fragment(&mut fragment.scene_fragment);
    render_brush_transform(&mut builder, *frame);
    *frame += 1;
}

fn render_brush_transform(sb: &mut SceneBuilder, i: usize) {
    let th = (std::f64::consts::PI / 180.0) * (i as f64);
    let linear = Gradient::new_linear((0.0, 0.0), (0.0, 200.0)).with_stops([
        Color::RED,
        Color::GREEN,
        Color::BLUE,
    ]);
    sb.fill(
        Fill::NonZero,
        Affine::translate((106.0, 106.0)),
        &linear,
        Some(around_center(Affine::rotate(th), Point::new(150.0, 150.0))),
        &Rect::from_origin_size(Point::default(), (300.0, 300.0)),
    );
    sb.stroke(
        &Stroke::new(106.0),
        Affine::IDENTITY,
        &linear,
        Some(around_center(
            Affine::rotate(th + std::f64::consts::PI / 2.),
            Point::new(176.5, 176.5),
        )),
        &Rect::from_origin_size(Point::new(53.0, 53.0), (406.0, 406.0)),
    );
}

fn around_center(transform: Affine, center: Point) -> Affine {
    Affine::translate(center.to_vec2()) * transform * Affine::translate(-center.to_vec2())
}
