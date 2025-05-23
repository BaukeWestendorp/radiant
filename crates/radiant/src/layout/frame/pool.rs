use crate::show::{self, AssetId, Show};
use crate::ui::FRAME_CELL_SIZE;
use gpui::{
    App, Empty, Entity, Focusable, MouseButton, ReadGlobal, UpdateGlobal, Window, div, prelude::*,
    px,
};
use ui::{
    ActiveTheme, ContainerStyle, Disableable, InteractiveColor, container, h6,
    interactive_container, utils::z_stack,
};

use super::Frame;

pub struct PoolFrame {
    frame: Entity<Frame>,
    pub kind: PoolFrameKind,
}

impl PoolFrame {
    pub fn new(kind: PoolFrameKind, frame: Entity<Frame>) -> Self {
        Self { kind, frame }
    }
}

pub enum PoolFrameKind {
    EffectGraphs,
    FixtureGroups,

    Cues,
    Sequences,
    Executors,

    Preset(PresetPoolFrameKind),
}

pub enum PresetPoolFrameKind {
    Dimmer,
    Position,
    Gobo,
    Color,
    Beam,
    Focus,
    Control,
    Shapers,
    Video,
}

impl PoolFrame {
    fn title(&self) -> &str {
        match &self.kind {
            PoolFrameKind::EffectGraphs => "Effect Graphs",
            PoolFrameKind::FixtureGroups => "Fixture Groups",

            PoolFrameKind::Cues => "Cues",
            PoolFrameKind::Sequences => "Sequences",
            PoolFrameKind::Executors => "Executors",

            PoolFrameKind::Preset(kind) => match kind {
                PresetPoolFrameKind::Dimmer => "Dimmer",
                PresetPoolFrameKind::Position => "Position",
                PresetPoolFrameKind::Gobo => "Gobo",
                PresetPoolFrameKind::Color => "Color",
                PresetPoolFrameKind::Beam => "Beam",
                PresetPoolFrameKind::Focus => "Focus",
                PresetPoolFrameKind::Control => "Control",
                PresetPoolFrameKind::Shapers => "Shapers",
                PresetPoolFrameKind::Video => "Video",
            },
        }
    }

    fn render_header_cell(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title().to_string();

        let border_color = if self.frame.focus_handle(cx).contains_focused(w, cx) {
            cx.theme().colors.border_focused
        } else {
            cx.theme().colors.header_border
        };

        div()
            .child(
                container(ContainerStyle {
                    background: cx.theme().colors.header_background,
                    border: border_color,
                    text_color: cx.theme().colors.text,
                })
                .size_full()
                .child(
                    div()
                        .h_full()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .text_center()
                        .child(h6(title)),
                ),
            )
            .id("pool-header")
            .size(FRAME_CELL_SIZE)
            .on_drag(
                super::HeaderDrag {
                    frame_entity_id: self.frame.entity_id(),
                    start_mouse_position: w.mouse_position(),
                },
                |_, _, _, cx| cx.new(|_| Empty),
            )
            .on_drag_move(cx.listener(|this, event, w, cx| {
                this.frame.update(cx, |frame, cx| frame.handle_header_drag(event, w, cx));
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, w, cx| {
                    this.frame.update(cx, |frame, cx| frame.release_resize_move(event, w, cx));
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, event, w, cx| {
                    this.frame.update(cx, |frame, cx| frame.release_resize_move(event, w, cx));
                }),
            )
    }

    fn render_cell(&mut self, id: u32, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cell_content = self.render_content(id, w, cx).map(|e| e.into_any_element());
        let has_content = cell_content.is_some();
        let cell_content = cell_content.unwrap_or_else(|| div().into_any_element());

        let overlay = div()
            .size_full()
            .pt_neg_0p5()
            .pl_0p5()
            .text_color(cx.theme().colors.text.muted())
            .child(id.to_string());

        interactive_container(id as usize, None)
            .size(FRAME_CELL_SIZE - px(2.0))
            .m_px()
            .cursor_pointer()
            .disabled(!has_content)
            .on_click(cx.listener(move |this, _event, _w, cx| {
                this.handle_on_click(id, cx);
                cx.notify();
            }))
            .child(
                z_stack([cell_content.into_any_element(), overlay.into_any_element()]).size_full(),
            )
    }

    fn render_content(
        &mut self,
        id: u32,
        _w: &mut Window,
        cx: &mut App,
    ) -> Option<impl IntoElement> {
        let render_basic_label = |label: String| {
            div()
                .h_full()
                .flex()
                .flex_col()
                .justify_center()
                .text_center()
                .child(label)
                .into_any_element()
        };

        let assets = &Show::global(cx).assets;
        match &self.kind {
            PoolFrameKind::EffectGraphs => assets
                .effect_graphs
                .get(&AssetId::new(id))
                .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            PoolFrameKind::FixtureGroups => assets
                .fixture_groups
                .get(&AssetId::new(id))
                .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            PoolFrameKind::Cues => assets
                .cues
                .get(&AssetId::new(id))
                .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            PoolFrameKind::Sequences => assets
                .sequences
                .get(&AssetId::new(id))
                .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            PoolFrameKind::Executors => assets
                .executors
                .get(&AssetId::new(id))
                .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            PoolFrameKind::Preset(kind) => match kind {
                PresetPoolFrameKind::Dimmer => assets
                    .dimmer_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Position => assets
                    .position_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Gobo => assets
                    .gobo_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Color => assets
                    .color_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Beam => assets
                    .beam_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Focus => assets
                    .focus_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Control => assets
                    .control_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Shapers => assets
                    .shapers_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
                PresetPoolFrameKind::Video => assets
                    .video_presets
                    .get(&AssetId::new(id))
                    .map(|asset| render_basic_label(asset.read(cx).label.clone())),
            },
        }
    }

    fn handle_on_click(&mut self, id: u32, cx: &mut Context<Self>) {
        Show::update_global(cx, |show, cx| {
            show.layout.update(cx, |layout, cx| {
                for frame in &mut layout.main_window.loaded_page.frames {
                    match &mut frame.kind {
                        show::FrameKind::Window(show::WindowFrameKind::EffectGraphEditor(
                            effect_graph,
                        )) => *effect_graph = AssetId::new(id),
                        _ => {}
                    }
                }
                cx.notify();
            });
            cx.notify();
        })
    }
}

impl Render for PoolFrame {
    fn render(&mut self, w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bounds = self.frame.read(cx).bounds;
        let area = bounds.size.width * bounds.size.height;
        let items =
            (0..area).map(|id| self.render_cell(id, w, cx).into_any_element()).collect::<Vec<_>>();

        div()
            .size_full()
            .flex()
            .flex_wrap()
            .child(self.render_header_cell(w, cx))
            .children(items)
            .overflow_hidden()
    }
}
