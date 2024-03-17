use gpui::{
    div, rgb, AnyElement, IntoElement, Model, ParentElement, Pixels, Render, Styled, View,
    ViewContext, VisualContext,
};
use itertools::Itertools;

use crate::show::patch::Fixture;
use crate::show::Show;
use crate::ui::sheet::{Sheet, SheetDelegate};

use super::Window;

pub struct FixtureSheetWindow {
    sheet: View<Sheet<FixtureSheetWindowDelegate>>,
}

impl FixtureSheetWindow {
    pub fn build(show: Model<Show>, cx: &mut ViewContext<Window>) -> View<Self> {
        cx.new_view(|cx| {
            let fixtures = show.read(cx).clone().patch_list.fixtures;
            let sheet_delegate = FixtureSheetWindowDelegate::new(show.clone(), fixtures);
            let sheet = cx.new_view(|_cx| Sheet::new(sheet_delegate, "fixture_sheet"));

            Self { sheet }
        })
    }
}

impl Render for FixtureSheetWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.sheet.clone())
    }
}

pub struct FixtureSheetWindowDelegate {
    show: Model<Show>,

    fixtures: Vec<Fixture>,
}

impl FixtureSheetWindowDelegate {
    pub fn new(show: Model<Show>, fixtures: Vec<Fixture>) -> Self {
        Self { show, fixtures }
    }
}

impl SheetDelegate for FixtureSheetWindowDelegate {
    type Data = Fixture;
    type ColumnId = FixtureSheetColumnId;

    fn columns(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<Self::ColumnId> {
        let mut labels = vec![
            FixtureSheetColumnId::Id,
            FixtureSheetColumnId::Name,
            FixtureSheetColumnId::Patch,
            FixtureSheetColumnId::Mode,
        ];

        let mut attribute_labels = self.show.update(cx, |show, _cx| {
            let all_used_attributes = show.patch_list.all_used_attributes();

            all_used_attributes
                .iter()
                .map(|a| FixtureSheetColumnId::Attribute(a.name.value().to_string()))
                .unique()
                .collect::<Vec<_>>()
        });

        labels.append(&mut attribute_labels);

        labels
    }

    fn values(&self, _cx: &mut ViewContext<Sheet<Self>>) -> &Vec<Self::Data> {
        &self.fixtures
    }

    fn column_width(
        &self,
        column_id: &Self::ColumnId,
        _cx: &mut ViewContext<Sheet<Self>>,
    ) -> Pixels {
        match column_id {
            FixtureSheetColumnId::Id => 50.0,
            FixtureSheetColumnId::Name => 100.0,
            FixtureSheetColumnId::Patch => 80.0,
            FixtureSheetColumnId::Mode => 150.0,
            FixtureSheetColumnId::Attribute(_) => 120.0,
        }
        .into()
    }

    fn header_label(
        &self,
        column_id: &Self::ColumnId,
        _cx: &mut ViewContext<Sheet<Self>>,
    ) -> String {
        match column_id {
            FixtureSheetColumnId::Id => "ID".to_string(),
            FixtureSheetColumnId::Name => "Name".to_string(),
            FixtureSheetColumnId::Patch => "Patch".to_string(),
            FixtureSheetColumnId::Mode => "Mode".to_string(),
            FixtureSheetColumnId::Attribute(name) => name.to_string(),
        }
    }

    fn render_cell_content(
        &self,
        column_id: &Self::ColumnId,
        fixture: &Self::Data,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        match column_id {
            FixtureSheetColumnId::Id => render_value(fixture.id),
            FixtureSheetColumnId::Name => render_value(Some(fixture.name.clone())),
            FixtureSheetColumnId::Patch => render_value(fixture.patch.clone()),
            FixtureSheetColumnId::Mode => {
                let name = self.show.update(cx, |show, _cx| {
                    show.patch_list.fixture_type(fixture).dmx_modes.modes[fixture.mode_index]
                        .name
                        .clone()
                });
                render_value(Some(name))
            }
            FixtureSheetColumnId::Attribute(name) => {
                let values_string = self.show.update(cx, |show, _cx| {
                    show.patch_list
                        .fixture_type(fixture)
                        .used_dmx_channels_for_mode(fixture.mode_index)
                        .unwrap()
                        .iter()
                        .find_map(|c| {
                            let attribute = c
                                .logical_channels
                                .get(0)
                                .unwrap()
                                .attribute
                                .references()
                                .get(0)?;

                            let offsets = c.offset.as_ref()?;

                            if attribute == name {
                                let values_string = offsets
                                    .iter()
                                    .map(|v| v.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                Some(values_string)
                            } else {
                                None
                            }
                        })
                });

                render_value(values_string)
            }
        }
        .into_any_element()
    }
}

fn render_value<T: ToString>(value: Option<T>) -> AnyElement {
    match value {
        Some(value) => div().child(value.to_string()),
        None => div().child("None").text_color(rgb(0x666666)),
    }
    .into_any_element()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FixtureSheetColumnId {
    Id,
    Name,
    Patch,
    Mode,
    Attribute(String),
}
