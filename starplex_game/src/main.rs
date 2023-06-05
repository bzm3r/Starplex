use bevy::window::{PrimaryWindow, WindowResized};
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Starplex".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(VelloPlugin)
        .add_systems(PreStartup, maximize_window)
        .add_systems(Startup, (setup_camera, setup_fragment_and_target))
        .add_systems(Update, resize_vello_target)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, draw_to_fragment)
        .add_systems(PreUpdate, find_vello_target)
        .run()
}

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

fn maximize_window(mut primary_window_q: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = primary_window_q.single_mut();
    primary_window.set_maximized(true);
}

fn setup_camera(mut commands: Commands) {
    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_fragment_and_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = primary_window_q.single();

    let size = Extent3d {
        width: primary_window.physical_width(),
        height: primary_window.physical_height(),
        ..default()
    };

    // This is the image that the fragment will render to.
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
    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn((VelloFragment::default(), VelloTarget::new(image_handle)));
}

fn resize_vello_target(
    target_q: Query<&mut VelloTarget>,
    mut resize_reader: EventReader<WindowResized>,
    mut images: ResMut<Assets<Image>>,
) {
    // Take the last resize event.
    let mut window_resized = None;
    for event in resize_reader.iter() {
        window_resized = Some(event);
    }

    // Resize image for post-processing material and re-spawn main camera.
    if let Some(event) = window_resized {
        if let Ok(vello_target) = target_q.get_single() {
            info!("Resizing vello target!");
            if let Some(target_image) = images.get_mut(vello_target.handle()) {
                // TODO: use physical dimensions.
                let size = Extent3d {
                    width: event.width as u32,
                    height: event.height as u32,
                    ..default()
                };
                target_image.texture_descriptor.size = size;
                target_image.resize(target_image.texture_descriptor.size);
            }
        }
    }
}

fn draw_to_fragment(
    mut fragment: Query<(&mut VelloFragment, &VelloTarget)>,
    mut frame: Local<usize>,
) {
    let (mut fragment, target) = fragment.single_mut();
    let mut builder = fragment.scene_builder();
    draw_stuff(&mut builder, *frame);
    let th = (std::f64::consts::PI / 180.0) * (*frame as f64);
    fragment.transform = Some(around_center(Affine::rotate(th), Point::new(target.handle().)));
    *frame += 1;
}

fn draw_stuff(sb: &mut SceneBuilder, _i: usize) {
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

fn find_vello_target(target_q: Query<&VelloTarget>) {
    for _ in target_q.iter() {
        info!("find_vello_target found a target!");
    }
}
