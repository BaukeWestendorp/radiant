#[derive(Clone)]
pub struct UpdateContext {
    pub global_time: f64,
    pub delta_time: f64,
    pub global_frame: u64,
}

impl mlua::UserData for UpdateContext {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("delta_time", |_, this| Ok(this.delta_time));
        fields.add_field_method_get("global_time", |_, this| Ok(this.global_time));
        fields.add_field_method_get("global_frame", |_, this| Ok(this.global_frame));
    }
}
