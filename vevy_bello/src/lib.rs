pub mod blit;
pub mod draw;
pub mod fragment;
pub mod renderer;
pub mod scene;
pub mod target;

use bevy::asset::load_internal_asset;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::core_2d::extract_core_2d_camera_phases;
use bevy::core_pipeline::fullscreen_vertex_shader::FULLSCREEN_SHADER_HANDLE;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::{Render, RenderApp, RenderSet};
use blit::{queue_blit_out_pipelines, BlitOutNode};
use draw::VelloDrawNode;
use renderer::VelloRenderer;
use scene::VelloScene;

use bevy::core_pipeline::blit::BlitPlugin;

pub const VELLO: &str = "vello";
pub const DRAW: &str = VelloDrawNode::NAME;
pub const BLIT_OUT: &str = BlitOutNode::NAME;
pub struct VelloPlugin;

impl Plugin for VelloPlugin {
    fn build(&self, app: &mut App) {
        // Basically like what bevy_core_pipeline/lib.rs, but with all core pipelines except for
        // blitting removed
        load_internal_asset!(
            app,
            FULLSCREEN_SHADER_HANDLE,
            "fullscreen_vertex_shader/fullscreen.wgsl",
            Shader::from_wgsl
        );

        app.register_type::<ClearColor>()
            .register_type::<ClearColorConfig>()
            .init_resource::<ClearColor>()
            .add_plugin(ExtractResourcePlugin::<ClearColor>::default())
            .add_plugin(BlitPlugin);

        // Basically like core_2d, except with all the mesh based drawing stuff removed
        app.register_type::<Camera2d>()
            .add_plugin(ExtractComponentPlugin::<Camera2d>::default())
            .add_plugin(ExtractComponentPlugin::<VelloScene>::default());

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .add_systems(ExtractSchedule, extract_core_2d_camera_phases)
            .add_systems(Render, queue_blit_out_pipelines.in_set(RenderSet::Queue));

        render_app
            .add_render_sub_graph(VELLO)
            .add_render_graph_node::<VelloDrawNode>(VELLO, DRAW)
            .add_render_graph_node::<BlitOutNode>(VELLO, BLIT_OUT)
            .add_render_graph_edges(VELLO, &[DRAW, BLIT_OUT]);
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Initialize the Vello Renderer
        render_app.init_resource::<VelloRenderer>();
    }
}
