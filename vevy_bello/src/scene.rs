use bevy::ecs::component::Component;
use bevy::ecs::query::WorldQuery;
use bevy::render::extract_component::ExtractComponent;

use vello::{kurbo::Affine, Scene, SceneBuilder, SceneFragment};

use crate::{fragment::VelloFragment, target::VelloTarget};

// Vello `Scene`s contain the `Encoding` that will be sent to and rendered by the GPU
#[derive(Component)]
pub struct VelloScene {
    pub scene: Scene,
    pub target: VelloTarget,
}

impl VelloScene {
    pub fn from_fragment(
        scene_frag: &SceneFragment,
        // transform: Option<Affine>,
        target: VelloTarget,
    ) -> Self {
        let mut scene = Scene::default();

        let mut builder = SceneBuilder::for_scene(&mut scene);
        builder.append(scene_frag, None);

        Self { scene, target }
    }
}

#[derive(WorldQuery)]
pub struct VelloSceneCreationQuery {
    pub fragment: &'static VelloFragment,
    pub target: &'static VelloTarget,
}

// Extracts a VelloScene for Rendering to the GPU
impl ExtractComponent for VelloScene {
    type Query = VelloSceneCreationQuery;

    // type Query = (&'static VelloFragment, &'static VelloTarget);

    type Filter = ();

    type Out = Self;

    fn extract_component(
        frag_query: bevy::ecs::query::QueryItem<'_, Self::Query>, //(fragment, target): bevy::ecs::query::QueryItem<'_, Self::Query>,
    ) -> Option<Self> {
        Some(VelloScene::from_fragment(
            &frag_query.fragment.scene_fragment,
            frag_query.target.clone(),
            //target.clone(),
        ))
    }

    // fn extract_component(frag_query: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Option<Self> {
    //     Some(VelloScene::from_fragment(
    //         &frag_query.fragment.scene_fragment,
    //         // frag_query.fragment.transform,
    //         frag_query.target.clone(),
    //     ))
    // }
}
