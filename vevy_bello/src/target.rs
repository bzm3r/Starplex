use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::texture::Image;

#[derive(Component, Clone)]
pub struct VelloTarget(Handle<Image>);

impl VelloTarget {
    pub fn get_handle_ref(&self) -> &Handle<Image> {
        &self.0
    }

    pub fn clone_handle(&self) -> Handle<Image> {
        self.0.clone()
    }
}
