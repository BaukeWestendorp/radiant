use super::{VirtualWindow, VirtualWindowDelegate, main::MainWindow};
use gpui::*;
use show::{Show, dmx_io::SacnSourceSettings};
use ui::{
    ActiveTheme as _, Checkbox, CheckboxEvent, Field, FieldEvent, FieldImpl, NumberField, TabView,
};

pub struct SettingsWindow {
    main_window: Entity<MainWindow>,
    tab_view: Entity<TabView>,
}

impl SettingsWindow {
    pub fn new(
        main_window: Entity<MainWindow>,
        w: &mut Window,
        cx: &mut Context<VirtualWindow<Self>>,
    ) -> Self {
        let tabs = vec![
            ui::Tab::new("dmx_io", "Dmx Io", cx.new(|cx| DmxIoView::new(w, cx)).into()),
            ui::Tab::new("patch", "Patch", cx.new(|_| EmptyView).into()),
        ];

        let tab_view = cx.new(|cx| {
            let mut tab_view = TabView::new(tabs, w, cx);
            tab_view.select_tab_ix(0);
            tab_view
        });

        Self { main_window, tab_view }
    }
}

impl VirtualWindowDelegate for SettingsWindow {
    fn title(&self, _cx: &App) -> &str {
        "Settings"
    }

    fn on_close_window(&mut self, _w: &mut Window, cx: &mut Context<VirtualWindow<Self>>) {
        self.main_window.update(cx, |main_window, cx| {
            main_window.close_settings_window(cx);
        })
    }

    fn render_content(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
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

    name_field: Entity<Field<String>>,
    local_universes_field: Entity<Field<UniverseIdList>>,
    destination_universe_field: Entity<Field<UniverseIdFieldImpl>>,
    priority_field: Entity<NumberField<u8>>,
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

        let destination_universe_field = cx.new(|cx| {
            let field = Field::<UniverseIdFieldImpl>::new(
                ElementId::NamedInteger("destination_universe".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_placeholder("Destination Universe Id", cx);
            let value = dmx::UniverseId::new(source.read(cx).destination_universe)
                .expect("should get a valid universe id");
            field.set_value(&value, cx);
            field
        });

        cx.subscribe(&destination_universe_field, |this, _, event, cx| match event {
            FieldEvent::Submit(new_destination_universe) => {
                this.source.update(cx, |source, cx| {
                    source.destination_universe = (*new_destination_universe).into();
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        let priority_field = cx.new(|cx| {
            let mut field = NumberField::new(
                ElementId::NamedInteger("priority".into(), ix),
                cx.focus_handle(),
                window,
                cx,
            );
            field.set_min(Some(0u8), cx);
            field.set_max(Some(200u8), cx);
            field.set_step(Some(1.0), cx);
            field.set_value(source.read(cx).priority, cx);
            field
        });

        cx.subscribe(&priority_field, |this, _, event, cx| match event {
            FieldEvent::Submit(new_priority) => {
                this.source.update(cx, |source, cx| {
                    source.priority = *new_priority;
                    cx.notify();
                });
            }
            _ => {}
        })
        .detach();

        let preview_data_checkbox =
            cx.new(|_| Checkbox::new(ElementId::NamedInteger("preview_data".into(), ix)));

        cx.subscribe(&preview_data_checkbox, |this, _, event, cx| match event {
            CheckboxEvent::Changed(new_preview_data) => {
                this.source.update(cx, |source, cx| {
                    source.preview_data = *new_preview_data;
                    cx.notify();
                });
            }
        })
        .detach();

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
            ])
    }
}

pub struct UniverseIdFieldImpl;

impl FieldImpl for UniverseIdFieldImpl {
    type Value = dmx::UniverseId;

    fn from_str_or_default(s: &str) -> Self::Value {
        let f64_value = s
            .parse::<f64>()
            .unwrap_or_default()
            .clamp(u16::from(dmx::UniverseId::MIN) as f64, u16::from(dmx::UniverseId::MAX) as f64);

        dmx::UniverseId::new(f64_value as u16).unwrap_or_default()
    }

    fn to_shared_string(value: &Self::Value) -> SharedString {
        value.to_string().into()
    }
}

#[derive(Default)]
struct UniverseIdList(Vec<dmx::UniverseId>);

impl std::fmt::Display for UniverseIdList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "{s}")
    }
}

impl std::str::FromStr for UniverseIdList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let universe_ids =
            s.split(',').filter_map(|s| s.trim().parse::<dmx::UniverseId>().ok()).collect();
        Ok(Self(universe_ids))
    }
}
