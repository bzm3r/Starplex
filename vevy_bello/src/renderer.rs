use bevy::{
    prelude::{FromWorld, Resource, World},
    render::renderer::{RenderDevice, RenderQueue},
};
use vello::{Renderer, RendererOptions};

#[derive(Resource)]
pub struct VelloRenderer(pub Renderer);

impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        VelloRenderer(
            Renderer::new(
                device.wgpu_device(),
                &RendererOptions {
                    surface_format: None,
                    timestamp_period: queue.0.get_timestamp_period(),
                },
            )
            .unwrap(),
        )
    }
}
