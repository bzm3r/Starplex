use bevy::{
    prelude::{FromWorld, Resource, World},
    render::renderer::RenderDevice,
};
use vello::{Renderer, RendererOptions};

#[derive(Resource)]
pub struct VelloRenderer(pub Renderer);

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
