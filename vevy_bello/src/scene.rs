use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::texture::Image;

use vello::{Scene, SceneBuilder};

use crate::{fragment::VelloFragmentQuery, target::VelloTarget};

// Vello `Scene`s contain the `Encoding` that will be sent to and rendered by the GPU
#[derive(Component)]
pub struct VelloScene {
    id: u64,
    scene: Scene,
    target: VelloTarget,
}

impl VelloScene {
    pub fn new(id: u64, scene: Scene, target: Handle<Image>) -> Self {
        Self { id, scene, target }
    }
}

// Extracts a VelloScene for Rendering to the GPU
impl ExtractComponent for VelloScene {
    type Query = VelloFragmentQuery;

    type Filter = ();

    type Out = Self;

    fn extract_component(frag_query: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Option<Self> {
        let mut scene = Scene::default();
        let mut builder = SceneBuilder::for_scene(&mut scene);
        builder.append(
            &frag_query.fragment.scene_fragment,
            Some(frag_query.fragment.transform),
        );
        Some(Self(scene, frag_query.target.clone_handle()))
    }
}
