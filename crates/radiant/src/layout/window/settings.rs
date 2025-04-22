use crate::app::APP_ID;
use anyhow::Context as _;
use gpui::*;
use show::{Show, dmx_io::SacnSourceSettings};
use ui::{ActiveTheme as _, DmxUniverseIdField, NumberField, TextField, ToggleButton};

use super::DEFAULT_REM_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Patch,
    DmxIo,
}

impl Tab {
    pub fn label(&self) -> &str {
        match self {
            Tab::Patch => "Patch",
            Tab::DmxIo => "DMX IO",
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Tab::Patch => "patch",
            Tab::DmxIo => "dmx_io",
        }
    }
}

pub struct SettingsWindow {
    active_tab: Tab,
    dmx_io_view: Entity<DmxIoView>,
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

        cx.open_window(window_options, |window, cx| {
            window.set_rem_size(DEFAULT_REM_SIZE);
            cx.new(|cx| Self {
                active_tab: Tab::DmxIo,
                dmx_io_view: cx.new(|cx| DmxIoView::new(window, cx)),
            })
        })
        .context("open settings window")
    }

    fn render_sidebar(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const TABS: [Tab; 2] = [Tab::Patch, Tab::DmxIo];

        div()
            .children(TABS.iter().map(|tab| {
                let id = ElementId::Name(format!("settings-tab-{}", tab.id()).into());
                ToggleButton::new(id)
                    .toggled(self.active_tab == *tab)
                    .on_click(cx.listener(|view, _, _window, _cx| view.active_tab = *tab))
                    .child(tab.label())
            }))
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .border_r_1()
            .border_color(cx.theme().border)
            .w_56()
            .h_full()
    }

    fn render_content(&self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match self.active_tab {
            Tab::Patch => div().child("Patch View").into_any_element(),
            Tab::DmxIo => self.dmx_io_view.clone().into_any_element(),
        })
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(cx.theme().background)
            .text_color(cx.theme().text_primary)
            .child(self.render_sidebar(window, cx))
            .child(self.render_content(window, cx))
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
            .map(|s| cx.new(|cx| SacnSourceSettingsView::new(s, window, cx)))
            .collect();

        Self { sacn_source_views }
    }
}

impl Render for DmxIoView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().m_2().children(self.sacn_source_views.clone()).size_full()
    }
}

struct SacnSourceSettingsView {
    source: Entity<SacnSourceSettings>,
    name_field: Entity<TextField>,
    local_universes_field: Entity<TextField>,
    destination_universe_field: Entity<DmxUniverseIdField>,
    priority_field: Entity<NumberField>,
}

impl SacnSourceSettingsView {
    pub fn new(
        source: Entity<SacnSourceSettings>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = TextField::new("name", cx.focus_handle(), window, cx);
            field.set_placeholder("Source Name".into(), cx);
            field
        });

        let local_universes_field = cx.new(|cx| {
            let field = TextField::new("local_universes", cx.focus_handle(), window, cx);
            field.set_placeholder("Local Universes".into(), cx);
            field
        });
        let destination_universe_field = cx.new(|cx| {
            DmxUniverseIdField::new("destination_universe", cx.focus_handle(), window, cx)
        });
        let priority_field = cx.new(|cx| {
            let mut field = NumberField::new("priority", cx.focus_handle(), window, cx);
            field.set_value(100.0, cx);
            field.set_min(Some(0.0));
            field.set_max(Some(200.0));
            field.set_step(Some(1.0));
            field
        });

        Self {
            source,
            name_field,
            local_universes_field,
            destination_universe_field,
            priority_field,
        }
    }
}

impl Render for SacnSourceSettingsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_data_toggle = ToggleButton::new("preview_data")
            .toggled(self.source.read(cx).preview_data)
            .on_click(cx.listener(|view, _event, _window, cx| {
                view.source.update(cx, |source, _cx| {
                    source.preview_data = !source.preview_data;
                });
            }))
            .child("Preview Data");

        div().w_full().flex().gap_2().border_b_1().border_color(cx.theme().border).p_2().children([
            div().min_w_24().child(self.name_field.clone()),
            div().min_w_24().child(self.local_universes_field.clone()),
            div().min_w_24().child(self.destination_universe_field.clone()),
            div().min_w_24().child(self.priority_field.clone()),
            div().min_w_24().child(preview_data_toggle),
            // self.sacn_output_type_dropdown.clone().into_any_element(),
        ])
    }
}
