use crate::app::APP_ID;
use anyhow::Context as _;
use gpui::*;
use show::{Show, dmx_io::SacnSourceSettings};
use ui::{
    ActiveTheme as _, Checkbox, DmxUniverseIdField, NumberField, TabView, TextField,
    TextInputEvent, root,
};

use super::DEFAULT_REM_SIZE;

pub struct SettingsWindow {
    tab_view: Entity<TabView>,
}

impl SettingsWindow {
    pub fn open(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(800.0), px(600.0)),
                cx,
            ))),
            app_id: Some(APP_ID.to_string()),
            ..Default::default()
        };

        cx.open_window(window_options, |w, cx| {
            w.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|cx| {
                let tabs = vec![
                    ui::Tab::new("dmx_io", "Dmx Io", cx.new(|cx| DmxIoView::new(w, cx)).into()),
                    ui::Tab::new("patch", "Patch", cx.new(|_| EmptyView).into()),
                ];

                let tab_view = cx.new(|cx| {
                    let mut tab_view = TabView::new(tabs, w, cx);
                    tab_view.select_tab_ix(0);
                    tab_view
                });

                Self { tab_view }
            })
        })
        .context("open settings window")
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        root(cx).size_full().child(self.tab_view.clone())
    }
}

struct DmxIoView {
    sacn_source_views: Vec<Entity<SacnSourceSettingsView>>,
}

impl DmxIoView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let sacn_source_views = Show::global(cx)
            .dmx_io_settings
            .sacn
            .sources
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, s)| cx.new(|cx| SacnSourceSettingsView::new(s, ix, window, cx)))
            .collect();

        Self { sacn_source_views }
    }
}

impl Render for DmxIoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().children(self.sacn_source_views.clone()).size_full()
    }
}

struct SacnSourceSettingsView {
    source: Entity<SacnSourceSettings>,

    name_field: Entity<TextField>,
    local_universes_field: Entity<TextField>,
    destination_universe_field: Entity<DmxUniverseIdField>,
    priority_field: Entity<NumberField>,
    preview_data_checkbox: Entity<Checkbox>,
}

impl SacnSourceSettingsView {
    pub fn new(
        source: Entity<SacnSourceSettings>,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = TextField::new(
                ElementId::NamedInteger("name".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_placeholder("Name".into(), cx);
            field.set_value(source.read(cx).name.clone().into(), cx);
            field
        });

        cx.subscribe(&name_field, |this, _, event, cx| match event {
            TextInputEvent::Submit(new_name) => {
                this.source.update(cx, |source, cx| {
                    source.name = new_name.to_string();
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        let local_universes_field = cx.new(|cx| {
            let field = TextField::new(
                ElementId::NamedInteger("local_universes".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_placeholder("Local Universes".into(), cx);
            field
        });

        let destination_universe_field = cx.new(|cx| {
            DmxUniverseIdField::new(
                ElementId::NamedInteger("destination_universe".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            )
        });

        let priority_field = cx.new(|cx| {
            let mut field = NumberField::new(
                ElementId::NamedInteger("priority".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_value(100.0, cx);
            field.set_min(Some(0.0));
            field.set_max(Some(200.0));
            field.set_step(Some(1.0));
            field
        });

        let preview_data_checkbox =
            cx.new(|_| Checkbox::new(ElementId::NamedInteger("preview_data".into(), ix)));

        Self {
            source,
            name_field,
            local_universes_field,
            destination_universe_field,
            priority_field,
            preview_data_checkbox,
        }
    }
}

impl Render for SacnSourceSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .items_center()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().colors.border)
            .p_2()
            .children([
                div().min_w_32().child(self.name_field.clone()),
                div().min_w_32().child(self.local_universes_field.clone()),
                div().min_w_32().child(self.destination_universe_field.clone()),
                div().min_w_32().child(self.priority_field.clone()),
                div().min_w_32().child(self.preview_data_checkbox.clone()),
                // self.sacn_output_type_dropdown.clone().into_any_element(),
            ])
    }
}
