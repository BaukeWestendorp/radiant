use std::num::NonZeroU32;

use gpui::{
    AnyElement, App, Bounds, Entity, FontWeight, Pixels, SharedString, Size, Window, div,
    prelude::*, relative, uniform_list,
};

use rd_engine::{
    event::Event,
    object::{Executor, ExecutorContent, ExecutorPage, Object as _, ObjectKind, Sequence, Slot},
};
use rd_ui::{ActiveTheme, TileDelegate, h_flex, v_flex};

use crate::engine::EngineAppExt;

pub struct ExecutorsTile {
    // FIXME: Keep this in some settings per tile?
    selected_executor_page_slot: Slot,

    executor_page_view: Entity<ExecutorPageView>,
}

impl ExecutorsTile {
    pub fn new(
        bounds: Bounds<u32>,
        cell_size: Size<Pixels>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let selected_executor_page_slot = Slot::new(NonZeroU32::new(1).unwrap());

        let executor_page_view = cx.new(|cx| {
            let executor_page = cx
                .engine_snapshot()
                .objects()
                .executor_pages()
                .get_by_slot(&selected_executor_page_slot)
                .ok()
                .expect("TODO: Implement executors tile without selected page")
                .clone();
            ExecutorPageView::new(executor_page, bounds.size.width, cell_size, cx)
        });

        cx.on_engine_event({
            let executor_page_view = executor_page_view.clone();
            move |event, cx| match event {
                Event::ObjectChanged { object_kind: ObjectKind::ExecutorPage, .. } => {
                    let Ok(new_executor_page) = cx
                        .engine_snapshot()
                        .objects()
                        .executor_pages()
                        .get_by_slot(&selected_executor_page_slot)
                        .cloned()
                    else {
                        return;
                    };

                    executor_page_view.update(cx, |executor_page, cx| {
                        *executor_page = ExecutorPageView::new(
                            new_executor_page,
                            bounds.size.width,
                            cell_size,
                            cx,
                        );
                        cx.notify();
                    });
                }
                _ => {}
            }
        })
        .detach();

        Self { selected_executor_page_slot, executor_page_view }
    }
}

impl TileDelegate for ExecutorsTile {
    fn title(&self, _cx: &App) -> SharedString {
        format!("Executors Page {}", self.selected_executor_page_slot).into()
    }

    // FIXME: Can we pass cell_size to render_content?
    fn render_content(&self, _bounds: Bounds<u32>, _window: &mut Window, _cx: &App) -> AnyElement {
        self.executor_page_view.clone().into_any_element()
    }
}

struct ExecutorPageView {
    executor_page: ExecutorPage,
    width: u32,
    cell_size: Size<Pixels>,
}

impl ExecutorPageView {
    fn new(
        executor_page: ExecutorPage,
        width: u32,
        cell_size: Size<Pixels>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self { executor_page, width, cell_size }
    }
}

impl Render for ExecutorPageView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let executors =
            self.executor_page.executors().iter().enumerate().take(self.width as usize).map(
                |(ix, executor)| {
                    let executor_header = h_flex()
                        .h_6()
                        .justify_center()
                        .border_b_1()
                        .border_color(cx.theme().border_primary)
                        .p_px()
                        .child(
                            h_flex()
                                .justify_center()
                                .size_full()
                                .border_1()
                                .bg(cx.theme().bg_primary)
                                .when(executor.enabled(), |e| {
                                    e.border_color(cx.theme().contrast.opacity(0.25))
                                        .font_weight(FontWeight::BOLD)
                                        .bg(cx.theme().indicate.playback)
                                })
                                .child(format!("{}.{}", self.executor_page.slot(), ix + 1)),
                        );

                    let empty_executor =
                        div().size_full().bg(cx.theme().bg_secondary).into_any_element();

                    let executor_content = match executor.content() {
                        Some(ExecutorContent::Sequence(sc)) => {
                            match cx
                                .engine_snapshot()
                                .objects()
                                .sequences()
                                .get_by_object_id(&sc.sequence())
                            {
                                Ok(sequence) => render_sequence_content(
                                    executor,
                                    sequence,
                                    sc.cue_index(),
                                    window,
                                    cx,
                                )
                                .into_any_element(),
                                Err(err) => {
                                    log::error!("{err}");
                                    empty_executor
                                }
                            }
                        }
                        None => empty_executor,
                    };

                    v_flex()
                        .w(self.cell_size.width)
                        .h_full()
                        .bg(cx.theme().bg_primary)
                        .border_1()
                        .border_color(cx.theme().border_primary)
                        .rounded(cx.theme().radius)
                        .text_color(cx.theme().fg_primary)
                        .child(executor_header)
                        .child(executor_content)
                },
            );

        div().size_full().flex().text_sm().children(executors).into_any_element()
    }
}

fn render_sequence_content(
    executor: &Executor,
    sequence: &Sequence,
    cue_index: usize,
    _window: &Window,
    cx: &App,
) -> impl IntoElement {
    let header = h_flex()
        .justify_center()
        .w_full()
        .h_6()
        .bg(cx.theme().bg_primary)
        .border_b_1()
        .border_color(cx.theme().border_primary)
        .when(cx.theme().shadow, |e| e.shadow_xs())
        .child(
            div()
                .w_full()
                .px_1()
                .overflow_hidden()
                .text_ellipsis()
                .child(sequence.name().to_string()),
        )
        .child(
            h_flex()
                .justify_center()
                .w_5()
                .min_w_5()
                .h_full()
                .border_l_1()
                .border_color(cx.theme().border_primary)
                .font_weight(FontWeight::BOLD)
                .text_xs()
                .child(sequence.slot().to_string()),
        );

    let cue_names = sequence
        .cues()
        .iter()
        .enumerate()
        .map(|(ix, cue)| (ix, cue.name().to_string()))
        .collect::<Vec<_>>();
    let cues = uniform_list("cues", cue_names.len(), move |range, _, cx| {
        cue_names[range]
            .iter()
            .map(|(ix, cue_name)| {
                let is_active = *ix == cue_index;
                h_flex()
                    .w_full()
                    .h_6()
                    .bg(cx.theme().bg_secondary)
                    .when(is_active, |e| e.bg(cx.theme().accent.opacity(0.2)))
                    .border_b_1()
                    .border_color(cx.theme().border_primary)
                    .child(
                        h_flex()
                            .justify_center()
                            .w_5()
                            .min_w_5()
                            .h_full()
                            .border_r_1()
                            .border_color(cx.theme().border_primary)
                            .when(is_active, |e| e.font_weight(FontWeight::BOLD))
                            .text_xs()
                            .child((ix + 1).to_string()),
                    )
                    .child(
                        div()
                            .w_full()
                            .px_1()
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(cue_name.to_string()),
                    )
            })
            .collect()
    })
    .size_full();

    let state = {
        let master = v_flex()
            .flex_col_reverse()
            .min_w_5()
            .w_5()
            .h_full()
            .border_r_1()
            .border_color(cx.theme().border_primary)
            .child(
                div()
                    .w_full()
                    .h(relative(executor.master()))
                    .bg(cx.theme().accent.opacity(0.5))
                    .border_t_1()
                    .border_color(cx.theme().accent),
            );

        h_flex().h_16().border_t_1().border_color(cx.theme().border_primary).child(master).child(
            v_flex().justify_center().size_full().child(
                div()
                    .text_center()
                    .child(format!("{}%", (executor.master() * 100.0) as i32))
                    .font_weight(FontWeight::BOLD),
            ),
        )
    };

    div().size_full().flex().flex_col().child(header).child(cues).child(state)
}
