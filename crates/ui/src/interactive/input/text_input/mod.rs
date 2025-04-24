use blink::BlinkCursor;
use element::TextElement;
use gpui::*;
use prelude::FluentBuilder;
use std::ops::Range;

mod blink;
mod element;

pub mod actions {
    use gpui::{App, KeyBinding, actions};

    pub const KEY_CONTEXT: &str = "TextInput";

    actions!(
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

    pub fn init(cx: &mut App) {
        macro_rules! kb {
            (macos = $kb_macos:literal, other = $kb_other:literal, $action:expr) => {
                if cfg!(target_os = "macos") {
                    KeyBinding::new($kb_macos, $action, Some(KEY_CONTEXT))
                } else {
                    KeyBinding::new($kb_other, $action, Some(KEY_CONTEXT))
                }
            };
            (macos = $kb_macos:literal, $action:expr) => {
                #[cfg(target_os = "macos")]
                KeyBinding::new($kb_macos, $action, Some(KEY_CONTEXT))
            };
            (all = $kb:literal, $action:expr) => {
                kb!(macos = $kb, other = $kb, $action)
            };
        }

        cx.bind_keys([
            kb!(all = "left", MoveLeft),
            kb!(all = "right", MoveRight),
            //
            kb!(all = "home", MoveToStartOfLine),
            kb!(all = "pageup", MoveToStartOfLine),
            kb!(macos = "cmd-left", MoveToStartOfLine),
            //
            kb!(all = "pagedown", MoveToEndOfLine),
            kb!(all = "end", MoveToEndOfLine),
            kb!(macos = "cmd-right", MoveToEndOfLine),
            //
            kb!(all = "shift-left", SelectLeft),
            kb!(all = "shift-right", SelectRight),
            kb!(all = "backspace", Backspace),
            kb!(all = "delete", Delete),
            kb!(all = "enter", Submit),
            //
            kb!(macos = "shift-cmd-left", other = "shift-home", SelectToStartOfLine),
            kb!(macos = "shift-cmd-right", other = "shift-end", SelectToEndOfLine),
            kb!(macos = "alt-left", other = "ctrl-left", MoveToPreviousWord),
            kb!(macos = "alt-right", other = "ctrl-right", MoveToNextWord),
            kb!(macos = "alt-shift-left", other = "shift-ctrl-left", SelectToStartOfWord),
            kb!(macos = "alt-shift-right", other = "shift-ctrl-right", SelectToEndOfWord),
            kb!(all = "secondary-a", SelectAll),
            kb!(all = "secondary-c", Copy),
            kb!(all = "secondary-x", Cut),
            kb!(all = "secondary-v", Paste),
        ]);
    }
}

pub type Validator = dyn Fn(&str) -> bool;

pub struct TextInput {
    id: ElementId,

    text: SharedString,
    placeholder: SharedString,
    disabled: bool,
    masked: bool,
    validator: Option<Box<Validator>>,
    interactive: bool,

    utf16_selection: Range<usize>,
    new_selection_start_utf16_offset: Option<usize>,

    focus_handle: FocusHandle,
    last_prepaint_state: Option<element::PrepaintState>,
    scroll_offset: Pixels,
    padding_x: Pixels,

    blink_cursor: Entity<BlinkCursor>,
}

impl TextInput {
    pub fn new(
        id: impl Into<ElementId>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let blink_cursor = cx.new(|_cx| BlinkCursor::new());
        cx.observe(&blink_cursor, |_, _, cx| cx.notify()).detach();

        cx.on_focus(&focus_handle, window, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, window, Self::handle_blur).detach();

        Self {
            id: id.into(),

            text: "".into(),
            placeholder: "".into(),
            disabled: false,
            masked: false,
            validator: None,
            interactive: true,

            utf16_selection: 0..0,
            new_selection_start_utf16_offset: None,

            focus_handle,
            last_prepaint_state: None,
            scroll_offset: px(0.0),
            padding_x: px(0.0),

            blink_cursor,
        }
    }

    pub fn px(mut self, padding_x: Pixels) -> Self {
        self.padding_x = padding_x;
        self
    }

    pub fn set_text(&mut self, text: SharedString, cx: &mut Context<Self>) {
        if self.validator.as_ref().is_some_and(|validator| !validator(&text)) {
            cx.emit(TextInputEvent::ValidationRejected);
            return;
        }

        self.text = text;
        cx.emit(TextInputEvent::Change(self.text.clone()));
        cx.notify();
    }

    pub fn text(&self) -> &SharedString {
        &self.text
    }

    pub fn set_placeholder(&mut self, placeholder: SharedString, cx: &mut Context<Self>) {
        self.placeholder = placeholder;
        cx.notify();
    }

    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn masked(&self) -> bool {
        self.masked
    }

    pub fn set_masked(&mut self, masked: bool) {
        self.masked = masked;
    }

    pub fn set_validator(&mut self, validator: Option<Box<Validator>>) {
        self.validator = validator;
    }

    pub fn is_interactive(&self) -> bool {
        self.interactive
    }

    pub fn interactive(&mut self, interactive: bool) {
        self.interactive = interactive;
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus_handle);
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

    pub fn unselect(&mut self, cx: &mut Context<Self>) {
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

    pub fn has_selection(&self) -> bool {
        self.utf16_selection.start != self.utf16_selection.end
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
        if self.disabled() || !self.is_focused(window) {
            return false;
        }

        self.blink_cursor.read(cx).visible()
    }
}

impl TextInput {
    fn handle_move_left(
        &mut self,
        _: &actions::MoveLeft,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_left(cx);
        } else {
            self.move_to(self.char_selection().start, cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_right(
        &mut self,
        _: &actions::MoveRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_right(cx);
        } else {
            self.move_to(self.utf16_selection_range().end, cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_to_start_of_word(
        &mut self,
        _: &actions::MoveToPreviousWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_start_of_word(cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_to_end_of_word(
        &mut self,
        _: &actions::MoveToNextWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_end_of_word(cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_to_start_of_line(
        &mut self,
        _: &actions::MoveToStartOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_start_of_line(cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_to_end_of_line(
        &mut self,
        _: &actions::MoveToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_end_of_line(cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_select_left(
        &mut self,
        _: &actions::SelectLeft,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_left(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_right(
        &mut self,
        _: &actions::SelectRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_right(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_word(
        &mut self,
        _: &actions::SelectToStartOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_word(
        &mut self,
        _: &actions::SelectToEndOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_line(
        &mut self,
        _: &actions::SelectToStartOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_line(
        &mut self,
        _: &actions::SelectToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_all(
        &mut self,
        _: &actions::SelectAll,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_all(cx);
    }

    fn handle_copy(&mut self, _: &actions::Copy, _window: &mut Window, cx: &mut Context<Self>) {
        self.copy_selection(cx);
    }

    fn handle_cut(&mut self, _: &actions::Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.cut_selection(window, cx);
    }

    fn handle_paste(&mut self, _: &actions::Paste, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.paste(window, cx);
    }

    fn handle_backspace(
        &mut self,
        _: &actions::Backspace,
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

    fn handle_delete(&mut self, _: &actions::Delete, window: &mut Window, cx: &mut Context<Self>) {
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

    fn handle_submit(&mut self, _: &actions::Submit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(TextInputEvent::Submit);
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
        if self.is_interactive() {
            self.blink_cursor.update(cx, |blink_cursor, cx| {
                blink_cursor.start(cx);
            });
        }
        cx.emit(TextInputEvent::Focus);
    }

    fn handle_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_focused(window) {
            self.unselect(cx);
            self.blink_cursor.update(cx, |blink_cursor, cx| {
                blink_cursor.stop(cx);
            });
            cx.emit(TextInputEvent::Blur);
        }
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
            self.unselect(cx);
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

        self.set_text(new_text.into(), cx);

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
        if self.disabled {
            return;
        }
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
        let can_interact = self.is_interactive() && !self.disabled();

        div()
            .id(self.id.clone())
            .key_context(actions::KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .size_full()
            .px(self.padding_x)
            .child(div().child(TextElement::new(cx.entity().clone())).overflow_hidden())
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
                    .on_mouse_down_out(
                        cx.listener(|this, _, window, cx| this.handle_blur(window, cx)),
                    )
                    .on_drag(self.id.clone(), |_, _, _, cx| cx.new(|_| EmptyView))
                    .on_drag_move(cx.listener(Self::handle_drag_move))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            })
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextInputEvent {
    Focus,
    Blur,
    Submit,
    ValidationRejected,
    Change(SharedString),
}

impl EventEmitter<TextInputEvent> for TextInput {}
