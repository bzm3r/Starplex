use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::ecs::query::{ReadOnlyWorldQuery, WorldQuery};
use bevy::render::texture::Image;

use vello::peniko::kurbo::Affine;
use vello::{SceneBuilder, SceneFragment};

use crate::target::VelloTarget;

// Vello [`SceneFragment`]s are stitched together into a Vello [`Scene`](vello::Scene)
#[derive(Component)]
pub struct VelloFragment {
    pub scene_fragment: SceneFragment,
    pub transform: Affine,
}

#[derive(WorldQuery)]
pub struct VelloFragmentQuery {
    pub fragment: &'static VelloFragment,
    pub target: &'static VelloTarget,
}
