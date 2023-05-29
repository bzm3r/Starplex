use vello::peniko::kurbo::Affine;

// Vello `SceneFragments` are stitched together into a Vello `Scene` (possibly in a multithreaded fashion)
#[derive(Component)]
pub struct VelloFragment {
    fragment: pub SceneFragment,
    transform: pub Affine,
};
