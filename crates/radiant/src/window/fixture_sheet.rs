use backstage::command::{Command, Object};
use backstage::show::Fixture;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, rgb, AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels,
    SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::sheet::{Sheet, SheetDelegate};

use super::{WindowDelegate, WindowView};
use crate::showfile::ShowfileManager;

pub struct FixtureSheetWindowDelegate {
    sheet: View<Sheet<FixtureSheetDelegate>>,
}

impl FixtureSheetWindowDelegate {
    pub fn new(cx: &mut WindowContext) -> Self {
        let fixtures = ShowfileManager::show(cx).fixtures();
        let sheet_delegate = FixtureSheetDelegate::new(fixtures.to_vec());
        let sheet = cx.new_view(|_cx| Sheet::new(sheet_delegate, "fixture_sheet"));

        Self { sheet }
    }
}

impl WindowDelegate for FixtureSheetWindowDelegate {
    fn title(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> Option<SharedString> {
        Some("Fixture Sheet".into())
    }

    fn render_content(&mut self, _cx: &mut ViewContext<WindowView<Self>>) -> impl IntoElement {
        div().size_full().child(self.sheet.clone())
    }
}

pub struct FixtureSheetDelegate {
    fixtures: Vec<Fixture>,
}

impl FixtureSheetDelegate {
    pub fn new(fixtures: Vec<Fixture>) -> Self {
        Self { fixtures }
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
        let mut attribute_labels = ShowfileManager::show(cx)
            .all_attributes()
            .iter()
            .map(|a| FixtureSheetColumnId::Attribute(a.name.clone()))
            .unique()
            .collect();

        labels.append(&mut attribute_labels);

        labels
    }

    fn values(&self, _cx: &mut ViewContext<Sheet<Self>>) -> &[Self::Data] {
        &self.fixtures
    }

    fn selected_rows(&self, cx: &mut ViewContext<Sheet<Self>>) -> Vec<usize> {
        let selected_fixtures = ShowfileManager::show(cx).selected_fixtures();
        self.fixtures
            .iter()
            .enumerate()
            .filter_map(|(i, f)| {
                if selected_fixtures.iter().any(|id| *id == f.id) {
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
            FixtureSheetColumnId::FixtureType => 150.0,
            FixtureSheetColumnId::Mode => 150.0,
            FixtureSheetColumnId::Attribute(_) => 75.0,
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

    fn render_header_cell(
        &self,
        column_id: &Self::ColumnId,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        self.render_cell(
            column_id,
            div().px_1().child(self.header_label(column_id, cx)),
            cx,
        )
    }

    fn render_cell_content(
        &self,
        column_id: &Self::ColumnId,
        fixture: &Self::Data,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> AnyElement {
        let mut background_color = None;

        let cell = match column_id {
            FixtureSheetColumnId::Id => render_value(Some(fixture.id)),
            FixtureSheetColumnId::Name => render_value(Some(fixture.label.clone())),
            FixtureSheetColumnId::Patch => render_value(Some(fixture.channel)),
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

                let values = channels
                    .iter()
                    .map(|channel| {
                        ShowfileManager::show(cx)
                            .stage_output()
                            .borrow()
                            .channel(*channel)
                            .unwrap_or(0)
                    })
                    .collect::<Vec<_>>();

                let values_string = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                // FIXME: Reimplement this.
                let value_in_programmer = true;

                if value_in_programmer {
                    background_color = Some(cx.theme().colors().programmer_change);
                }

                div().child(values_string).into_any_element()
            }
        }
        .into_any_element();

        let fixture_id = fixture.id;
        div()
            .when_some(background_color, |this, bg| this.bg(bg))
            .px_1()
            .child(cell)
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                ShowfileManager::update(cx, |showfile, _cx| {
                    if let Err(err) = showfile
                        .show
                        .execute_command(&Command::Select(Some(Object::Fixture(Some(fixture_id)))))
                    {
                        log::error!("Failed to select fixture: {:?}", err);
                    }
                })
            })
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
