use std::sync::Mutex;

use bevy::{
    prelude::{FromWorld, Resource, World},
    render::renderer::{RenderDevice, RenderQueue},
};
use vello::{RenderParams, Renderer, RendererOptions, Scene};
use wgpu::{Device, Queue, TextureView};

/// Wraps a Vello [`Renderer`](vello::Renderer) for use as a Bevy [`Resource`](bevy::prelude::Resource).
///
/// Internally, the Vello [`Renderer`](vello::Renderer) is wrapped in a [`Mutex`](std::sync::Mutex), so that
/// it can be called from multi-threaded contexts that only have access to `VelloRenderer` by reference.  
#[derive(Resource)]
pub struct VelloRenderer(pub Mutex<Renderer>);

impl VelloRenderer {
    /// Try rendering to a texture, using the internal [`Mutex`](std::sync::Mutex) wrapped
    /// Vello [`Renderer`](vello::Renderer).
    ///
    /// Currently, this method panics if:
    ///     * access through the [`Mutex`](std::sync::Mutex) fails.
    ///     * the call to Vello's [`render_to_texture`](vello::Renderer::render_to_texture) fails.
    pub fn try_render_to_texture(
        &self,
        device: &Device,
        queue: &Queue,
        scene: &Scene,
        texture_view: &TextureView,
        params: &RenderParams,
    ) {
        self.0
            .lock()
            .unwrap()
            .render_to_texture(device, queue, scene, texture_view, params)
            .unwrap();
    }
}
impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();
        let queue = world.get_resource::<RenderQueue>().unwrap();

        VelloRenderer(Mutex::new(
            Renderer::new(
                device.wgpu_device(),
                &RendererOptions {
                    surface_format: None,
                    timestamp_period: queue.0.get_timestamp_period(),
                },
            )
            .unwrap(),
        ))
    }
}
