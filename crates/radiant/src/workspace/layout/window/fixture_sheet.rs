use gpui::{
    div, rgb, AnyElement, IntoElement, Model, ParentElement, Pixels, Styled, View, ViewContext,
    VisualContext, WindowContext,
};
use itertools::Itertools;

use crate::color;
use crate::show::patch::Fixture;
use crate::show::Show;
use crate::ui::sheet::{Sheet, SheetDelegate};

use super::{Window, WindowDelegate};

pub struct FixtureSheetWindowDelegate {
    sheet: View<Sheet<FixtureSheetDelegate>>,
}

impl FixtureSheetWindowDelegate {
    pub fn new(show: Model<Show>, cx: &mut WindowContext) -> Self {
        let fixtures = show.read(cx).clone().patch_list.fixtures;
        let sheet_delegate = FixtureSheetDelegate::new(show.clone(), fixtures);
        let sheet = cx.new_view(|_cx| Sheet::new(sheet_delegate, "fixture_sheet"));

        Self { sheet }
    }
}

impl WindowDelegate for FixtureSheetWindowDelegate {
    fn title(&self) -> String {
        "Fixture Sheet".to_string()
    }

    fn render_content(&self, _cx: &mut ViewContext<Window<Self>>) -> impl IntoElement {
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
            FixtureSheetColumnId::Mode,
        ];

        let all_used_attributes = self.show.read(cx).patch_list.all_used_attributes();
        let mut attribute_labels = all_used_attributes
            .iter()
            .map(|a| FixtureSheetColumnId::Attribute(a.name.clone()))
            .unique()
            .collect::<Vec<_>>();

        labels.append(&mut attribute_labels);

        labels
    }

    fn values(&self, _cx: &mut ViewContext<Sheet<Self>>) -> &Vec<Self::Data> {
        &self.fixtures
    }

    fn selected_rows(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<usize> {
        let selected_fixture_ids = self.show.read(cx).programmer.selection.clone();
        self.fixtures
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if let Some(fixture_id) = f.id {
                    if selected_fixture_ids.contains(&fixture_id) {
                        return Some(i);
                    }
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
            FixtureSheetColumnId::Patch => render_value(fixture.channel.clone()),
            FixtureSheetColumnId::Mode => {
                let patch_list = &self.show.read(cx).patch_list;
                let name = patch_list
                    .get_fixture_type(&fixture.fixture_type_id)
                    .dmx_modes[fixture.mode_index]
                    .name
                    .clone();

                render_value(Some(name))
            }
            FixtureSheetColumnId::Attribute(name) => {
                let values = match fixture.channel_value_for_attribute(name) {
                    Some(values) => values,
                    None => return render_value::<String>(None),
                };

                let values_string = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                div()
                    .bg(color::lighten(
                        gpui::green().into(),
                        values[0] as f32 / 255.0,
                    ))
                    .child(values_string)
                    .into_any_element()
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
