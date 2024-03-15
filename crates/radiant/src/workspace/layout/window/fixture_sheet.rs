use gpui::prelude::FluentBuilder;
use gpui::{
    div, rgb, AnyElement, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext,
    VisualContext,
};

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

    fn value_labels(&self) -> Vec<String> {
        vec!["ID".into(), "Name".into(), "Patch".into(), "Mode".into()]
    }

    fn values(&self) -> &Vec<Self::Data> {
        &self.fixtures
    }

    fn column_widths(&self) -> Vec<gpui::Pixels> {
        vec![
            gpui::px(50.0),
            gpui::px(200.0),
            gpui::px(80.0),
            gpui::px(200.0),
        ]
    }

    fn render_row_items(
        &self,
        fixture: &Self::Data,
        cx: &mut ViewContext<Sheet<Self>>,
    ) -> Vec<AnyElement> {
        let valid_channels = self.show.update(cx, |show, _cx| {
            fixture
                .get_valid_channels(&mut show.patch_list)
                .collect::<Vec<_>>()
                .len()
        });

        let id = match fixture.id {
            Some(id) => format!("{}", id),
            None => "None".to_string(),
        };

        let patch = match &fixture.patch {
            Some(patch) => patch.to_string(),
            None => "None".to_string(),
        };

        vec![
            self.render_cell(
                div()
                    .when(fixture.id.is_none(), |this| this.text_color(rgb(0x888888)))
                    .child(id),
                cx,
            ),
            self.render_cell(div().child(fixture.name.clone()), cx),
            self.render_cell(
                div()
                    .when(fixture.patch.is_none(), |this| {
                        this.text_color(rgb(0x888888))
                    })
                    .child(patch),
                cx,
            ),
            self.render_cell(
                div().child(format!(
                    "Mode {} ({} channels)",
                    fixture.mode_index + 1,
                    valid_channels
                )),
                cx,
            ),
        ]
    }
}
