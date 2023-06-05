use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraphContext},
        renderer::{RenderContext, RenderQueue},
    },
};

use crate::{renderer::VelloRenderer, scene::VelloScene};

/// The post process node used for the render graph
pub struct VelloDrawNode {
    // The node needs a query to gather data from the ECS in order to do its rendering,
    // but it's not a normal system so we need to define it manually.
    scene_query: QueryState<&'static VelloScene>,
}

impl VelloDrawNode {
    pub const NAME: &str = "vello_render";
}

impl FromWorld for VelloDrawNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            scene_query: QueryState::new(world),
        }
    }
}

impl Node for VelloDrawNode {
    // This will run every frame before the run() method
    // The important difference is that `self` is `mut` here
    fn update(&mut self, world: &mut World) {
        // Since this is not a system we need to update the query manually.
        // This is mostly boilerplate. There are plans to remove this in the future.
        // For now, you can just copy it.
        self.scene_query.update_archetypes(world);
    }

    // Runs the node logic
    // This is where you encode draw commands.
    //
    // This will run on every view on which the graph is running. If you don't want your effect to run on every camera,
    // you'll need to make sure you have a marker component to identify which camera(s) should run the effect.
    fn run(
        &self,
        _graph_context: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the GPU images
        let gpu_images = world.resource::<RenderAssets<Image>>();

        // Get the Vello renderer
        let renderer = world.resource::<VelloRenderer>();
        // Get the render device
        let device = render_context.render_device();
        let queue = world.resource::<RenderQueue>();

        for scene in self.scene_query.iter_manual(world) {
            info!("Found a VelloScene to render!");
            let target_gpu_image = gpu_images.get(scene.target.handle()).unwrap();
            let params = vello::RenderParams {
                base_color: vello::peniko::Color::AQUAMARINE,
                width: target_gpu_image.size.x as u32,
                height: target_gpu_image.size.y as u32,
            };

            renderer.try_render_to_texture(
                device.wgpu_device(),
                queue,
                &scene.scene,
                &target_gpu_image.texture_view,
                &params,
            );
        }

        Ok(())
    }
}
