use gpui::{
    App, DismissEvent, ElementId, Entity, MouseButton, MouseDownEvent, Pixels, Point, ScrollHandle,
    Subscription, Window, anchored, deferred, div, point, prelude::*, px,
};
use ui::{
    Checkbox, CheckboxEvent, ContainerStyle, ContextMenu, Disableable, Draggable, Field,
    NumberField, Pannable, Table, TableColumn, TableDelegate, TableRow, container,
};

gpui::actions!(interactive_tab, [Action1, Action2]);

pub struct InteractiveTab {
    scroll_handle: ScrollHandle,

    disable_fields_checkbox: Entity<Checkbox>,
    text_field: Entity<Field<String>>,
    masked_text_field: Entity<Field<String>>,
    f32_field: Entity<NumberField<f32>>,
    i8_field: Entity<NumberField<i8>>,
    checkbox: Entity<Checkbox>,
    checkbox_disabled: Entity<Checkbox>,

    table: Entity<Table<ExampleTable>>,

    draggable: Entity<Draggable>,
    pannable: Entity<Pannable>,

    context_menu: Option<(Entity<ContextMenu>, Point<Pixels>, Subscription)>,
}

impl InteractiveTab {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let disable_fields_checkbox = cx.new(|_| Checkbox::new("disable-fields-checkbox"));

        cx.subscribe(&disable_fields_checkbox, |this: &mut Self, _, event: &CheckboxEvent, cx| {
            match event {
                CheckboxEvent::Change(selected) => {
                    this.text_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.masked_text_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.f32_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                    this.i8_field.update(cx, |f, cx| f.set_disabled(*selected, cx));
                }
            }
        })
        .detach();

        Self {
            scroll_handle: ScrollHandle::new(),

            disable_fields_checkbox,
            text_field: cx.new(|cx| {
                let field = Field::new("text", cx.focus_handle(), window, cx);
                field.set_placeholder("Text Field Placeholder", cx);
                field
            }),
            masked_text_field: cx.new(|cx| {
                let field = Field::new("masked-text", cx.focus_handle(), window, cx);
                field.set_placeholder("Masked Text Field Placeholder", cx);
                field.set_masked(true, cx);
                field
            }),
            f32_field: cx.new(|cx| NumberField::new("f32-num", cx.focus_handle(), window, cx)),
            i8_field: cx.new(|cx| NumberField::new("i8", cx.focus_handle(), window, cx)),
            checkbox: cx.new(|_| Checkbox::new("checkbox")),
            checkbox_disabled: cx.new(|_| Checkbox::new("checkbox-disabled").disabled(true)),

            table: cx.new(|cx| Table::new(ExampleTable::new(), "table", window, cx)),

            draggable: cx.new(|cx| {
                Draggable::new("draggable", point(px(40.0), px(40.0)), None, cx.new(|_| ExampleBox))
            }),
            pannable: cx.new(|cx| {
                Pannable::new("pannable", point(px(40.0), px(40.0)), cx.new(|_| ExampleBox))
            }),

            context_menu: None,
        }
    }

    fn deploy_context_menu(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let context_menu = cx.new(|cx| {
            ContextMenu::new(window, cx)
                .action("Action 1", Box::new(Action1))
                .divider()
                .destructive_action("Action 2", Box::new(Action2))
        });

        let subscription = cx.subscribe(&context_menu, |this, _, _: &DismissEvent, cx| {
            this.context_menu.take();
            cx.notify();
        });

        self.context_menu = Some((context_menu, position, subscription));
    }
}

impl Render for InteractiveTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row = |label, input| {
            div()
                .w_full()
                .flex()
                .gap_2()
                .child(div().child(label).w_40())
                .child(div().child(input).w_full())
        };

        let inputs = div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(row("Disable Fields", self.disable_fields_checkbox.clone().into_any_element()))
            .child(row("Text", self.text_field.clone().into_any_element()))
            .child(row("Masked Text", self.masked_text_field.clone().into_any_element()))
            .child(row("f32 Number", self.f32_field.clone().into_any_element()))
            .child(row("i8 Number", self.i8_field.clone().into_any_element()));

        let checkboxes = div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(row("Checkbox", self.checkbox.clone().into_any_element()))
            .child(row("Disabled", self.checkbox_disabled.clone().into_any_element()));

        let context_menu = container(ContainerStyle::normal(window, cx))
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, w, cx| {
                    this.deploy_context_menu(event.position, w, cx);
                }),
            )
            .on_action(|_: &Action1, _window: &mut Window, _cx: &mut App| {
                println!("Action 1 dispatched!");
            })
            .on_action(|_: &Action2, _window: &mut Window, _cx: &mut App| {
                println!("Action 2 dispatched!");
            })
            .child("Click me for a context menu")
            .children(self.context_menu.as_ref().map(|(menu, position, _)| {
                deferred(
                    anchored()
                        .position(*position)
                        .anchor(gpui::Corner::BottomLeft)
                        .child(menu.clone()),
                )
                .with_priority(1)
            }));

        let table = div().h_64().child(self.table.clone());

        let draggable = container(ContainerStyle::normal(window, cx))
            .w_full()
            .h_64()
            .child(self.draggable.clone());

        let pannable = container(ContainerStyle::normal(window, cx))
            .w_full()
            .h_64()
            .child(self.pannable.clone());

        div()
            .id("typography_tab")
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .size_full()
            .p_2()
            .flex()
            .flex_col()
            .gap_2()
            .child(ui::section("Inputs").mb_4().child(inputs))
            .child(ui::section("Checkboxes").mb_4().child(checkboxes))
            .child(ui::section("Context Menu").mb_4().child(context_menu))
            .child(ui::section("Table").mb_4().child(table))
            .child(ui::section("Draggable").mb_4().child(draggable))
            .child(ui::section("Pannable").mb_4().child(pannable))
    }
}

struct ExampleBox;

impl Render for ExampleBox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        container(ContainerStyle::normal(window, cx)).size_20().child("Draggable Box")
    }
}

struct ExampleTable {
    rows: Vec<ExampleRow>,
}

impl ExampleTable {
    pub fn new() -> Self {
        Self {
            rows: vec![
                ExampleRow { id: 1.into(), text: "Text 1".to_string(), state: false, value: 42.25 },
                ExampleRow { id: 2.into(), text: "Text 2".to_string(), state: true, value: 3.14 },
                ExampleRow { id: 3.into(), text: "Text 3".to_string(), state: false, value: 1.618 },
            ],
        }
    }
}

impl TableDelegate for ExampleTable {
    type Row = ExampleRow;
    type Column = ExampleColumn;

    fn rows(&mut self, _cx: &mut App) -> Vec<Self::Row> {
        self.rows.clone()
    }
}

#[derive(Clone)]
struct ExampleRow {
    id: ElementId,
    text: String,
    state: bool,
    value: f32,
}

impl TableRow<ExampleTable> for ExampleRow {
    fn id(&self, _cx: &mut Context<Table<ExampleTable>>) -> gpui::ElementId {
        self.id.clone()
    }

    fn render_cell(
        &self,
        column: &ExampleColumn,
        _window: &mut Window,
        _cx: &mut Context<Table<ExampleTable>>,
    ) -> impl IntoElement {
        match column {
            ExampleColumn::Text => div().child(self.text.clone()),
            ExampleColumn::State => div().child(self.state.to_string()),
            ExampleColumn::Value => div().child(self.value.to_string()),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum ExampleColumn {
    Text,
    State,
    Value,
}

impl TableColumn for ExampleColumn {
    fn label(&self) -> &str {
        match self {
            ExampleColumn::Text => "Text",
            ExampleColumn::State => "State",
            ExampleColumn::Value => "Value",
        }
    }

    fn all<'a>() -> &'a [Self] {
        &[Self::Text, Self::State, Self::Value]
    }
}
