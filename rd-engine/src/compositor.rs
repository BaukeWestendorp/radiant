use std::{collections::HashMap, sync::Arc};

use zeevonk::{
    project::{FixtureId, IntoFixtureId, IntoFixtureIds},
    value::AttributeValues,
};

use crate::{Cue, CueList, EffectAgent, ObjectRegistry, Programmer, Recipe, RecipeContent};

pub struct Compositor {
    highlighted_fixtures: Vec<FixtureId>,

    objects: Arc<ObjectRegistry>,
    programmer: Arc<Programmer>,
    effect_agent: Arc<EffectAgent>,
}

impl Compositor {
    pub fn new(
        objects: Arc<ObjectRegistry>,
        programmer: Arc<Programmer>,
        effect_agent: Arc<EffectAgent>,
    ) -> Self {
        Self { highlighted_fixtures: Vec::new(), objects, programmer, effect_agent }
    }

    pub fn highlight_fixture(&mut self, fixture_id: impl IntoFixtureId) {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return };
        if !self.highlighted_fixtures.contains(&fixture_id) {
            self.highlighted_fixtures.push(fixture_id);
        }
    }

    pub fn highlight_fixtures(&mut self, fixture_ids: impl IntoFixtureIds) {
        for fixture_id in fixture_ids.into_fixture_ids() {
            if !self.highlighted_fixtures.contains(&fixture_id) {
                self.highlighted_fixtures.push(fixture_id);
            }
        }
    }

    pub fn unhighlight_fixture(&mut self, fixture_id: impl IntoFixtureId) {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return };
        self.highlighted_fixtures.retain(|id| id != &fixture_id);
    }

    pub fn unhighlight_fixtures(&mut self, fixture_ids: impl IntoFixtureIds) {
        let fixture_ids = fixture_ids.into_fixture_ids().collect::<Vec<_>>();
        self.highlighted_fixtures.retain(|id| !fixture_ids.contains(id));
    }

    pub fn is_fixture_highlighted(&self, fixture_id: impl IntoFixtureId) -> bool {
        let Some(fixture_id) = fixture_id.into_fixture_id() else { return false };
        self.highlighted_fixtures.contains(&fixture_id)
    }

    pub fn highlighted_fixtures(&self) -> &[FixtureId] {
        &self.highlighted_fixtures
    }

    pub fn compose<'a>(&'a self) -> anyhow::Result<Composition<'a>> {
        let mut attribute_values = self.programmer.programmed_values().clone();

        for cue_list in self.objects.get_all::<CueList>() {
            let Some(cue) = cue_list.cues().first() else { continue };
            self.compose_cue(cue, &mut attribute_values)?;
        }

        Ok(Composition { attribute_values, highlighted_fixtures: &self.highlighted_fixtures })
    }

    fn compose_cue(&self, cue: &Cue, attribute_values: &mut AttributeValues) -> anyhow::Result<()> {
        for recipe in cue.recipes() {
            self.compose_recipe(recipe, attribute_values)?;
        }

        Ok(())
    }

    fn compose_recipe(
        &self,
        recipe: &Recipe,
        attribute_values: &mut AttributeValues,
    ) -> anyhow::Result<()> {
        match recipe.content() {
            RecipeContent::Effect { effect } => {
                let fixture_ids = recipe.fixture_collection().to_fixture_ids(&self.objects);
                let Some(effect) = self.objects.get(*effect) else {
                    anyhow::bail!("Could not find effect '{:?}'", effect);
                };
                let mut parameters = HashMap::new();
                self.effect_agent.tick(recipe.id(), effect, &fixture_ids, &mut parameters);

                for (fixture_id, params) in parameters {
                    for param in params {
                        for (attribute, value) in param.to_attribute_values() {
                            attribute_values.set(fixture_id, attribute, value);
                        }
                    }
                }
            }
            RecipeContent::Static(params) => {
                let fixture_ids = recipe.fixture_collection().to_fixture_ids(&self.objects);
                for fixture_id in fixture_ids {
                    for param in params {
                        for (attribute, value) in param.to_attribute_values() {
                            attribute_values.set(*fixture_id, attribute, value);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct Composition<'a> {
    pub attribute_values: AttributeValues,
    pub highlighted_fixtures: &'a [FixtureId],
}
