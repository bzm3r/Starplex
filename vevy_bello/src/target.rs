use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::texture::Image;

#[derive(Component)]
pub struct VelloTarget(Handle<Image>);

impl VelloTarget {
    pub fn clone_handle(&self) -> Handle<Image> {
        self.0.clone()
    }
}
