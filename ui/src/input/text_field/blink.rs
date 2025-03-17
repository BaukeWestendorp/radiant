use gpui::{Context, Timer};
use std::time::Duration;

const BLINK_TIME: Duration = Duration::from_millis(1000);

pub struct BlinkCursor {
    visible: bool,
    paused: bool,
    epoch: u64,
}

impl BlinkCursor {
    pub fn new() -> Self {
        Self { visible: true, paused: false, epoch: 0 }
    }

    pub fn visible(&self) -> bool {
        // Always visible when paused.
        self.paused || self.visible
    }

    pub fn start(&mut self, cx: &mut Context<Self>) {
        self.blink(self.epoch, cx);
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.epoch = 0;
        self.visible = false;
        cx.notify();
    }

    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.paused = true;
        cx.notify();

        let epoch = self.next_epoch();
        cx.spawn(|this, mut cx| async move {
            Timer::after(BLINK_TIME).await;

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| {
                    this.paused = true;
                    this.blink(epoch, cx);
                })
                .ok();
            }
        })
        .detach();
    }

    fn next_epoch(&mut self) -> u64 {
        self.epoch += 1;
        self.epoch
    }

    fn blink(&mut self, wait_until_epoch: u64, cx: &mut Context<Self>) {
        if self.paused || self.epoch != wait_until_epoch {
            return;
        }

        self.visible = !self.visible;
        cx.notify();

        let epoch = self.next_epoch();
        cx.spawn(|this, mut cx| async move {
            Timer::after(BLINK_TIME).await;

            if let Some(this) = this.upgrade() {
                this.update(&mut cx, |this, cx| this.blink(epoch, cx)).ok();
            }
        })
        .detach();
    }
}
