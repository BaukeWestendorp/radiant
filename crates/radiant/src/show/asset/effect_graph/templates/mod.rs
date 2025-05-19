mod asset;
mod context;
mod math;
mod pipeline;

pub fn insert_templates(graph: &mut super::EffectGraph) {
    math::insert_templates(graph);
    context::insert_templates(graph);
    pipeline::insert_templates(graph);
    asset::insert_templates(graph);
}
