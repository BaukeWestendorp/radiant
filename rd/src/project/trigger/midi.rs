use std::convert::TryFrom;
use std::fmt;

use anyhow::Context;
use rd_ui::EnumerableValue;

use crate::project::TriggerTarget;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[facet(deny_unknown_fields)]
pub struct MidiMapping {
    pub device_name: String,
    pub device_channel: MidiChannel,
    pub filter: MidiFilter,
    pub target: TriggerTarget,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiFilter {
    #[rd_ui(label = "Control Change")]
    ControlChange {
        #[rd_ui(label = "Controller")]
        controller: MidiController,
    },
    #[rd_ui(label = "Note On")]
    NoteOn {
        #[rd_ui(label = "Note")]
        note: MidiNote,
    },
    #[rd_ui(label = "Note Off")]
    NoteOff {
        #[rd_ui(label = "Note")]
        note: MidiNote,
    },
    #[rd_ui(label = "Pitch Bend")]
    PitchBend,
}

impl fmt::Display for MidiFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiFilter::ControlChange { controller: MidiController::Single(c) } => {
                write!(f, "CC {}", c.get())
            }
            MidiFilter::ControlChange { controller: MidiController::All } => {
                write!(f, "CC (Any)")
            }
            MidiFilter::NoteOn { note: MidiNote::Single(n) } => {
                write!(f, "Note On ({})", n.get())
            }
            MidiFilter::NoteOn { note: MidiNote::All } => write!(f, "Note On (Any)"),
            MidiFilter::NoteOff { note: MidiNote::Single(n) } => {
                write!(f, "Note Off ({})", n.get())
            }
            MidiFilter::NoteOff { note: MidiNote::All } => write!(f, "Note Off (Any)"),
            MidiFilter::PitchBend => write!(f, "Pitch Bend"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiChannel {
    #[default]
    All,
    Single(#[rd_ui(label = "Channel")] u4),
}

impl std::fmt::Display for MidiChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiChannel::All => write!(f, "All"),
            MidiChannel::Single(channel) => write!(f, "{}", channel.get()),
        }
    }
}

impl EnumerableValue for MidiChannel {
    fn enumerated_value(&self, offset: usize) -> Self {
        match self {
            MidiChannel::All => MidiChannel::All,
            MidiChannel::Single(channel) => {
                let new_channel = channel.get().saturating_add(offset as u8);
                let clamped_channel = new_channel.min(u4::MAX);
                MidiChannel::Single(u4(clamped_channel))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiNote {
    #[default]
    All,
    Single(#[rd_ui(label = "Note")] u7),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(facet::Facet)]
#[cfg_attr(feature = "rd-ui", derive(rd_ui::Input))]
#[repr(C)]
pub enum MidiController {
    #[default]
    All,
    Single(#[rd_ui(label = "Controller")] u7),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[allow(non_camel_case_types)]
pub struct u7(u8);

impl u7 {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 127;

    pub fn new(value: u8) -> Option<Self> {
        (value <= Self::MAX).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for u7 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("MIDI value must be between 0 and 127")
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::AutoInput for u7 {
    type Delegate = rd_ui::Slider<u7>;

    fn build_input(
        initial_value: Self,
        window: &mut rd_ui::gpui::Window,
        cx: &mut rd_ui::gpui::App,
    ) -> rd_ui::gpui::Entity<rd_ui::InputState<Self::Delegate>> {
        use rd_ui::gpui::AppContext as _;
        cx.new(move |cx| {
            let slider = rd_ui::Slider::new(cx.focus_handle(), window, cx)
                .with_value(Some(initial_value), cx);
            rd_ui::InputState::new(slider, window, cx)
        })
    }
}

impl rd_ui::SliderValue for u7 {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(Self::MIN as f64, Self::MAX as f64);
        Self(clamped as u8)
    }

    fn min_value() -> Option<Self> {
        Some(Self(Self::MIN))
    }

    fn max_value() -> Option<Self> {
        Some(Self(Self::MAX))
    }

    fn step_value() -> Option<Self> {
        Some(Self(1))
    }
}

impl std::str::FromStr for u7 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>().context("Invalid number")?;
        Self::new(value).context("MIDI value must be between 0 and 127")
    }
}

impl std::fmt::Display for u7 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[derive(facet::Facet)]
#[facet(transparent)]
#[allow(non_camel_case_types)]
pub struct u4(u8);

impl u4 {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 15;

    pub fn new(value: u8) -> Option<Self> {
        (value <= Self::MAX).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for u4 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or("MIDI channel must be between 0 and 15")
    }
}

#[cfg(feature = "rd-ui")]
impl rd_ui::AutoInput for u4 {
    type Delegate = rd_ui::Slider<u4>;

    fn build_input(
        initial_value: Self,
        window: &mut rd_ui::gpui::Window,
        cx: &mut rd_ui::gpui::App,
    ) -> rd_ui::gpui::Entity<rd_ui::InputState<Self::Delegate>> {
        use rd_ui::gpui::AppContext as _;
        cx.new(move |cx| {
            let slider = rd_ui::Slider::new(cx.focus_handle(), window, cx)
                .with_value(Some(initial_value), cx);
            rd_ui::InputState::new(slider, window, cx)
        })
    }
}

impl rd_ui::SliderValue for u4 {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(Self::MIN as f64, Self::MAX as f64);
        Self(clamped as u8)
    }

    fn min_value() -> Option<Self> {
        Some(Self(Self::MIN))
    }

    fn max_value() -> Option<Self> {
        Some(Self(Self::MAX))
    }

    fn step_value() -> Option<Self> {
        Some(Self(1))
    }
}

impl std::str::FromStr for u4 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u8>().context("Invalid number")?;
        Self::new(value).context("MIDI channel must be between 0 and 15")
    }
}

impl std::fmt::Display for u4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
