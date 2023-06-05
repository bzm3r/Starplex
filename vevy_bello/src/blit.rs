use bevy::core_pipeline::blit::{BlitPipeline, BlitPipelineKey};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{CachedRenderPipelineId, SpecializedRenderPipelines};
use bevy::render::{
    camera::{CameraOutputMode, ExtractedCamera},
    render_graph::{Node, NodeRunError, RenderGraphContext},
    render_resource::{
        BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, LoadOp, Operations,
        PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, SamplerDescriptor,
        TextureViewId,
    },
    renderer::RenderContext,
    view::ViewTarget,
};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::scene::VelloScene;

#[derive(Component)]
pub struct BlitOutPipeline {
    cached_id: CachedRenderPipelineId,
}

pub fn queue_blit_out_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<BlitPipeline>>,
    blit_pipeline: Res<BlitPipeline>,
    view_targets: Query<(Entity, &ViewTarget, Option<&ExtractedCamera>)>,
) {
    for (entity, view_target, camera) in view_targets.iter() {
        let blend_state = if let Some(ExtractedCamera {
            output_mode: CameraOutputMode::Write { blend_state, .. },
            ..
        }) = camera
        {
            *blend_state
        } else {
            None
        };
        let key = BlitPipelineKey {
            texture_format: view_target.out_texture_format(),
            blend_state,
            samples: 1,
        };
        let cached_id = pipelines.specialize(&pipeline_cache, &blit_pipeline, key);

        commands
            .entity(entity)
            .insert(BlitOutPipeline { cached_id });
    }
}

pub struct BlitOutNode {
    cached_texture_bind_groups: Mutex<HashMap<TextureViewId, BindGroup>>,
    scene_query: QueryState<&'static VelloScene>,
    view_query: QueryState<(&'static ViewTarget, &'static BlitOutPipeline)>,
}

impl BlitOutNode {
    pub const NAME: &str = "blit_out";
}

impl FromWorld for BlitOutNode {
    fn from_world(world: &mut World) -> Self {
        Self {
            cached_texture_bind_groups: Mutex::default(),
            scene_query: QueryState::new(world),
            view_query: QueryState::new(world),
        }
    }
}

impl Node for BlitOutNode {
    // This will run every frame before the run() method
    // The important difference is that `self` is `mut` here
    fn update(&mut self, world: &mut World) {
        // Since this is not a system we need to update the query manually.
        // This is mostly boilerplate. There are plans to remove this in the future.
        // For now, you can just copy it.
        self.scene_query.update_archetypes(world);
        self.view_query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        info!("blitting to output!");
        let view_entity = graph.view_entity();

        let Ok((view_target, blit_out_pipeline)) = self.view_query.get_manual(world, view_entity) else {
            error!("did not find view target + blit out pipeline");
            return Ok(());
        };

        let pipeline_cache = world.get_resource::<PipelineCache>().unwrap();
        let blit_pipeline = world.get_resource::<BlitPipeline>().unwrap();

        let color_attachment_load_op = LoadOp::Clear(Default::default());
        let mut cached_bind_groups = self.cached_texture_bind_groups.lock().unwrap();

        // Get the GPU images
        let gpu_images = world.resource::<RenderAssets<Image>>();

        for (scene_count, vello_scene) in self.scene_query.iter_manual(world).enumerate() {
            info!("blitter found a vello scene! (count: {})", scene_count);

            let target_view = &gpu_images
                .get(vello_scene.target.handle())
                .unwrap()
                .texture_view;

            let bind_group = cached_bind_groups
                .entry(target_view.id())
                .or_insert_with(|| {
                    let sampler = render_context
                        .render_device()
                        .create_sampler(&SamplerDescriptor::default());

                    render_context
                        .render_device()
                        .create_bind_group(&BindGroupDescriptor {
                            label: None,
                            layout: &blit_pipeline.texture_bind_group,
                            entries: &[
                                BindGroupEntry {
                                    binding: 0,
                                    resource: BindingResource::TextureView(target_view),
                                },
                                BindGroupEntry {
                                    binding: 1,
                                    resource: BindingResource::Sampler(&sampler),
                                },
                            ],
                        })
                });

            let pipeline = match pipeline_cache.get_render_pipeline(blit_out_pipeline.cached_id) {
                Some(pipeline) => pipeline,
                None => return Ok(()),
            };

            let pass_descriptor = RenderPassDescriptor {
                label: Some("vello_output_blit_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: view_target.out_texture(),
                    resolve_target: None,
                    ops: Operations {
                        load: color_attachment_load_op,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            };

            let mut render_pass = render_context
                .command_encoder()
                .begin_render_pass(&pass_descriptor);

            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}
