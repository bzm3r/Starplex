use starplex_lib::vello_plugin::{VelloFragment, VelloPlugin, VelloScene};
use vello::kurbo::{Affine, Point, Rect};
use vello::peniko::{Color, Fill, Gradient, Stroke};
use vello::{SceneBuilder, SceneFragment};

use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VelloPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugin(ExtractComponentPlugin::<VelloScene>::default())
        .add_systems(Update, render_fragment)
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

    // The main pass camera.
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    commands.spawn((
        VelloFragment(SceneFragment::default()),
        VelloTarget(image_handle),
    ));
}

fn render_fragment(mut fragment: Query<&mut VelloFragment>, mut frame: Local<usize>) {
    let mut fragment = fragment.single_mut();
    let mut builder = SceneBuilder::for_fragment(&mut fragment.0);
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
