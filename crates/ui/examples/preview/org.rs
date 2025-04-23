use gpui::{Window, div, prelude::*};
use ui::container;

pub struct OrganizationTab {}

impl OrganizationTab {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for OrganizationTab {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let section =
            ui::section("Section Header", div().child("This is a section's content").mb_4(), cx);

        let divider = ui::divider(cx);

        let container = container("This is a container's content", cx);

        div()
            .p_2()
            .child(ui::section("Section", section, cx).mb_4())
            .child(ui::section("Divider", divider, cx).mb_4())
            .child(ui::section("Container", container, cx).mb_4())
    }
}
