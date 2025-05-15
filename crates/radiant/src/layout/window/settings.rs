use crate::app::AppState;
use crate::show::{Show, dmx_io::SacnSourceSettings};
use crate::ui::vw::{VirtualWindow, VirtualWindowDelegate};
use gpui::*;
use ui::{
    Checkbox, CheckboxEvent, ContainerStyle, Field, FieldEvent, FieldImpl, NumberField, TabView,
    Table, TableColumn, TableDelegate, TableRow, container, section,
};

pub struct SettingsWindow {
    tab_view: Entity<TabView>,
}

impl SettingsWindow {
    pub fn new(w: &mut Window, cx: &mut Context<VirtualWindow<Self>>) -> Self {
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
    }
}

impl VirtualWindowDelegate for SettingsWindow {
    fn title(&self, _cx: &App) -> SharedString {
        "Settings".into()
    }

    fn on_close_window(&mut self, _w: &mut Window, cx: &mut Context<VirtualWindow<Self>>) {
        AppState::update_global(cx, |state, _cx| {
            state.close_settings_window();
        });
    }

    fn render_content(
        &mut self,
        _w: &mut Window,
        _cx: &mut Context<VirtualWindow<Self>>,
    ) -> impl IntoElement {
        div().size_full().child(self.tab_view.clone())
    }
}

impl Focusable for SettingsWindow {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        cx.focus_handle()
    }
}

struct DmxIoView {
    sacn_source_table: Entity<Table<SacnSourceTable>>,
}

impl DmxIoView {
    pub fn new(w: &mut Window, cx: &mut Context<Self>) -> Self {
        let sacn_source_table =
            cx.new(|cx| Table::new(SacnSourceTable::new(w, cx), "sacn-source-table", w, cx));

        Self { sacn_source_table }
    }
}

impl Render for DmxIoView {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_2()
            .child(
                section("sACN Inputs & Outputs")
                    .child(
                        container(ContainerStyle::normal(w, cx))
                            .size_full()
                            .child(self.sacn_source_table.clone()),
                    )
                    .h_1_2(),
            )
            .size_full()
    }
}

struct SacnSourceTable {
    rows: Vec<SacnSourceTableRow>,
}

impl SacnSourceTable {
    pub fn new(w: &mut Window, cx: &mut App) -> Self {
        let rows = Show::global(cx)
            .dmx_io_settings
            .sacn
            .sources
            .clone()
            .into_iter()
            .enumerate()
            .map(|(ix, s)| SacnSourceTableRow::new(s, ix, w, cx))
            .collect();

        Self { rows }
    }
}

impl TableDelegate for SacnSourceTable {
    type Row = SacnSourceTableRow;

    type Column = SacnSourceTableColumn;

    fn rows(&mut self, _cx: &mut App) -> Vec<Self::Row> {
        self.rows.clone()
    }
}

#[derive(Clone)]
struct SacnSourceTableRow {
    id: ElementId,
    name_field: Entity<Field<String>>,
    local_universes_field: Entity<Field<UniverseIdList>>,
    destination_universe_field: Entity<Field<UniverseIdFieldImpl>>,
    priority_field: Entity<NumberField<u8>>,
    preview_data_checkbox: Entity<Checkbox>,
}

impl SacnSourceTableRow {
    pub fn new(
        source: Entity<SacnSourceSettings>,
        ix: usize,
        w: &mut Window,
        cx: &mut App,
    ) -> Self {
        let name_field = cx.new(|cx| {
            let field = Field::new(
                ElementId::NamedInteger("name".into(), ix as u64),
                cx.focus_handle(),
                w,
                cx,
            );
            field.set_placeholder("Name", cx);
            field.set_value(&source.read(cx).name.clone(), cx);
            field
        });

        cx.subscribe(&name_field, {
            let source = source.clone();
            move |_, event: &FieldEvent<String>, cx| match event {
                FieldEvent::Submit(new_name) => {
                    source.update(cx, |source, cx| {
                        source.name = new_name.clone();
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        let local_universes_field = cx.new(|cx| {
            let field = Field::<UniverseIdList>::new(
                ElementId::NamedInteger("local_universes".into(), ix as u64),
                cx.focus_handle(),
                w,
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

        cx.subscribe(&local_universes_field, {
            let source = source.clone();
            move |_, event, cx| match event {
                FieldEvent::Submit(new_local_universes) => {
                    source.update(cx, |source, cx| {
                        source.local_universes =
                            new_local_universes.0.iter().map(|u| (*u).into()).collect();
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        let destination_universe_field = cx.new(|cx| {
            let field = Field::<UniverseIdFieldImpl>::new(
                ElementId::NamedInteger("destination_universe".into(), ix as u64),
                cx.focus_handle(),
                w,
                cx,
            );
            field.set_placeholder("Destination Universe Id", cx);
            let value = dmx::UniverseId::new(source.read(cx).destination_universe)
                .expect("should get a valid universe id");
            field.set_value(&value, cx);
            field
        });

        cx.subscribe(&destination_universe_field, {
            let source = source.clone();
            move |_, event, cx| match event {
                FieldEvent::Submit(new_destination_universe) => {
                    source.update(cx, |source, cx| {
                        source.destination_universe = (*new_destination_universe).into();
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        let priority_field = cx.new(|cx| {
            let mut field = NumberField::new(
                ElementId::NamedInteger("priority".into(), ix as u64),
                cx.focus_handle(),
                w,
                cx,
            );
            field.set_min(Some(0u8), cx);
            field.set_max(Some(200u8), cx);
            field.set_step(Some(1.0), cx);
            field.set_value(source.read(cx).priority, cx);
            field
        });

        cx.subscribe(&priority_field, {
            let source = source.clone();
            move |_, event, cx| match event {
                FieldEvent::Submit(new_priority) => {
                    source.update(cx, |source, cx| {
                        source.priority = *new_priority;
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        let preview_data_checkbox =
            cx.new(|_| Checkbox::new(ElementId::NamedInteger("preview_data".into(), ix as u64)));

        cx.subscribe(&preview_data_checkbox, {
            let source = source.clone();
            move |_, event, cx| match event {
                CheckboxEvent::Changed(new_preview_data) => {
                    source.update(cx, |source, cx| {
                        source.preview_data = *new_preview_data;
                        cx.notify();
                    });
                }
            }
        })
        .detach();

        Self {
            id: ElementId::Integer(ix as u64),
            name_field,
            local_universes_field,
            destination_universe_field,
            priority_field,
            preview_data_checkbox,
        }
    }
}

impl TableRow<SacnSourceTable> for SacnSourceTableRow {
    fn id(&self, _cx: &mut Context<Table<SacnSourceTable>>) -> ElementId {
        self.id.clone()
    }

    fn render_cell(
        &self,
        column: &SacnSourceTableColumn,
        _w: &mut Window,
        _cx: &mut Context<Table<SacnSourceTable>>,
    ) -> impl IntoElement
    where
        Self: Sized,
    {
        match column {
            SacnSourceTableColumn::Name => {
                div().p_1().child(self.name_field.clone()).into_any_element()
            }
            SacnSourceTableColumn::LocalUniverses => {
                div().p_1().child(self.local_universes_field.clone()).into_any_element()
            }
            SacnSourceTableColumn::DestinationUniverse => {
                div().p_1().child(self.destination_universe_field.clone()).into_any_element()
            }
            SacnSourceTableColumn::Priority => {
                div().p_1().child(self.priority_field.clone()).into_any_element()
            }
            SacnSourceTableColumn::PreviewData => {
                div().p_1().child(self.preview_data_checkbox.clone()).into_any_element()
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum SacnSourceTableColumn {
    Name,
    LocalUniverses,
    DestinationUniverse,
    Priority,
    PreviewData,
}

impl TableColumn for SacnSourceTableColumn {
    fn label(&self) -> &str {
        match self {
            SacnSourceTableColumn::Name => "Name",
            SacnSourceTableColumn::LocalUniverses => "Local Universes",
            SacnSourceTableColumn::DestinationUniverse => "Destination Universe",
            SacnSourceTableColumn::Priority => "Priority",
            SacnSourceTableColumn::PreviewData => "Preview Data",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[
            Self::Name,
            Self::LocalUniverses,
            Self::DestinationUniverse,
            Self::Priority,
            Self::PreviewData,
        ]
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
