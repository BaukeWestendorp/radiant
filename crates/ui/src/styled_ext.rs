use gpui::Styled;

/// Extends [`Styled`].
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    /// Centers elements.
    ///
    /// Sets `flex()`, `items_center()`, `justify_center()`
    fn center_flex(self) -> Self {
        self.flex().items_center().justify_center()
    }
}

impl<E: Styled> StyledExt for E {}
