mod context;
mod math;

pub fn insert_templates(graph: &mut super::EffectGraph) {
    math::insert_templates(graph);
    context::insert_templates(graph);
}
