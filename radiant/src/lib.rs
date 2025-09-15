pub mod attr;
pub mod builtin;
pub mod cmd;
pub mod comp;
pub mod engine;
pub mod error;

/// Re-export of `gdtf` crate.
pub mod gdtf {
    pub use gdtf::*;

    pub fn channel_count(dmx_mode: &dmx_mode::DmxMode) -> u16 {
        let mut low_offset = 1;
        let mut high_offset = 1;

        for dmx_channel in &dmx_mode.dmx_channels {
            let offsets = dmx_channel
                .offset
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|offset| (offset - 1).clamp(u16::MIN as i32, u16::MAX as i32) as u16);

            for offset in offsets {
                if low_offset == 1 || offset < low_offset {
                    low_offset = offset;
                }
                if high_offset == 1 || offset > high_offset {
                    high_offset = offset;
                }
            }
        }

        high_offset - low_offset + 1
    }
}
