pub mod fragment;
pub mod node;
pub mod renderer;
pub mod scene;
pub mod target;

use bevy::core_pipeline::core_3d;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;
use node::VelloNode;
use renderer::VelloRenderer;
use scene::VelloScene;

pub struct VelloPlugin;

impl Plugin for VelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<VelloScene>::default());
        // .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        //     // Uncomment this to override the default log settings:
        //     // level: bevy::log::Level::TRACE,
        //     // filter: "wgpu=warn,bevy_ecs=info".to_string(),
        //     ..default()
        // }));

        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        // // This should probably use the render graph, but working out the dependencies there is awkward
        // render_app.add_systems(Render, render_scenes.in_set(RenderSet::Render));

        // Bevy's renderer uses a render graph which is a collection of nodes in a directed acyclic graph.
        // It currently runs on each view/camera and executes each node in the specified order.
        // It will make sure that any node that needs a dependency from another node
        // only runs when that dependency is done.
        //
        // Each node can execute arbitrary work, but it generally runs at least one render pass.
        // A node only has access to the render world, so if you need data from the main world
        // you need to extract it manually or with the plugin like above.
        // Add a [`Node`] to the [`RenderGraph`]
        // The Node needs to impl FromWorld
        render_app
            .add_render_graph_node::<VelloNode>(
                // Specifiy the name of the graph, in this case we want the graph for 3d
                core_3d::graph::NAME,
                // It also needs the name of the node
                VelloNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                &[
                    core_3d::graph::node::TONEMAPPING,
                    VelloNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
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
