use gpui::{
    AnyElement, App, Bounds, FontWeight, Pixels, ReadGlobal, SharedString, Size, Window, div,
    prelude::*, relative, uniform_list,
};
use rd_engine::{
    CueList, Executor, ExecutorContent, ExecutorPage, Object as _, ObjectKind, ObjectReference,
    SlotId,
};
use rd_ui::{ActiveTheme, TileDelegate, h_flex, v_flex};

use crate::engine::Engine;

pub struct ExecutorsTile {
    // FIXME: Keep this in some settings per tile?
    page_slot_id: SlotId,

    cell_size: Size<Pixels>,
}

impl ExecutorsTile {
    pub fn new(cell_size: Size<Pixels>, _window: &mut Window, _cx: &mut App) -> Self {
        Self { page_slot_id: SlotId::new(1).unwrap(), cell_size }
    }
}

impl TileDelegate for ExecutorsTile {
    fn title(&self, _cx: &App) -> SharedString {
        format!("Executors Page {}", self.page_slot_id).into()
    }

    // FIXME: Can we pass cell_size to render_content?
    fn render_content(&self, bounds: Bounds<u32>, window: &mut Window, cx: &App) -> AnyElement {
        let Some(page) = Engine::global(cx)
            .engine()
            .objects()
            .get::<ExecutorPage>((ObjectKind::ExecutorPage, self.page_slot_id))
        else {
            todo!();
        };

        let executors = page.executors().iter().enumerate().take(bounds.size.width as usize).map(
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
                                    .bg(cx.theme().success)
                            })
                            .child(format!("{}.{}", self.page_slot_id, ix + 1)),
                    );

                let executor_content = match executor.content() {
                    Some(ExecutorContent::CueList { cue_list, cue_index }) => {
                        render_cue_list_content(executor, *cue_list, *cue_index, window, cx)
                            .into_any_element()
                    }
                    None => div().size_full().bg(cx.theme().bg_secondary).into_any_element(),
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

fn render_cue_list_content(
    executor: &Executor,
    cue_list: ObjectReference,
    cue_index: usize,
    _window: &Window,
    cx: &App,
) -> impl IntoElement {
    let Some(cue_list) = Engine::global(cx).engine().objects().get::<CueList>(cue_list) else {
        log::error!("CueList not found");
        return div();
    };

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
                .child(cue_list.name().to_string()),
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
                .child(cue_list.slot_id().to_string()),
        );

    let cue_names = cue_list
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
