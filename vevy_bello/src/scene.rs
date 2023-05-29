// Vello `Scene`s contain the `Encoding` that will be sent to and rendered by the GPU
#[derive(Component)]
pub struct VelloScene {
    scene: Scene,
    target: Handle<Image>,
};

pub struct FragmentQuery {
    fragment: &'static VelloFragment,
    target: Handle<Image>,
}

// Extracts a VelloScene for Rendering to the GPU
impl ExtractComponent for VelloScene {
    type Query = (&'static VelloFragment, &'static Canvas);

    type Filter = ();

    type Out = Self;

    fn extract_component(
        (fragment, target): bevy::ecs::query::QueryItem<'_, Self::Query>,
    ) -> Option<Self> {
        let mut scene = Scene::default();
        let mut builder = SceneBuilder::for_scene(&mut scene);
        builder.append(&fragment.fragment, Some(fragment.transform));
        Some(Self(scene, target.0.clone()))
    }
}
