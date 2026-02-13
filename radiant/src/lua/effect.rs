#[derive(Clone, Copy)]
pub struct UpdateContext {
    pub global_time: f64,
    pub delta_time: f64,
    pub global_frame: u64,
}

impl UpdateContext {
    pub fn new() -> Self {
        Self { global_time: 0.0, delta_time: 0.0, global_frame: 0 }
    }

    pub fn next_frame(mut self, global_time: f64, delta_time: f64) -> Self {
        self.global_time = global_time;
        self.delta_time = delta_time;
        self.global_frame = self.global_frame.saturating_add(1);
        self
    }
}

impl mlua::UserData for UpdateContext {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("global_time", |_, this| Ok(this.global_time));
        fields.add_field_method_get("delta_time", |_, this| Ok(this.delta_time));
        fields.add_field_method_get("global_frame", |_, this| Ok(this.global_frame));
    }
}
