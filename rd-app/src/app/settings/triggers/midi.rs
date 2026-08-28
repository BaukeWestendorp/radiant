use rd_ui::{
    comp::stateful::{Table, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

pub struct MidiTabView {
    table: Entity<Table<rd::project::MidiMapping>>,
}

impl MidiTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mappings = cx.new(|cx| uncommitted_project.read(cx).trigger.midi.clone());

        cx.observe(&uncommitted_project, {
            let mappings = mappings.clone();
            move |_, uncommitted_project, cx| {
                mappings.write(cx, uncommitted_project.read(cx).trigger.midi.clone());
            }
        })
        .detach();

        cx.observe(&mappings, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, mappings, cx| {
                let new_mappings = mappings.read(cx).clone();

                if new_mappings == uncommitted_project.read(cx).trigger.midi {
                    return;
                }

                uncommitted_project.update(cx, |project, _| {
                    project.trigger.midi = new_mappings;
                });
            }
        })
        .detach();

        let table = cx.new(|cx| {
            Table::new("midi-triggers", mappings.clone(), window, cx).with_columns(
                vec![
                    TableColumn::<rd::project::MidiMapping>::new("Device Name")
                        .with_element(|row, _, _| row.device_name.to_string().into_any_element()),
                    TableColumn::<rd::project::MidiMapping>::new("Device Channel").with_element(
                        |row, _, _| row.device_channel.to_string().into_any_element(),
                    ),
                    TableColumn::<rd::project::MidiMapping>::new("Filter")
                        .with_element(|row, _, _| row.filter.to_string().into_any_element()),
                    TableColumn::<rd::project::MidiMapping>::new("Target")
                        .with_element(|row, _, _| row.target.to_string().into_any_element()),
                ],
                cx,
            )
        });

        Self { table }
    }
}

impl Render for MidiTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}
