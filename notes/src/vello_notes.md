# Vello Notes

* (from the [roadmap](https://github.com/linebender/vello/blob/main/doc/roadmap_2023.md)) The current state is a "scene fragment" API, specifically so that fragments can be built in multiple threads and then efficiently assembled into a scene.

* Question about `SceneBuilder`'s `append`: currently, `SceneBuilder`'s `append` looks like:

```
/// Appends a fragment to the scene.
    pub fn append(&mut self, fragment: &SceneFragment, transform: Option<Affine>) {
        self.scene.append(
            &fragment.data,
            &transform.map(|xform| Transform::from_kurbo(&xform)),
        );
    }
```

It takes an optional [`kurbo::Affine`](https://docs.rs/kurbo/latest/kurbo/struct.Affine.html) and maps it into a [`vello::Transform`](https://github.com/linebender/vello/blob/ea224b459c48267e0e63747c9b192d62ef432ac9/crates/encoding/src/math.rs#L12). Note the details of the two types: there is a possible cost to the conversion from `f64` -> `f32`

Would it make sense to provide a method on `SceneBuilder` that is like `append`, but takes directly an  `Option<vello::Transform>`? (Possible use case: the user has previously computed and cached the `vello::Transform`s from their relevant `kurbo::Affine`s)

The answer to this question should come from profiling results that suggest that there is a benefit to this caching the results of this conversion.

If there is a benefit: to avoid proliferation of `append`-like methods, we could have an `AffineLike` trait `append` would take an `Option<vello::AffineLike>`? A required method on `AffineLike` would be `into_transform`, which maps the `AffineLike` into a `vello::Transform`. (For the `impl AffineLike`of `vello::Transform`, this would be the identity, which hopefully the compiler can "simplify" away?)

Then `append` would look like:

```rust
/// Appends a fragment to the scene.
    pub fn append(&mut self, fragment: &SceneFragment, transform: Option<AffineLike>) {
        self.scene.append(
            &fragment.data,
            &transform.into_transform()),
        );
    }
```
