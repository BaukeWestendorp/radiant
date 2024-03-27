use backstage::show::{Fixture, Show};
use gpui::{
    div, rgb, AnyElement, IntoElement, Model, ParentElement, Pixels, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use itertools::Itertools;

use crate::ui::sheet::{Sheet, SheetDelegate};

use super::{WindowDelegate, WindowView};

pub struct FixtureSheetWindowDelegate {
    sheet: View<Sheet<FixtureSheetDelegate>>,
}

impl FixtureSheetWindowDelegate {
    pub fn new(show: Model<Show>, cx: &mut WindowContext) -> Self {
        let fixtures = show.read(cx).fixtures();
        let sheet_delegate = FixtureSheetDelegate::new(show.clone(), fixtures.clone());
        let sheet = cx.new_view(|_cx| Sheet::new(sheet_delegate, "fixture_sheet"));

        Self { sheet }
    }
}

impl WindowDelegate for FixtureSheetWindowDelegate {
    fn title(&self) -> String {
        "Fixture Sheet".to_string()
    }

    fn render_content(&self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.sheet.clone())
    }
}

pub struct FixtureSheetDelegate {
    show: Model<Show>,
    fixtures: Vec<Fixture>,
}

impl FixtureSheetDelegate {
    pub fn new(show: Model<Show>, fixtures: Vec<Fixture>) -> Self {
        Self { show, fixtures }
    }
}

impl SheetDelegate for FixtureSheetDelegate {
    type Data = Fixture;
    type ColumnId = FixtureSheetColumnId;

    fn columns(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<Self::ColumnId> {
        let mut labels = vec![
            FixtureSheetColumnId::Id,
            FixtureSheetColumnId::Name,
            FixtureSheetColumnId::Patch,
            FixtureSheetColumnId::FixtureType,
            FixtureSheetColumnId::Mode,
        ];

        // FIXME: We probably should only show useful attributes.
        let mut attribute_labels = self
            .show
            .read(cx)
            .all_attributes()
            .iter()
            .map(|a| FixtureSheetColumnId::Attribute(a.name.clone()))
            .unique()
            .collect();

        labels.append(&mut attribute_labels);

        labels
    }

    fn values(&self, _cx: &mut ViewContext<Sheet<Self>>) -> &Vec<Self::Data> {
        &self.fixtures
    }

    fn selected_rows(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<usize> {
        let selected_fixtures = self.show.read(cx).selected_fixtures();
        self.fixtures
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if selected_fixtures.iter().any(|selected| selected.id == f.id) {
                    return Some(i);
                }
                None
            })
            .collect()
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
            FixtureSheetColumnId::FixtureType => 50.0,
            FixtureSheetColumnId::Mode => 50.0,
            FixtureSheetColumnId::Attribute(_) => 50.0,
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
            FixtureSheetColumnId::FixtureType => "Fixture Type".to_string(),
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
            FixtureSheetColumnId::Id => render_value(Some(fixture.id)),
            FixtureSheetColumnId::Name => render_value(Some(fixture.label.clone())),
            FixtureSheetColumnId::Patch => render_value(Some(fixture.channel.clone())),
            FixtureSheetColumnId::FixtureType => {
                render_value(Some(fixture.description.fixture_type.name.clone()))
            }
            FixtureSheetColumnId::Mode => {
                let name = fixture.current_dmx_mode().name.clone();
                render_value(Some(name))
            }
            FixtureSheetColumnId::Attribute(name) => {
                let Some(channels) = fixture.dmx_channels_for_attribute(name) else {
                    return div().into_any_element();
                };

                let values_string = self.show.update(cx, |show, _cx| {
                    let values = channels
                        .iter()
                        .map(
                            |channel| match show.stage_output_dmx_value_for_channel(*channel) {
                                Some(values) => values,
                                None => todo!(),
                            },
                        )
                        .collect::<Vec<_>>();

                    values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                });

                div().child(values_string).into_any_element()
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
    FixtureType,
    Mode,
    Attribute(String),
}
