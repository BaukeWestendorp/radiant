pub(crate) mod text_field;
pub(crate) mod text_input;

pub enum FieldEvent {
    Focus,
    Blur,
    Submit,
    Change,
}
