pub mod fragment;
pub mod scene;
pub mod target;

use bevy::prelude::*;
use bevy::render::{
    extract_component::ExtractComponent, render_asset::RenderAssets, renderer::RenderDevice,
    renderer::RenderQueue, Render, RenderApp, RenderSet,
};
use vello::{Renderer, RendererOptions, Scene, SceneBuilder, SceneFragment};

pub struct VelloPlugin;



#[derive(Resource)]
pub struct VelloRenderer(Renderer);

impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
        VelloRenderer(
            Renderer::new(
                device.wgpu_device(),
                &RendererOptions {
                    surface_format: None,
                },
            )
            .unwrap(),
        )
    }
}

fn render_scenes(
    mut renderer: ResMut<VelloRenderer>,
    mut scenes: Query<&VelloScene>,
    gpu_images: Res<RenderAssets<Image>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    for scene in &mut scenes {
        let gpu_image = gpu_images.get(&scene.1).unwrap();
        let params = vello::RenderParams {
            base_color: vello::peniko::Color::AQUAMARINE,
            width: gpu_image.size.x as u32,
            height: gpu_image.size.y as u32,
        };
        renderer
            .0
            .render_to_texture(
                device.wgpu_device(),
                &queue,
                &scene.0,
                &gpu_image.texture_view,
                &params,
            )
            .unwrap();
    }
}

impl Plugin for VelloPlugin {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app.init_resource::<VelloRenderer>();
        // This should probably use the render graph, but working out the dependencies there is awkward
        render_app.add_systems(Render, render_scenes.in_set(RenderSet::Render));
    }
}
