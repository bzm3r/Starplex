use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::texture::Image;

use vello::{Scene, SceneBuilder};

use crate::{fragment::{VelloFragmentQuery, VelloFragment}, target::VelloTarget};

// Vello `Scene`s contain the `Encoding` that will be sent to and rendered by the GPU
#[derive(Component)]
pub struct VelloScene {
    scene: Scene,
    fragments: Vec<VelloFragment>,
    target: VelloTarget,
}

pub struct RenderScene()

impl VelloScene {
    pub fn new(target: Handle<Image>) -> Self {
        Self { scene: Scene::default(), fragments: Vec::new(), target }
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
