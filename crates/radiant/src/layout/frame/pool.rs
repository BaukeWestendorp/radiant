use crate::show::{self, AssetId, Show};
use crate::ui::FRAME_CELL_SIZE;
use gpui::{App, Entity, ReadGlobal, SharedString, UpdateGlobal, Window, div, prelude::*, px};
use ui::{ActiveTheme, Disableable, InteractiveColor, h6, interactive_container, utils::z_stack};

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

impl PoolFrameKind {
    pub fn into_show(&self) -> show::PoolFrameKind {
        match self {
            Self::EffectGraphs => show::PoolFrameKind::EffectGraphs,
            Self::FixtureGroups => show::PoolFrameKind::FixtureGroups,
            Self::Cues => show::PoolFrameKind::Cues,
            Self::Sequences => show::PoolFrameKind::Sequences,
            Self::Executors => show::PoolFrameKind::Executors,
            Self::Preset(kind) => match kind {
                PresetPoolFrameKind::Dimmer => show::PoolFrameKind::DimmerPresets,
                PresetPoolFrameKind::Position => show::PoolFrameKind::PositionPresets,
                PresetPoolFrameKind::Gobo => show::PoolFrameKind::GoboPresets,
                PresetPoolFrameKind::Color => show::PoolFrameKind::ColorPresets,
                PresetPoolFrameKind::Beam => show::PoolFrameKind::BeamPresets,
                PresetPoolFrameKind::Focus => show::PoolFrameKind::FocusPresets,
                PresetPoolFrameKind::Control => show::PoolFrameKind::ControlPresets,
                PresetPoolFrameKind::Shapers => show::PoolFrameKind::ShapersPresets,
                PresetPoolFrameKind::Video => show::PoolFrameKind::VideoPresets,
            },
        }
    }

    pub fn from_show(from: &show::PoolFrameKind) -> Self {
        match from {
            show::PoolFrameKind::EffectGraphs => PoolFrameKind::EffectGraphs,
            show::PoolFrameKind::FixtureGroups => PoolFrameKind::FixtureGroups,
            show::PoolFrameKind::Cues => PoolFrameKind::Cues,
            show::PoolFrameKind::Sequences => PoolFrameKind::Sequences,
            show::PoolFrameKind::Executors => PoolFrameKind::Executors,
            show::PoolFrameKind::DimmerPresets => {
                PoolFrameKind::Preset(PresetPoolFrameKind::Dimmer)
            }
            show::PoolFrameKind::PositionPresets => {
                PoolFrameKind::Preset(PresetPoolFrameKind::Position)
            }
            show::PoolFrameKind::GoboPresets => PoolFrameKind::Preset(PresetPoolFrameKind::Gobo),
            show::PoolFrameKind::ColorPresets => PoolFrameKind::Preset(PresetPoolFrameKind::Color),
            show::PoolFrameKind::BeamPresets => PoolFrameKind::Preset(PresetPoolFrameKind::Beam),
            show::PoolFrameKind::FocusPresets => PoolFrameKind::Preset(PresetPoolFrameKind::Focus),
            show::PoolFrameKind::ControlPresets => {
                PoolFrameKind::Preset(PresetPoolFrameKind::Control)
            }
            show::PoolFrameKind::ShapersPresets => {
                PoolFrameKind::Preset(PresetPoolFrameKind::Shapers)
            }
            show::PoolFrameKind::VideoPresets => PoolFrameKind::Preset(PresetPoolFrameKind::Video),
        }
    }
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
    fn title(&self) -> SharedString {
        self.kind.into_show().to_string().into()
    }

    fn render_header_cell(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content = div()
            .h_full()
            .flex()
            .flex_col()
            .justify_center()
            .text_center()
            .child(h6(self.title()))
            .into_any_element();

        super::header_container(self.frame.clone(), content, window, cx).size(FRAME_CELL_SIZE)
    }

    fn render_cell(
        &mut self,
        id: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cell_content = self.render_content(id, window, cx).map(|e| e.into_any_element());
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
        _window: &mut Window,
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
                        )) => *effect_graph = Some(AssetId::new(id)),
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bounds = self.frame.read(cx).bounds;
        let area = bounds.size.width * bounds.size.height;
        let items = (0..area)
            .map(|id| self.render_cell(id, window, cx).into_any_element())
            .collect::<Vec<_>>();

        div()
            .size_full()
            .flex()
            .flex_wrap()
            .child(self.render_header_cell(window, cx))
            .children(items)
            .overflow_hidden()
    }
}
