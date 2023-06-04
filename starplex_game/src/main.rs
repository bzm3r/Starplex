use vello::kurbo::{Affine, Point, Rect};
use vello::peniko::{Color, Fill, Gradient, Stroke};
use vello::SceneBuilder;

use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};

use vevy_bello::fragment::VelloFragment;
use vevy_bello::target::VelloTarget;
use vevy_bello::VelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(VelloPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
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
    // let size = Extent3d {
    //     width: 512,
    //     height: 512,
    //     ..default()
    // };

    // // This is the texture that will be rendered to.
    // let mut image = Image {
    //     texture_descriptor: TextureDescriptor {
    //         label: None,
    //         size,
    //         dimension: TextureDimension::D2,
    //         format: TextureFormat::Rgba8Unorm,
    //         mip_level_count: 1,
    //         sample_count: 1,
    //         usage: TextureUsages::TEXTURE_BINDING
    //             | TextureUsages::COPY_DST
    //             | TextureUsages::STORAGE_BINDING,
    //         view_formats: &[],
    //     },
    //     ..default()
    // };

    // // fill image.data with zeroes
    // image.resize(size);

    // let image_handle = images.add(image);

    // // Light
    // // NOTE: Currently lights are shared between passes - see https://github.com/bevyengine/bevy/issues/3462
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    //     ..default()
    // });

    // let cube_size = 4.0;
    // let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    // let material_handle = materials.add(StandardMaterial {
    //     base_color_texture: Some(image_handle.clone()),
    //     reflectance: 0.02,
    //     unlit: false,
    //     ..default()
    // });

    // Main pass cube, with material containing the rendered first pass texture.
    // commands.spawn((
    //     PbrBundle {
    //         mesh: cube_handle,
    //         material: material_handle,
    //         transform: Transform::from_xyz(0.0, 0.0, 1.5)
    //             .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 5.0)),
    //         ..default()
    //     },
    //     MainPassCube,
    // ));

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // commands.spawn((VelloFragment::default(), VelloTarget::new(image_handle)));
    commands.spawn(VelloFragment::default());
}

fn render_fragment(mut fragment: Query<&mut VelloFragment>, mut frame: Local<usize>) {
    let mut fragment = fragment.single_mut();
    let mut builder = fragment.scene_builder();
    render_brush_transform(&mut builder, *frame);
    let th = (std::f64::consts::PI / 180.0) * (*frame as f64);
    fragment.transform = Some(around_center(Affine::rotate(th), Point::default()));
    *frame += 1;
}

fn render_brush_transform(sb: &mut SceneBuilder, _i: usize) {
    let linear = Gradient::new_linear((0.0, 0.0), (0.0, 200.0)).with_stops([
        Color::RED,
        Color::GREEN,
        Color::BLUE,
    ]);
    sb.fill(
        Fill::NonZero,
        Affine::translate((106.0, 106.0)),
        &linear,
        None, //Some(around_center(Affine::rotate(th), Point::new(150.0, 150.0))),
        &Rect::from_origin_size(Point::default(), (300.0, 300.0)),
    );
    sb.stroke(
        &Stroke::new(106.0),
        Affine::IDENTITY,
        &linear,
        None,
        &Rect::from_origin_size(Point::default(), (406.0, 406.0)),
    );
}

fn around_center(transform: Affine, center: Point) -> Affine {
    Affine::translate(center.to_vec2()) * transform * Affine::translate(-center.to_vec2())
}
