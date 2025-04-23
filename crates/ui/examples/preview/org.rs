use gpui::{FocusHandle, Window, div, prelude::*};
use ui::{ContainerStyle, Disableable, Selectable, container, interactive_container};

pub struct OrganizationTab {
    c1_fh: FocusHandle,
    c2_fh: FocusHandle,
    c3_fh: FocusHandle,
    c4_fh: FocusHandle,
}

impl OrganizationTab {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            c1_fh: cx.focus_handle(),
            c2_fh: cx.focus_handle(),
            c3_fh: cx.focus_handle(),
            c4_fh: cx.focus_handle(),
        }
    }
}

impl Render for OrganizationTab {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let section =
            ui::section("Section Header").child(div().child("This is a section's content").mb_4());

        let divider = ui::divider(cx);

        let c_normal = container(ContainerStyle::normal(w, cx))
            .w_64()
            .child(div().m_2().child("This is a normal container's content"));
        let c_focused = container(ContainerStyle::focused(w, cx))
            .w_64()
            .child(div().m_2().child("This is a focused container's content"));
        let c_selected = container(ContainerStyle::selected(w, cx))
            .w_64()
            .child(div().m_2().child("This is a selected container's content"));
        let c_normal_disabled = container(ContainerStyle::normal(w, cx).disabled())
            .w_64()
            .child(div().m_2().child("This is a normal container's content"));
        let c_focused_disabled = container(ContainerStyle::focused(w, cx).disabled())
            .w_64()
            .child(div().m_2().child("This is a focused container's content"));
        let c_selected_disabled = container(ContainerStyle::selected(w, cx).disabled())
            .w_64()
            .child(div().m_2().child("This is a selected container's content"));

        let ic_1 = interactive_container("interactive_c_1", Some(self.c1_fh.clone()))
            .w_64()
            .child(div().m_2().child("This is an interactive container's content"));
        let ic_2 = interactive_container("interactive_c_2", Some(self.c2_fh.clone()))
            .w_64()
            .child(div().m_2().child("This is another interactive container's content"));
        let ic_selected = interactive_container("interactive_c_3", Some(self.c3_fh.clone()))
            .w_64()
            .child(div().m_2().child("This is a selected interactive container's content"))
            .selected(true);
        let ic_disabled = interactive_container("interactive_c_4", Some(self.c4_fh.clone()))
            .w_64()
            .child(div().m_2().child("This is a disabled interactive container's content"))
            .disabled(true);

        let noninteractive_containers = div()
            .flex()
            .flex_wrap()
            .gap_2()
            .child(ui::section("Normal Container").mb_4().child(c_normal))
            .child(ui::section("Focused Container").mb_4().child(c_focused))
            .child(ui::section("Selected Container").mb_4().child(c_selected))
            .child(ui::section("Normal Container Disabled").mb_4().child(c_normal_disabled))
            .child(ui::section("Focused Container Disabled").mb_4().child(c_focused_disabled))
            .child(ui::section("Selected Container Disabled").mb_4().child(c_selected_disabled));

        let interactive_containers = div()
            .flex()
            .flex_wrap()
            .gap_2()
            .child(ui::section("Interactive Container 1").mb_4().child(ic_1))
            .child(ui::section("Interactive Container 2").mb_4().child(ic_2))
            .child(ui::section("Interactive Container Selected").mb_4().child(ic_selected))
            .child(ui::section("Interactive Container Disabled").mb_4().child(ic_disabled));

        div()
            .p_2()
            .child(ui::section("Section").mb_4().child(section))
            .child(ui::section("Divider").mb_4().child(divider))
            .child(noninteractive_containers)
            .child(interactive_containers)
    }
}
