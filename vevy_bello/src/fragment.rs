use bevy::ecs::component::Component;

use vello::peniko::kurbo::Affine;
use vello::{SceneBuilder, SceneFragment};

// Vello [`SceneFragment`]s are stitched together into a Vello [`Scene`](vello::Scene)
#[derive(Component, Default)]
pub struct VelloFragment {
    pub scene_fragment: SceneFragment,
    // pub transform: Option<Affine>,
}

impl VelloFragment {
    pub fn scene_builder(&mut self) -> SceneBuilder {
        SceneBuilder::for_fragment(&mut self.scene_fragment)
    }
}
