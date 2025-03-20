use crate::theme::ActiveTheme;
use blink::BlinkCursor;
use gpui::*;
use std::ops::Range;
use text_element::TextElement;

mod blink;
mod text_element;

const KEY_CONTEXT: &str = "TextInput";

// TODO:
// - History

actions!(
    input,
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
        kb!(macos = "cmd-a", other = "ctrl-a", SelectAll),
        kb!(macos = "cmd-c", other = "ctrl-c", Copy),
        kb!(macos = "cmd-x", other = "ctrl-x", Cut),
        kb!(macos = "cmd-v", other = "ctrl-v", Paste),
    ]);
}

pub type Validator = dyn Fn(&str) -> bool;

pub struct TextField {
    text: SharedString,
    placeholder: SharedString,
    disabled: bool,
    masked: bool,
    validator: Option<Box<Validator>>,

    utf16_selection: Range<usize>,
    new_selection_start_utf16_offset: Option<usize>,

    focus_handle: FocusHandle,
    last_prepaint_state: Option<text_element::PrepaintState>,
    scroll_offset: Pixels,

    blink_cursor: Entity<BlinkCursor>,
}

impl TextField {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let blink_cursor = cx.new(|_cx| BlinkCursor::new());
        cx.observe(&blink_cursor, |_, _, cx| cx.notify()).detach();

        cx.on_focus(&focus_handle, window, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, window, Self::handle_blur).detach();

        Self {
            text: "".into(),
            placeholder: "".into(),
            disabled: false,
            masked: false,
            validator: None,

            utf16_selection: 0..0,
            new_selection_start_utf16_offset: None,

            focus_handle,
            last_prepaint_state: None,
            scroll_offset: px(0.0),
            blink_cursor,
        }
    }

    pub fn set_text(&mut self, text: SharedString, cx: &mut Context<Self>) {
        if self.validator.as_ref().is_some_and(|validator| !validator(&text)) {
            cx.emit(TextFieldEvent::ValidationRejected);
            return;
        }

        self.text = text;
        cx.emit(TextFieldEvent::Change(self.text.clone()));
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

    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;

        self.blink_cursor.update(cx, |blink_cursor, cx| {
            if disabled {
                blink_cursor.stop(cx);
            } else {
                blink_cursor.start(cx);
            }
        });

        cx.notify();
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

    pub fn focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    pub fn move_to(&mut self, mut utf16_offset: usize, cx: &mut Context<Self>) {
        utf16_offset = utf16_offset.clamp(0, self.text.len());
        self.utf16_selection = utf16_offset..utf16_offset;
        self.hold_cursor_blink(cx);
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

    fn select_all(&mut self, cx: &mut Context<Self>) {
        self.end_current_selection(cx);
        self.move_to(0, cx);
        self.start_selection();
        self.move_to(self.text().len(), cx);
        self.end_current_selection(cx);
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

    fn hold_cursor_blink(&mut self, cx: &mut Context<Self>) {
        self.blink_cursor.update(cx, |blink_cursor, cx| {
            blink_cursor.hold(cx);
        });
    }

    fn show_cursor(&self, window: &Window, cx: &App) -> bool {
        if self.disabled() && self.focused(window) {
            return true;
        }

        self.blink_cursor.read(cx).visible()
    }
}

impl TextField {
    fn handle_move_left(&mut self, _: &MoveLeft, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.has_selection() {
            self.move_left(cx);
        } else {
            self.move_to(self.char_selection().start, cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_move_right(&mut self, _: &MoveRight, _window: &mut Window, cx: &mut Context<Self>) {
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
        _: &MoveToPreviousWord,
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
        _: &MoveToNextWord,
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
        _: &MoveToStartOfLine,
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
        _: &MoveToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.has_selection() {
            self.move_to_end_of_line(cx);
        }
        self.end_current_selection(cx);
        self.unselect(cx);
    }

    fn handle_select_left(&mut self, _: &SelectLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.start_selection();
        self.move_left(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_right(
        &mut self,
        _: &SelectRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_right(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_word(
        &mut self,
        _: &SelectToStartOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_word(
        &mut self,
        _: &SelectToEndOfWord,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_word(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_start_of_line(
        &mut self,
        _: &SelectToStartOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_start_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_to_end_of_line(
        &mut self,
        _: &SelectToEndOfLine,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.start_selection();
        self.move_to_end_of_line(cx);
        self.commit_current_selection(cx);
    }

    fn handle_select_all(&mut self, _: &SelectAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_all(cx);
    }

    fn handle_copy(&mut self, _: &Copy, _window: &mut Window, cx: &mut Context<Self>) {
        self.copy_selection(cx);
    }

    fn handle_cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.cut_selection(window, cx);
    }

    fn handle_paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.paste(window, cx);
    }

    fn handle_backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
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

    fn handle_delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
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

    fn handle_submit(&mut self, _: &Submit, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(TextFieldEvent::Submit);
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

        self.hold_cursor_blink(cx);

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
        event: &DragMoveEvent<()>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        self.blink_cursor.update(cx, |blink_cursor, cx| {
            blink_cursor.start(cx);
        });
        cx.emit(TextFieldEvent::Focus);
    }

    fn handle_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.focused(window) {
            self.unselect(cx);
            self.blink_cursor.update(cx, |blink_cursor, cx| {
                blink_cursor.stop(cx);
            });
            cx.emit(TextFieldEvent::Blur);
        }
    }
}

impl EntityInputHandler for TextField {
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

        self.unselect(cx);

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

impl Render for TextField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focused(window);

        let background_color =
            if focused { cx.theme().background_focused } else { cx.theme().background };

        let border_color = if focused {
            cx.theme().border_color_focused
        } else if self.disabled {
            cx.theme().border_color_muted
        } else {
            cx.theme().border_color
        };

        let text_color =
            if self.disabled { cx.theme().text_muted } else { cx.theme().text_primary };

        div()
            .id("text_input")
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .bg(background_color)
            .p_1()
            .border_1()
            .border_color(border_color)
            .rounded(cx.theme().radius)
            .text_color(text_color)
            .child(
                div().child(TextElement::new(cx.entity().clone())).cursor_text().overflow_hidden(),
            )
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
            .on_mouse_down_out(cx.listener(|this, _, window, cx| this.handle_blur(window, cx)))
            .on_drag((), |_, _, _, cx| cx.new(|_| EmptyView))
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextFieldEvent {
    Focus,
    Blur,
    Submit,
    ValidationRejected,
    Change(SharedString),
}

impl EventEmitter<TextFieldEvent> for TextField {}
