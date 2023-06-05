use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::texture::Image;

/// A [`Handle`](bevy::asset::Handle) to a target [`Image`](bevy::asset::Image) [`Image`](bevy::render::texture::Image)
/// that a [`VelloScene`] will be drawn to.  
///
/// It is a wrapper around a [`Handle`](bevy::asset::Handle) for an [`Image`](bevy::asset::Image)
/// so that we can make it a [`Component`](bevy::ecs::component::Component).
#[derive(Component, Clone)]
pub struct VelloTarget {
    pub handle: Handle<Image>,
    pub size: (f32, f32),
}

impl VelloTarget {
    pub fn new(image: Handle<Image>, size: (f32, f32)) -> Self {
        VelloTarget { handle: image, size }
    }

    pub fn clone_handle(&self) -> Handle<Image> {
        self.handle.clone()
    }
}
