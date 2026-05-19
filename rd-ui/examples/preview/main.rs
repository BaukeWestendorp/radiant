mod interactive;
mod misc;
mod scrollable;
mod table;
mod tabs;
mod theme;
mod tiles;
mod typo;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();
    app::run()?;
    Ok(())
}

mod app {
    use gpui::prelude::*;
    use gpui::{Entity, Window, div};
    use rd_ui::{ConfigAppExt as _, Tab, Tabs, TabsState, TabsVariant};

    use crate::interactive::InteractivePreview;
    use crate::misc::MiscPreview;
    use crate::scrollable::ScrollablePreview;
    use crate::table::TablePreview;
    use crate::tabs::TabsPreview;
    use crate::theme::ThemePreview;
    use crate::tiles::TilesPreview;
    use crate::typo::TypoPreview;

    pub fn run() -> anyhow::Result<()> {
        rd_ui::build_simple_app()
            .window_title("MaakUI Preview")
            .config(
                rd_ui::config::Config::builder()
                    .add_source(rd_ui::config::File::from(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join("examples")
                            .join("preview")
                            .join("config.toml"),
                    ))
                    .build()?,
            )
            .run(|window, cx| cx.new(|cx| PreviewApp::new(window, cx)));

        Ok(())
    }

    struct PreviewApp {
        tabs: Entity<TabsState>,

        tab_interactive: Entity<InteractivePreview>,
        tab_tabs: Entity<TabsPreview>,
        tab_table: Entity<TablePreview>,
        tab_scrollable: Entity<ScrollablePreview>,
        tab_theme: Entity<ThemePreview>,
        tab_tiles: Entity<TilesPreview>,
        tab_typo: Entity<TypoPreview>,

        tab_misc: Entity<MiscPreview>,
    }

    impl PreviewApp {
        fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
            Self {
                tabs: cx.new(|cx| {
                    let selected =
                        cx.config().get_string("current_tab").unwrap_or("button".to_string());
                    TabsState::new().with_selected(selected)
                }),

                tab_interactive: cx.new(|cx| InteractivePreview::new(window, cx)),
                tab_tabs: cx.new(|cx| TabsPreview::new(window, cx)),
                tab_table: cx.new(|cx| TablePreview::new(window, cx)),
                tab_scrollable: cx.new(|cx| ScrollablePreview::new(window, cx)),
                tab_theme: cx.new(|cx| ThemePreview::new(window, cx)),
                tab_tiles: cx.new(|cx| TilesPreview::new(window, cx)),
                tab_typo: cx.new(|cx| TypoPreview::new(window, cx)),
                tab_misc: cx.new(|cx| MiscPreview::new(window, cx)),
            }
        }
    }

    impl Render for PreviewApp {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                // FIXME: TabVariant::Sidebar fucks with the table width.
                Tabs::new("preview-pages", self.tabs.clone(), TabsVariant::Top).tabs([
                    Tab::new(
                        "interactive",
                        "Interactive",
                        self.tab_interactive.clone().into_any_element(),
                    ),
                    Tab::new("typo", "Typography", self.tab_typo.clone().into_any_element()),
                    Tab::new("theme", "Theme", self.tab_theme.clone().into_any_element()),
                    Tab::new("tabs", "Tabs", self.tab_tabs.clone().into_any_element()),
                    Tab::new("table", "Table", self.tab_table.clone().into_any_element()),
                    Tab::new(
                        "scrollable",
                        "Scrollable",
                        self.tab_scrollable.clone().into_any_element(),
                    ),
                    Tab::new("tiles", "Tiles", self.tab_tiles.clone().into_any_element()),
                    Tab::new("misc", "Miscellaneous", self.tab_misc.clone().into_any_element()),
                ]),
            )
        }
    }
}

pub fn alpha_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::red())
        .bg(gpui::red().opacity(0.2))
        .child("Alpha")
        .font_weight(gpui::FontWeight::BOLD)
}

pub fn beta_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::green())
        .bg(gpui::green().opacity(0.2))
        .child("Beta")
        .font_weight(gpui::FontWeight::BOLD)
}

pub fn gamma_content() -> gpui::Div {
    use gpui::{ParentElement as _, Styled as _};

    gpui::div()
        .size_full()
        .p_2()
        .flex()
        .justify_center()
        .items_center()
        .border_1()
        .border_color(gpui::blue())
        .bg(gpui::blue().opacity(0.2))
        .child("Gamma")
        .font_weight(gpui::FontWeight::BOLD)
}
