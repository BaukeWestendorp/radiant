use crate::app::{APP_ID, AppState};
use crate::layout::{GRID_SIZE, Page};
use crate::show::Show;
use crate::ui::FRAME_CELL_SIZE;
use anyhow::Context as _;
use gpui::{
    App, Bounds, ElementId, Entity, FocusHandle, Pixels, ReadGlobal, TitlebarOptions, UpdateGlobal,
    Window, WindowBounds, WindowHandle, WindowOptions, div, prelude::*, px, size,
};
use ui::{ActiveTheme, Disableable, interactive_container, root, utils::z_stack};

use super::DEFAULT_REM_SIZE;

const PAGE_LIST_WIDTH: Pixels = px(FRAME_CELL_SIZE.0 * 1.5);

pub struct MainWindow {
    page: Entity<Page>,

    focus_handle: FocusHandle,
}

impl MainWindow {
    pub fn open(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let min_size = size(
            FRAME_CELL_SIZE * GRID_SIZE.width as f32 + PAGE_LIST_WIDTH,
            FRAME_CELL_SIZE * GRID_SIZE.height as f32,
        );

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, min_size, cx))),
            window_min_size: Some(min_size),
            app_id: Some(APP_ID.to_string()),
            titlebar: Some(TitlebarOptions { title: Some("Radiant".into()), ..Default::default() }),
            ..Default::default()
        };

        cx.open_window(window_options, |w, cx| {
            w.set_rem_size(DEFAULT_REM_SIZE);

            cx.new(|cx| {
                let layout = Show::global(cx).layout.clone();

                let page = cx.new(|cx| {
                    cx.observe_in(&layout, w, |this, layout, w, cx| {
                        *this = Page::from_show(&layout, w, cx);
                        log::debug!("Updating FrameContainer<MainFrame>");
                        cx.notify();
                    })
                    .detach();

                    Page::from_show(&layout, w, cx)
                });

                cx.observe(&page, move |_, page, cx| {
                    layout.update(cx, |layout, cx| {
                        layout.main_window.loaded_page = page.read(cx).clone().into_show(cx);
                    });
                })
                .detach();

                Self { page, focus_handle: cx.focus_handle() }
            })
        })
        .context("open main window")
    }
}

impl MainWindow {
    fn handle_open_settings(
        &mut self,
        _: &actions::OpenSettings,
        w: &mut Window,
        cx: &mut Context<Self>,
    ) {
        AppState::update_global(cx, |state, cx| state.open_settings_window(w, cx));
        cx.notify();
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layout = &Show::global(cx).layout.read(cx);
        let pages = &layout.main_window.pages;
        let pages_list = div()
            .w(PAGE_LIST_WIDTH)
            .h(FRAME_CELL_SIZE * GRID_SIZE.height as f32)
            .flex()
            .flex_col()
            .flex_shrink_0()
            .occlude()
            .children((0..GRID_SIZE.height).map(|ix| {
                let id = ElementId::named_usize("page", ix as usize);
                match pages.get(ix as usize) {
                    Some(page) => interactive_container(id, None)
                        .w_full()
                        .h(FRAME_CELL_SIZE)
                        .child(page.label.clone())
                        .flex()
                        .justify_center()
                        .items_center()
                        .cursor_pointer()
                        .on_click(move |_, _, cx| handle_click_page(ix as usize, cx)),
                    None => {
                        interactive_container(id, None).w_full().h(FRAME_CELL_SIZE).disabled(true)
                    }
                }
            }));

        fn handle_click_page(ix: usize, cx: &mut App) {
            Show::global(cx).layout.clone().update(cx, |layout, cx| {
                let page = layout
                    .main_window
                    .pages
                    .get(ix)
                    .expect("Should have valid index as it was clicked")
                    .clone();
                layout.main_window.loaded_page = page;
                cx.notify();
            });
        }

        let main_layout = div()
            .flex()
            .size_full()
            .bg(cx.theme().colors.bg_primary)
            .text_color(cx.theme().colors.text)
            .child(self.page.clone())
            .child(pages_list);

        let settings_window = match AppState::global(cx).settings_window() {
            Some(window) => div().size_full().p_4().child(window.clone()),
            None => div(),
        };

        let preset_selector_window = match AppState::global(cx).preset_selector_window() {
            Some(window) => div().size_full().p_4().child(window.clone()),
            None => div(),
        };

        root(cx)
            .track_focus(&self.focus_handle)
            .key_context(actions::KEY_CONTEXT)
            .size_full()
            .on_action(cx.listener(Self::handle_open_settings))
            .child(z_stack([main_layout, settings_window, preset_selector_window]).size_full())
    }
}

pub mod actions {
    use gpui::*;

    pub const KEY_CONTEXT: &str = "MainWindow";

    actions!(main_window, [OpenSettings]);

    pub fn init(cx: &mut App) {
        bind_keys(cx);
    }

    fn bind_keys(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("secondary-,", OpenSettings, Some(KEY_CONTEXT))]);
    }
}
