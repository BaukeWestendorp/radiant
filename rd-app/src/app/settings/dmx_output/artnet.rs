use rd_ui::{
    comp::stateful::{Table, TableColumn},
    gpui::{Entity, Window, div, prelude::*},
};

pub struct ArtnetOutputTabView {
    table: Entity<Table<rd::project::ArtnetOutputInstanceConfig>>,
}

impl ArtnetOutputTabView {
    pub fn new(
        uncommitted_project: Entity<rd::Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let instances = cx.new(|cx| uncommitted_project.read(cx).output.artnet.instances.clone());

        cx.observe(&uncommitted_project, {
            let instances = instances.clone();
            move |_, uncommitted_project, cx| {
                instances.write(cx, uncommitted_project.read(cx).output.artnet.instances.clone());
            }
        })
        .detach();

        cx.observe(&instances, {
            let uncommitted_project = uncommitted_project.clone();
            move |_, instances, cx| {
                let new_instances = instances.read(cx).clone();

                if new_instances == uncommitted_project.read(cx).output.artnet.instances {
                    return;
                }

                uncommitted_project.update(cx, |project, _| {
                    project.output.artnet.instances = new_instances;
                });
            }
        })
        .detach();

        let table = cx.new(|cx| {
            Table::new("artnet-output-instances", instances.clone(), window, cx).with_columns(vec![
                TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Name")
                    .with_element(|row, _, _| row.name.to_string().into_any_element()),
                TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Port Address")
                    .with_element(|row, _, _| row.port_address.to_string().into_any_element()),
                TableColumn::<rd::project::ArtnetOutputInstanceConfig>::new("Local Universe")
                    .with_element(|row, _, _| row.local_universe.to_string().into_any_element()),
            ])
        });

        Self { table }
    }
}

impl Render for ArtnetOutputTabView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}
