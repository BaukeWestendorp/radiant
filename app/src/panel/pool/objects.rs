use std::num::NonZeroU32;

use gpui::prelude::*;
use gpui::{ReadGlobal, Window, div};
use radiant::engine::{Command, Keyword, ObjectReference, Selection};
use radiant::show::{Group, Object, ObjectKind, PoolId, PresetDimmer};
use ui::interactive::modal::ModalExt;
use ui::theme::ActiveTheme;

use crate::panel::pool::{PoolPanel, PoolPanelDelegate};
use crate::state::{
    AppState, InteractionState, exec_cmd_and_log_err, exec_current_cmd_and_log_err,
    process_cmd_param, with_show,
};
use crate::ui::modal::StringModal;
use crate::ui::{DELETE_COLOR, RENAME_COLOR, STORE_COLOR, UPDATE_COLOR};

pub struct ObjectPool<T: Object> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Object> ObjectPool<T> {
    pub fn new() -> Self {
        Self { marker: Default::default() }
    }
}

impl<T: Object + Default + 'static> PoolPanelDelegate for ObjectPool<T> {
    fn cell_has_content(&self, pool_id: NonZeroU32, cx: &mut Context<PoolPanel<Self>>) -> bool {
        let id = with_show(cx, |show| {
            show.objects().get_by_pool_id::<T>(PoolId::new(pool_id)).map(|obj| obj.id())
        });
        id.is_some()
    }

    fn handle_cell_click(
        &self,
        pool_id: NonZeroU32,
        _event: &gpui::ClickEvent,
        window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) {
        let kind = T::default().kind();
        let pool_id = PoolId::new(pool_id);

        match AppState::global(cx).interaction_state(cx) {
            InteractionState::Store => {
                process_cmd_param(kind, cx);
                process_cmd_param(pool_id, cx);
                exec_current_cmd_and_log_err(cx);
            }
            InteractionState::Update => {
                process_cmd_param(kind, cx);
                process_cmd_param(pool_id, cx);
                exec_current_cmd_and_log_err(cx);
            }
            InteractionState::Delete => {
                process_cmd_param(kind, cx);
                process_cmd_param(pool_id, cx);
                exec_current_cmd_and_log_err(cx);
            }
            InteractionState::Rename => {
                cx.open_modal(move |cx| {
                    StringModal::new(window, cx).on_submit(move |value, cx| {
                        process_cmd_param(kind, cx);
                        process_cmd_param(pool_id, cx);
                        process_cmd_param(value.to_string(), cx);
                        exec_current_cmd_and_log_err(cx);
                        cx.close_modal();
                    })
                });
            }
            InteractionState::None => match kind {
                ObjectKind::Group => exec_cmd_and_log_err(
                    Command::Select {
                        selection: Selection::Object(ObjectReference { kind, pool_id }),
                    },
                    cx,
                ),
                ObjectKind::Executor => todo!(),
                ObjectKind::Sequence => todo!(),
                ObjectKind::PresetDimmer
                | ObjectKind::PresetPosition
                | ObjectKind::PresetGobo
                | ObjectKind::PresetColor
                | ObjectKind::PresetBeam
                | ObjectKind::PresetFocus
                | ObjectKind::PresetControl
                | ObjectKind::PresetShapers
                | ObjectKind::PresetVideo => {
                    process_cmd_param(Keyword::Recall, cx);
                    process_cmd_param(kind, cx);
                    process_cmd_param(pool_id, cx);
                    exec_current_cmd_and_log_err(cx);
                }
            },
        }
    }

    fn render_cell_content(
        &self,
        pool_id: NonZeroU32,
        _window: &mut Window,
        cx: &mut Context<PoolPanel<Self>>,
    ) -> impl IntoElement {
        let border_color = match AppState::global(cx).interaction_state(cx) {
            InteractionState::Store => STORE_COLOR.opacity(0.80),
            InteractionState::Update => UPDATE_COLOR.opacity(0.80),
            InteractionState::Delete => DELETE_COLOR.opacity(0.80),
            InteractionState::Rename => RENAME_COLOR.opacity(0.80),
            InteractionState::None => gpui::transparent_black(),
        };

        with_show(cx, |show| {
            let kind = T::default().kind();

            match kind {
                ObjectKind::Group => {
                    let Some(group) = show.objects().get_by_pool_id::<Group>(PoolId::new(pool_id))
                    else {
                        return div();
                    };

                    div().child(group.name().to_string())
                }
                ObjectKind::Executor => todo!(),
                ObjectKind::Sequence => {
                    let Some(sequence) =
                        show.objects().get_by_pool_id::<Group>(PoolId::new(pool_id))
                    else {
                        return div();
                    };

                    div().child(sequence.name().to_string())
                }
                ObjectKind::PresetDimmer => {
                    let Some(preset) =
                        show.objects().get_by_pool_id::<PresetDimmer>(PoolId::new(pool_id))
                    else {
                        return div();
                    };

                    div().child(preset.name().to_string())
                }
                ObjectKind::PresetPosition => todo!(),
                ObjectKind::PresetGobo => todo!(),
                ObjectKind::PresetColor => todo!(),
                ObjectKind::PresetBeam => todo!(),
                ObjectKind::PresetFocus => todo!(),
                ObjectKind::PresetControl => todo!(),
                ObjectKind::PresetShapers => todo!(),
                ObjectKind::PresetVideo => todo!(),
            }
        })
        .size_full()
        .border_2()
        .border_color(border_color)
        .rounded(cx.theme().radius)
    }
}
