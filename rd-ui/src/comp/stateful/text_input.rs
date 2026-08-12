use gpui::{
    App, Bounds, ClipboardItem, DragMoveEvent, ElementId, EmptyView, Entity, EntityInputHandler,
    EventEmitter, FocusHandle, Focusable, MouseButton, MouseDownEvent, MouseUpEvent, Pixels,
    SharedString, UTF16Selection, Window, div, point, prelude::*, px,
};
use std::ops::Range;

use crate::{
    ActiveTheme,
    comp::{Disableable, FocusableComponent, Identifiable, stateful},
};

pub(crate) mod action {
    pub const KEY_CONTEXT: &str = "TextInput";

    gpui::actions!(
        text_input,
        [
            MoveLeft,
            MoveRight,
            MoveToPreviousWord,
            MoveToNextWord,
            MoveToStartOfLine,
            MoveToEndOfLine,
            SelectLeft,
            SelectRight,
            SelectToStartOfWord,
            SelectToEndOfWord,
            SelectToStartOfLine,
            SelectToEndOfLine,
            SelectAll,
            Copy,
            Cut,
            Paste,
            Backspace,
            Delete,
            Submit
        ]
    );
}

pub struct TextInput {
    id: ElementId,

    text: SharedString,
    placeholder: SharedString,
    disabled: bool,
    masked: bool,
    interactive: bool,
    validator: Option<Box<dyn Fn(&str) -> bool>>,
    submit_validator: Option<Box<dyn Fn(&str) -> bool>>,
    text_size: Pixels,

    utf16_selection: Range<usize>,
    new_selection_start_utf16_offset: Option<usize>,

    focus_handle: Entity<FocusHandle>,
    last_prepaint_state: Option<element::PrepaintState>,
    scroll_offset: Pixels,

    blink_cursor: Entity<blink::BlinkCursor>,
}

impl TextInput {
    pub fn new(id: impl Into<ElementId>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let blink_cursor = cx.new(|_cx| blink::BlinkCursor::new());
        cx.observe(&blink_cursor, |_, _, cx| cx.notify()).detach();

        let focus_handle = cx.new(|cx| cx.focus_handle().tab_stop(true));
        cx.on_focus(&focus_handle.read(cx).clone(), window, Self::handle_focus).detach();
        cx.on_blur(&focus_handle.read(cx).clone(), window, Self::handle_blur).detach();
        cx.observe_in(&focus_handle, window, |_, focus_handle, window, cx| {
            let focus_handle = focus_handle.read(cx).clone();
            cx.on_focus(&focus_handle, window, Self::handle_focus).detach();
            cx.on_blur(&focus_handle, window, Self::handle_blur).detach();
        })
        .detach();

        Self {
            id: id.into(),
            focus_handle,

            text: "".into(),
            placeholder: "".into(),
            disabled: false,
            masked: false,
            interactive: true,
            validator: None,
            submit_validator: None,
            text_size: cx.theme().font_size,

            utf16_selection: 0..0,
            new_selection_start_utf16_offset: None,

            last_prepaint_state: None,
            scroll_offset: px(0.0),

            blink_cursor,
        }
    }

    pub fn text(&self) -> &SharedString {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        if let Some(validator) = &self.validator
            && !text.trim().is_empty()
            && !validator(&text)
        {
            return;
        }

        self.text = text;
        cx.emit(stateful::event::Change(self.text.clone()));
        cx.notify();
    }

    pub fn with_text(mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        self.set_text(text, cx);
        self
    }

    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }

    pub fn set_placeholder(&mut self, placeholder: SharedString, cx: &mut Context<Self>) {
        self.placeholder = placeholder;
        cx.notify();
    }

    pub fn with_placeholder(mut self, placeholder: SharedString, cx: &mut Context<Self>) -> Self {
        self.set_placeholder(placeholder, cx);
        self
    }

    pub fn masked(&self) -> bool {
        self.masked
    }

    pub fn set_masked(&mut self, masked: bool) {
        self.masked = masked;
    }

    pub fn with_masked(mut self, masked: bool) -> Self {
        self.set_masked(masked);
        self
    }

    pub fn set_validator<F: Fn(&str) -> bool + 'static>(&mut self, validator: F) {
        self.validator = Some(Box::new(validator));
    }

    pub fn with_validator(mut self, validator: impl Fn(&str) -> bool + 'static) -> Self {
        self.set_validator(validator);
        self
    }

    pub fn set_submit_validator<F: Fn(&str) -> bool + 'static>(&mut self, validator: F) {
        self.submit_validator = Some(Box::new(validator));
    }

    pub fn with_submit_validator(mut self, validator: impl Fn(&str) -> bool + 'static) -> Self {
        self.set_submit_validator(validator);
        self
    }

    pub fn interactive(&self) -> bool {
        self.interactive
    }

    pub fn set_interactive(&mut self, interactive: bool, cx: &mut Context<Self>) {
        self.interactive = interactive;
        self.blink_cursor.update(cx, |blink_cursor, cx| blink_cursor.stop(cx));
    }

    pub fn with_interactive(mut self, interactive: bool, cx: &mut Context<Self>) -> Self {
        self.set_interactive(interactive, cx);
        self
    }

    pub fn text_size(&self) -> Pixels {
        self.text_size
    }

    pub fn set_text_size(&mut self, text_size: Pixels) {
        self.text_size = text_size;
    }

    pub fn with_text_size(mut self, text_size: Pixels) -> Self {
        self.set_text_size(text_size);
        self
    }

    pub fn move_to(&mut self, mut utf16_offset: usize, cx: &mut Context<Self>) {
        utf16_offset = utf16_offset.clamp(0, self.text.len());
        self.utf16_selection = utf16_offset..utf16_offset;
        self.hold_and_start_cursor_blink(cx);
        cx.notify();
    }

    pub fn move_left(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = self.cursor_char_offset().saturating_sub(1);
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn move_right(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = self.cursor_char_offset().saturating_add(1);
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn move_to_start_of_word(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = self.start_of_word_char_offset();
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn move_to_end_of_word(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = self.end_of_word_char_offset();
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn move_to_start_of_line(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = 0;
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn move_to_end_of_line(&mut self, cx: &mut Context<Self>) {
        let new_char_offset = self.text().chars().count();
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    pub fn has_selection(&self) -> bool {
        self.utf16_selection.start != self.utf16_selection.end
    }

    pub fn select(&mut self, utf16_range: Range<usize>, cx: &mut Context<Self>) {
        self.utf16_selection = utf16_range;
        cx.notify();
    }

    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        self.end_current_selection(cx);
        self.move_to(0, cx);
        self.start_selection();
        self.move_to(self.text().len(), cx);
        self.end_current_selection(cx);
    }

    pub fn deselect(&mut self, cx: &mut Context<Self>) {
        self.utf16_selection.start = self.cursor_utf16_offset();
        cx.notify();
    }

    fn start_selection(&mut self) {
        if self.new_selection_start_utf16_offset.is_some() {
            return;
        };

        self.new_selection_start_utf16_offset = Some(self.cursor_utf16_offset());
    }

    fn commit_current_selection(&mut self, cx: &mut Context<Self>) {
        if let Some(start) = self.new_selection_start_utf16_offset {
            let end = self.cursor_utf16_offset();
            self.select(start..end, cx);
        }
    }

    fn end_current_selection(&mut self, cx: &mut Context<Self>) {
        self.commit_current_selection(cx);
        self.new_selection_start_utf16_offset = None;
        cx.notify();
    }

    fn delete_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        let range = self.utf16_selection_range();
        self.replace_text_in_range(Some(range), "", window, cx);
    }

    fn copy_selection(&mut self, cx: &mut Context<Self>) {
        self.commit_current_selection(cx);
        let utf16_range = self.utf16_selection_range();
        let text = self.text[utf16_range].to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn cut_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.copy_selection(cx);
        self.delete_selection(window, cx);
    }

    fn paste(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.commit_current_selection(cx);
        if let Some(text) = cx.read_from_clipboard().and_then(|c| c.text()) {
            let utf16_range = self.utf16_selection_range();
            self.replace_text_in_range(Some(utf16_range), &text, window, cx);
        }
    }

    fn select_word_under_cursor(&mut self, cx: &mut Context<Self>) {
        self.move_to_start_of_word(cx);
        self.start_selection();
        self.move_to_end_of_word(cx);
        self.end_current_selection(cx);
    }

    fn char_offset_to_utf16(&self, char_offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for c in self.text.chars() {
            if utf8_count >= char_offset {
                break;
            }
            utf8_count += c.len_utf8();
            utf16_offset += c.len_utf16();
        }

        utf16_offset
    }

    fn char_offset_from_utf16(&self, utf16_offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.text.chars() {
            if utf16_count >= utf16_offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn cursor_utf16_offset(&self) -> usize {
        self.utf16_selection.end
    }

    fn cursor_char_offset(&self) -> usize {
        self.char_offset_from_utf16(self.cursor_utf16_offset())
    }

    fn cursor_char_range(&self) -> Range<usize> {
        let char_offset = self.cursor_char_offset();
        char_offset..char_offset
    }

    fn start_of_word_char_offset(&self) -> usize {
        let mut offset = self.cursor_char_offset();
        while offset > 0 && self.text.chars().nth(offset - 1).unwrap().is_whitespace() {
            offset -= 1;
        }
        while offset > 0 && !self.text.chars().nth(offset - 1).unwrap().is_whitespace() {
            offset -= 1;
        }
        offset
    }

    fn end_of_word_char_offset(&self) -> usize {
        let mut offset = self.cursor_char_offset();
        let chars = self.text.chars().count();
        while offset < chars && self.text.chars().nth(offset).unwrap().is_whitespace() {
            offset += 1;
        }
        while offset < chars && !self.text.chars().nth(offset).unwrap().is_whitespace() {
            offset += 1;
        }
        offset
    }

    fn utf16_selection_range(&self) -> Range<usize> {
        if self.utf16_selection.end < self.utf16_selection.start {
            self.utf16_selection.end..self.utf16_selection.start
        } else {
            self.utf16_selection.clone()
        }
    }

    fn char_selection(&self) -> Range<usize> {
        let start = self.char_offset_to_utf16(self.utf16_selection_range().start);
        let end = self.char_offset_to_utf16(self.utf16_selection_range().end);
        start..end
    }

    fn hold_and_start_cursor_blink(&mut self, cx: &mut App) {
        self.blink_cursor.update(cx, |blink_cursor, cx| {
            blink_cursor.hold_and_start(cx);
        });
    }

    fn cursor_shown(&self, window: &Window, cx: &App) -> bool {
        if self.disabled(cx) || !self.focus_handle.read(cx).is_focused(window) {
            return false;
        }

        self.blink_cursor.read(cx).visible()
    }
}

impl TextInput {
    fn handle_move_left(
        &mut self,
        _: &action::MoveLeft,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_left(cx);
        } else {
            self.move_to(self.char_selection().start, cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_move_right(
        &mut self,
        _: &action::MoveRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_right(cx);
        } else {
            self.move_to(self.utf16_selection_range().end, cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_move_to_start_of_word(
        &mut self,
        _: &action::MoveToPreviousWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_start_of_word(cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_move_to_end_of_word(
        &mut self,
        _: &action::MoveToNextWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_end_of_word(cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_move_to_start_of_line(
        &mut self,
        _: &action::MoveToStartOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_start_of_line(cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_move_to_end_of_line(
        &mut self,
        _: &action::MoveToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_end_of_line(cx);
        }
        self.end_current_selection(cx);
        self.deselect(cx);
    }

    fn handle_select_left(
        &mut self,
        _: &action::SelectLeft,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_left(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_right(
        &mut self,
        _: &action::SelectRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_right(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_word(
        &mut self,
        _: &action::SelectToStartOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_word(
        &mut self,
        _: &action::SelectToEndOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_line(
        &mut self,
        _: &action::SelectToStartOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_line(
        &mut self,
        _: &action::SelectToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_all(
        &mut self,
        _: &action::SelectAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_all(cx);
    }

    fn handle_copy(&mut self, _: &action::Copy, _window: &mut Window, cx: &mut Context<Self>) {
        self.copy_selection(cx);
    }

    fn handle_cut(&mut self, _: &action::Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.cut_selection(window, cx);
    }

    fn handle_paste(&mut self, _: &action::Paste, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.paste(window, cx);
    }

    fn handle_backspace(
        &mut self,
        _: &action::Backspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        if self.has_selection() {
            self.delete_selection(window, cx);
            return;
        }

        let utf16_offset = self.cursor_utf16_offset();
        let utf16_range = utf16_offset.saturating_sub(1)..utf16_offset;
        self.replace_text_in_range(Some(utf16_range), "", window, cx);
    }

    fn handle_delete(&mut self, _: &action::Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        if self.has_selection() {
            self.delete_selection(window, cx);
            return;
        }

        let utf16_offset = self.cursor_utf16_offset();
        let utf16_range = utf16_offset..utf16_offset.saturating_add(1);
        self.replace_text_in_range(Some(utf16_range), "", window, cx);
    }

    fn handle_submit(&mut self, _: &action::Submit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(stateful::event::Submit(self.text.clone()));
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let point = event.position + point(self.scroll_offset, px(0.0));
        let Some(char_offset) = self.character_index_for_point(point, window, cx) else {
            return;
        };

        self.hold_and_start_cursor_blink(cx);

        let utf16_offset = self.char_offset_to_utf16(char_offset);
        self.move_to(utf16_offset, cx);

        match event.click_count {
            2 => {
                self.select_word_under_cursor(cx);
                return;
            }
            3 => {
                self.select_all(cx);
                return;
            }
            _ => {}
        }

        self.start_selection();
    }

    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<ElementId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.drag(cx) != &self.id {
            return;
        }

        if !event.event.dragging() {
            return;
        }

        let point = event.event.position + point(self.scroll_offset, px(0.0));
        let Some(char_offset) = self.character_index_for_point(point, window, cx) else {
            return;
        };

        let utf16_offset = self.char_offset_to_utf16(char_offset);
        self.move_to(utf16_offset, cx);
        self.commit_current_selection(cx);
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.end_current_selection(cx);
    }

    fn handle_focus(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.interactive() {
            self.blink_cursor.update(cx, |blink_cursor, cx| {
                blink_cursor.start(cx);
            });
        }
    }

    fn handle_blur(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(submit_validator) = &self.submit_validator
            && !submit_validator(&self.text)
        {
            self.set_text("", cx);
        }

        self.deselect(cx);
        self.blink_cursor.update(cx, |blink_cursor, cx| {
            blink_cursor.stop(cx);
        });
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        _utf16_range: std::ops::Range<usize>,
        _adjusted_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        todo!()
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        todo!()
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<std::ops::Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        utf16_range: Option<std::ops::Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        if self.has_selection() {
            let selection_range = self.utf16_selection_range();
            self.deselect(cx);
            self.replace_text_in_range(Some(selection_range), text, _window, cx);
            return;
        }

        let char_range = match utf16_range {
            Some(utf16_range) => {
                self.char_offset_from_utf16(utf16_range.start)
                    ..self.char_offset_from_utf16(utf16_range.end)
            }
            _ => self.cursor_char_range(),
        };

        let new_text =
            self.text[0..char_range.start].to_owned() + text + &self.text[char_range.end..];

        self.set_text(new_text, cx);

        // Move the cursor to the end of the inserted text.
        let utf16_offset = self.char_offset_to_utf16(char_range.start) + text.len();
        self.move_to(utf16_offset, cx);

        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _utf16_range: Option<std::ops::Range<usize>>,
        _new_text: &str,
        _utf16_new_selected_range: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        if self.disabled {}
    }

    fn bounds_for_range(
        &mut self,
        _utf16_range: std::ops::Range<usize>,
        _element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        todo!()
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let prepaint_state = self.last_prepaint_state.as_ref()?;
        let x = point.x - prepaint_state.bounds.origin.x;
        let char_offset = prepaint_state.line.closest_index_for_x(x);
        Some(char_offset)
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_interact = self.interactive() && !self.disabled(cx);

        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle(cx))
            .key_context(action::KEY_CONTEXT)
            .text_size(self.text_size)
            .size_full()
            .child(
                div()
                    .size_full()
                    .child(element::TextInputElement::new(cx.entity().clone()))
                    .overflow_hidden(),
            )
            .when(can_interact, |e| {
                e.cursor_text()
                    .on_action(cx.listener(Self::handle_move_left))
                    .on_action(cx.listener(Self::handle_move_right))
                    .on_action(cx.listener(Self::handle_move_to_start_of_word))
                    .on_action(cx.listener(Self::handle_move_to_end_of_word))
                    .on_action(cx.listener(Self::handle_move_to_start_of_line))
                    .on_action(cx.listener(Self::handle_move_to_end_of_line))
                    .on_action(cx.listener(Self::handle_select_left))
                    .on_action(cx.listener(Self::handle_select_right))
                    .on_action(cx.listener(Self::handle_select_to_start_of_word))
                    .on_action(cx.listener(Self::handle_select_to_end_of_word))
                    .on_action(cx.listener(Self::handle_select_to_start_of_line))
                    .on_action(cx.listener(Self::handle_select_to_end_of_line))
                    .on_action(cx.listener(Self::handle_select_all))
                    .on_action(cx.listener(Self::handle_copy))
                    .on_action(cx.listener(Self::handle_cut))
                    .on_action(cx.listener(Self::handle_paste))
                    .on_action(cx.listener(Self::handle_backspace))
                    .on_action(cx.listener(Self::handle_delete))
                    .on_action(cx.listener(Self::handle_submit))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                    .on_mouse_down_out(cx.listener(|_, _, w, _| w.blur()))
                    .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_| EmptyView))
                    .on_drag_move(cx.listener(Self::handle_drag_move))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            })
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.read(cx).clone()
    }
}

impl FocusableComponent for TextInput {
    fn set_focus_handle(&mut self, focus_handle: FocusHandle, cx: &mut App) {
        self.focus_handle.write(cx, focus_handle);
    }
}

impl Identifiable for TextInput {
    fn id(&self, _cx: &App) -> &ElementId {
        &self.id
    }
}

impl Disableable for TextInput {
    fn disabled(&self, _cx: &App) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool, _cx: &mut App) {
        self.disabled = disabled;
    }
}

impl EventEmitter<stateful::event::Submit<SharedString>> for TextInput {}
impl EventEmitter<stateful::event::Change<SharedString>> for TextInput {}

mod element {
    use super::TextInput;
    use crate::{HslaExt, comp::Disableable, theme::ActiveTheme};

    use gpui::{
        App, BorderStyle, Bounds, ElementId, ElementInputHandler, Entity, FontStyle,
        GlobalElementId, InspectorElementId, LayoutId, Pixels, ShapedLine, Style, TextAlign,
        Window, fill, outline, point, prelude::*, px, size,
    };

    pub struct TextInputElement {
        input: Entity<TextInput>,
    }

    impl TextInputElement {
        pub fn new(input: Entity<TextInput>) -> Self {
            Self { input }
        }
    }

    impl Element for TextInputElement {
        type RequestLayoutState = ();
        type PrepaintState = PrepaintState;

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn request_layout(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let mut style = Style::default();
            style.size.height = window.line_height().into();

            let layout_id = window.request_layout(style, [], cx);
            (layout_id, ())
        }

        fn prepaint(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            bounds: Bounds<Pixels>,
            _: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut App,
        ) -> Self::PrepaintState {
            let input = self.input.read(cx);
            let style = window.text_style();

            // Text.
            let display_text = if input.text().is_empty() {
                input.placeholder().to_string()
            } else if input.masked() {
                input.text().chars().map(|_| '*').collect()
            } else {
                input.text().to_string()
            };

            // Line.
            let text_size = style.font_size.to_pixels(window.rem_size());
            let text_len = display_text.len();
            let mut run = style.to_run(text_len);
            if input.text().is_empty() {
                run.color = cx.theme().fg_tertiary;
                run.font.style = FontStyle::Italic;
            } else if input.disabled(cx) {
                run.color = run.color.disabled();
            };
            let line =
                window.text_system().shape_line(display_text.into(), text_size, &[run], None);

            // Cursor.
            let cursor_x_offset = line.x_for_index(input.cursor_char_offset());
            let cursor_origin = bounds.origin + point(cursor_x_offset, px(0.0));
            let cursor_bounds =
                gpui::bounds(cursor_origin, size(cx.theme().cursor_width, window.line_height()));

            // Selection.
            let char_selection = input.char_selection();
            let start = line.x_for_index(char_selection.start);
            let end = line.x_for_index(char_selection.end);
            let selection_bounds = gpui::bounds(
                bounds.origin + point(start, px(0.0)),
                size(end - start, window.line_height()),
            );

            let prepaint_state = PrepaintState { bounds, line, cursor_bounds, selection_bounds };
            self.input
                .update(cx, |input, _cx| input.last_prepaint_state = Some(prepaint_state.clone()));
            prepaint_state
        }

        fn paint(
            &mut self,
            _: Option<&GlobalElementId>,
            _: Option<&InspectorElementId>,
            _: Bounds<Pixels>,
            _: &mut Self::RequestLayoutState,
            prepaint: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
            let Self::PrepaintState { bounds, line, cursor_bounds, selection_bounds } = prepaint;

            // Calculate scroll offset.
            let cursor_px_offset = cursor_bounds.right() - bounds.left();
            if cursor_px_offset >= bounds.size.width {
                self.input.update(cx, |input, _cx| {
                    let new_offset = cursor_px_offset - bounds.size.width;
                    if new_offset > input.scroll_offset {
                        input.scroll_offset = new_offset;
                    }
                });
            }
            let scroll_offset = self.input.read(cx).scroll_offset;
            if cursor_px_offset < scroll_offset {
                self.input.update(cx, |input, _cx| {
                    let new_offset = cursor_px_offset - cursor_bounds.size.width;
                    if new_offset < input.scroll_offset {
                        input.scroll_offset = new_offset;
                    }
                });
            }

            let input = self.input.read(cx);
            let should_show_cursor = input.cursor_shown(window, cx);
            let focus_handle = input.focus_handle.clone();

            // Handle Input.
            window.handle_input(
                &focus_handle.read(cx),
                ElementInputHandler::new(*bounds, self.input.clone()),
                cx,
            );

            let text_offset = point(-input.scroll_offset, px(0.0));

            // Paint selection.
            window.paint_quad(fill(*selection_bounds + text_offset, cx.theme().bg_selected));
            window.paint_quad(outline(
                *selection_bounds + text_offset,
                cx.theme().border_selected.opacity(0.25),
                BorderStyle::Solid,
            ));

            // Paint text.
            _ = line.paint(
                bounds.origin + text_offset,
                window.line_height(),
                TextAlign::Left,
                Some(prepaint.bounds.size.width),
                window,
                cx,
            );

            // Paint cursor if visible and input is not disabled.
            if should_show_cursor {
                window.paint_quad(fill(*cursor_bounds + text_offset, cx.theme().accent));
            }
        }

        fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
            None
        }
    }

    impl IntoElement for TextInputElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    #[derive(Debug, Clone)]
    pub struct PrepaintState {
        pub bounds: Bounds<Pixels>,
        pub line: ShapedLine,
        pub cursor_bounds: Bounds<Pixels>,
        pub selection_bounds: Bounds<Pixels>,
    }
}

mod blink {
    use std::time::Duration;

    use gpui::prelude::*;

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
                cx.background_executor().timer(HOLD_TIME).await;

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
                cx.background_executor().timer(BLINK_TIME).await;
                this.update(cx, |this: &mut Self, cx| this.blink(epoch, cx)).ok();
            })
            .detach();
        }
    }
}
