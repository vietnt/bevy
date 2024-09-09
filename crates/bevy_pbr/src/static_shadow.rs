use bevy_ecs::{component::Component, system::Resource};
use bevy_render::render_resource::{Extent3d, ImageCopyTexture};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct StaticShadowCaster;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct StaticShadowMapMarker(pub u32);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct StaticShadowRequestUpdate;

#[derive(Resource, Default)]
pub struct StaticShadowMap {
    pub vecs: Vec<(usize, usize, Extent3d)>,
}
