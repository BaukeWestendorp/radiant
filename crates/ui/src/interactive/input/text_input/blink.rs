use gpui::*;
use std::time::Duration;

const BLINK_TIME: Duration = Duration::from_millis(1000);
const HOLD_TIME: Duration = Duration::from_millis(500);

pub struct BlinkCursor {
    visible: bool,
    paused: bool,
    epoch: u64,
}

impl BlinkCursor {
    pub fn new() -> Self {
        Self { visible: false, paused: false, epoch: 0 }
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

    pub fn hold_and_start(&mut self, cx: &mut Context<Self>) {
        self.paused = true;
        cx.notify();

        let epoch = self.next_epoch();

        cx.spawn(async move |this, cx| {
            Timer::after(HOLD_TIME).await;

            this.update(cx, |this, cx| {
                this.paused = false;
                this.blink(epoch, cx);
            })
            .ok();
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
        cx.spawn(async move |this, cx| {
            Timer::after(BLINK_TIME).await;
            this.update(cx, |this: &mut Self, cx| this.blink(epoch, cx)).ok();
        })
        .detach();
    }
}
