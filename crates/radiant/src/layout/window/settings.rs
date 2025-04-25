use super::DEFAULT_REM_SIZE;
use crate::app::APP_ID;
use anyhow::Context as _;
use gpui::*;
use show::{Show, dmx_io::SacnSourceSettings};
use ui::{ActiveTheme as _, Field, FieldEvent, FieldImpl, TabView, root};

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

#[derive(Default)]
struct UniverseIdList(Vec<dmx::UniverseId>);

impl FieldImpl for UniverseIdList {
    type Value = Self;

    fn to_shared_string(value: &Self::Value) -> SharedString {
        value.0.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",").into()
    }

    fn from_str_or_default(s: &str) -> Self {
        let universe_ids =
            s.split(',').filter_map(|s| s.trim().parse::<dmx::UniverseId>().ok()).collect();
        Self(universe_ids)
    }
}

struct SacnSourceSettingsView {
    source: Entity<SacnSourceSettings>,

    name_field: Entity<Field<String>>,
    local_universes_field: Entity<Field<UniverseIdList>>,
    // destination_universe_field: Entity<NumberField>,
    // priority_field: Entity<NumberField>,
    // preview_data_checkbox: Entity<Checkbox>,
}

impl SacnSourceSettingsView {
    pub fn new(
        source: Entity<SacnSourceSettings>,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = Field::new(
                ElementId::NamedInteger("name".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_placeholder("Name", cx);
            field.set_value(&source.read(cx).name.clone(), cx);
            field
        });

        cx.subscribe(&name_field, |this, _, event: &FieldEvent<String>, cx| match event {
            FieldEvent::Submit(new_name) => {
                this.source.update(cx, |source, cx| {
                    source.name = new_name.clone();
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        let local_universes_field = cx.new(|cx| {
            let field = Field::<UniverseIdList>::new(
                ElementId::NamedInteger("local_universes".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_placeholder("Local Universes (e.g. '1 2 3')", cx);
            let universe_ids = source
                .read(cx)
                .local_universes
                .iter()
                .filter_map(|u| match dmx::UniverseId::new(*u) {
                    Ok(universe_id) => Some(universe_id),
                    Err(err) => {
                        log::warn!(
                            "Failed to parse UniverseId when generating settings field: {err}"
                        );
                        None
                    }
                })
                .collect::<Vec<_>>();
            field.set_value(&UniverseIdList(universe_ids), cx);
            field
        });

        cx.subscribe(&local_universes_field, |this, _, event, cx| match event {
            FieldEvent::Submit(new_local_universes) => {
                this.source.update(cx, |source, cx| {
                    source.local_universes =
                        new_local_universes.0.iter().map(|u| (*u).into()).collect();
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        // let destination_universe_field = cx.new(|cx| {
        //     NumberField::<dmx::UniverseId>::new(
        //         ElementId::NamedInteger("destination_universe".into(), ix),
        //         cx.focus_handle(),
        //         window,
        //         cx,
        //     )
        // });

        // let priority_field = cx.new(|cx| {
        //     let mut field = NumberField::new(
        //         ElementId::NamedInteger("priority".into(), ix),
        //         cx.focus_handle(),
        //         window,
        //         cx,
        //     );
        //     field.set_max(Some(200u8), cx);
        //     field
        // });

        // let preview_data_checkbox =
        //     cx.new(|_| Checkbox::new(ElementId::NamedInteger("preview_data".into(), ix)));

        Self {
            source,
            name_field,
            local_universes_field,
            // destination_universe_field,
            // priority_field,
            // preview_data_checkbox,
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
                // div().min_w_32().child(self.destination_universe_field.clone()),
                // div().min_w_32().child(self.priority_field.clone()),
                // div().min_w_32().child(self.preview_data_checkbox.clone()),
                // self.sacn_output_type_dropdown.clone().into_any_element(),
            ])
    }
}
