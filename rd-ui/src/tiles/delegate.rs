use gpui::{
    AnyElement, App, Bounds, ElementId, Entity, FontWeight, Pixels, SharedString, Window, div,
    relative,
};
use gpui::{Size, prelude::*};
use uuid::Uuid;

use crate::{ActiveTheme, HslaExt as _, h_flex};

pub trait TileDelegate {
    fn title(&self, cx: &App) -> SharedString;

    fn render_content(&self, bounds: Bounds<u32>, window: &mut Window, cx: &App) -> AnyElement;

    fn show_header(&self, _cx: &App) -> bool {
        true
    }
}

pub trait PoolTileDelegate {
    fn title(&self, cx: &App) -> SharedString;

    fn is_occupied(&self, slot: u32, cx: &App) -> bool;

    fn occupied_content(&self, slot: u32, cx: &App) -> impl IntoElement;

    fn on_activate_slot(&mut self, slot: u32, window: &mut Window, cx: &mut App);

    fn on_activate_empty_slot(&mut self, _slot: u32, _window: &mut Window, _cx: &mut App) {}

    fn slot_overlay_label(&self, slot: u32, _cx: &App) -> String {
        slot.to_string()
    }

    fn empty_slots_clickable(&self, _cx: &App) -> bool {
        false
    }

    fn element_id_prefix(&self, _cx: &App) -> &'static str {
        "pool_slot"
    }
}

pub struct PoolTile<D: PoolTileDelegate + 'static> {
    delegate: Entity<D>,
    cell_size: Size<Pixels>,
    show_header_cell: bool,
    id: Uuid,
}

impl<D: PoolTileDelegate + 'static> PoolTile<D> {
    pub fn new(delegate: Entity<D>, cell_size: Size<Pixels>) -> Self {
        Self { delegate, cell_size, show_header_cell: true, id: Uuid::new_v4() }
    }

    pub fn delegate(&self) -> Entity<D> {
        self.delegate.clone()
    }

    pub fn cell_size(&self) -> Size<Pixels> {
        self.cell_size
    }

    pub fn with_cell_size(mut self, cell_size: Size<Pixels>) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn show_header_cell(&self) -> bool {
        self.show_header_cell
    }

    pub fn with_show_header_cell(mut self, show_header_cell: bool) -> Self {
        self.show_header_cell = show_header_cell;
        self
    }
}

impl<D: PoolTileDelegate + 'static> TileDelegate for PoolTile<D> {
    fn title<'a>(&self, cx: &'a App) -> SharedString {
        self.delegate.read(cx).title(cx)
    }

    fn show_header(&self, _cx: &App) -> bool {
        false
    }

    fn render_content(&self, bounds: Bounds<u32>, _window: &mut Window, cx: &App) -> AnyElement {
        let slot_count = (bounds.size.width * bounds.size.height) as usize;

        let header_cell = div()
            .w(self.cell_size().width)
            .h(self.cell_size().height)
            .bg(cx.theme().bg_tile_header)
            .border_1()
            .border_color(cx.theme().border_tile_header)
            .rounded(cx.theme().radius)
            .text_color(cx.theme().fg_tile_header)
            .font_weight(FontWeight::BOLD)
            .child(
                h_flex()
                    .justify_center()
                    .h_full()
                    .child(div().w_full().text_center().child(self.delegate.read(cx).title(cx))),
            );

        let slot_cells = (0..slot_count).map(|ix| {
            let slot = (ix as u32) + 1;

            let occupied = self.delegate.read(cx).is_occupied(slot, cx);

            let id_overlay = div()
                .text_sm()
                .p_1()
                .line_height(relative(0.8))
                .absolute()
                .size_full()
                .text_color(cx.theme().fg_tertiary)
                .child(self.delegate.read(cx).slot_overlay_label(slot, cx));

            if occupied {
                let delegate = self.delegate.clone();

                div()
                    .id(ElementId::named_usize(
                        delegate.read(cx).element_id_prefix(cx),
                        slot as usize,
                    ))
                    .relative()
                    .w(self.cell_size.width)
                    .h(self.cell_size.height)
                    .bg(cx.theme().bg_secondary)
                    .border_1()
                    .border_color(cx.theme().border_secondary)
                    .rounded(cx.theme().radius)
                    .hover(|e| {
                        e.bg(cx.theme().bg_secondary.hover())
                            .border_color(cx.theme().border_secondary.hover())
                    })
                    .active(|e| {
                        e.bg(cx.theme().bg_secondary.active())
                            .border_color(cx.theme().border_secondary.active())
                            .top(cx.theme().button_depression)
                    })
                    .child(id_overlay)
                    .child(delegate.read(cx).occupied_content(slot, cx))
                    .on_click(move |_, window, cx| {
                        delegate.update(cx, |d, cx| d.on_activate_slot(slot, window, cx));
                    })
            } else {
                let delegate = self.delegate.clone();

                let base = div()
                    .id(ElementId::named_usize(
                        delegate.read(cx).element_id_prefix(cx),
                        slot as usize,
                    ))
                    .relative()
                    .w(self.cell_size.width)
                    .h(self.cell_size.height)
                    .bg(cx.theme().bg_primary)
                    .border_1()
                    .border_color(cx.theme().border_primary)
                    .rounded(cx.theme().radius)
                    .child(id_overlay)
                    .child(div().w(self.cell_size.width).h(self.cell_size.height));

                if delegate.read(cx).empty_slots_clickable(cx) {
                    base.on_click(move |_, window, cx| {
                        delegate.update(cx, |d, cx| d.on_activate_empty_slot(slot, window, cx));
                    })
                } else {
                    base
                }
            }
        });

        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .size_full()
            .when(self.show_header_cell(), |e| e.child(header_cell))
            .children(slot_cells)
            .into_any_element()
    }
}
