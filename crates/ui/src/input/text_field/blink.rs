use std::time::Duration;

use crate::theme::ActiveTheme;

use gpui::{ModelContext, Timer};

pub struct BlinkCursor {
    visible: bool,
    paused: bool,
    epoch: usize,
}

impl BlinkCursor {
    pub fn new() -> Self {
        Self {
            visible: false,
            paused: false,
            epoch: 0,
        }
    }

    pub fn start(&mut self, cx: &mut ModelContext<Self>) {
        self.blink(self.epoch, cx);
    }

    pub fn stop(&mut self, cx: &mut ModelContext<Self>) {
        self.epoch = 0;
        cx.notify();
    }

    pub fn visible(&self) -> bool {
        // Always show the cursor when paused.
        self.visible || self.paused
    }

    fn next_epoch(&mut self) -> usize {
        self.epoch += 1;
        self.epoch
    }

    fn blink(&mut self, epoch: usize, cx: &mut ModelContext<Self>) {
        if self.paused || epoch != self.epoch {
            return;
        }

        self.visible = !self.visible;
        cx.notify();

        // Schedule for next blink.
        let epoch = self.next_epoch();
        let interval = Duration::from_millis(cx.theme().cursor_blink_interval_ms);
        cx.spawn(|this, mut cx| async move {
            Timer::after(interval).await;
            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| this.blink(epoch, cx)).ok();
            }
        })
        .detach();
    }

    pub fn pause(&mut self, cx: &mut ModelContext<Self>) {
        self.paused = true;
        cx.notify();

        let epoch = self.next_epoch();
        let pause_delay = Duration::from_millis(cx.theme().cursor_blink_pause_delay_ms);
        cx.spawn(|this, mut cx| async move {
            Timer::after(pause_delay).await;
            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| {
                    this.paused = false;
                    this.blink(epoch, cx);
                })
                .ok();
            }
        })
        .detach();
    }
}
