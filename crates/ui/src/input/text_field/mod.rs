use std::ops::Range;

use blink::BlinkCursor;
use gpui::*;
use unicode_segmentation::*;

use crate::{theme::ActiveTheme, ContainerKind, InteractiveContainer, StyledExt};

mod blink;
mod element;

actions!(
    input,
    [
        Backspace,
        Delete,
        Enter,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        SelectToHome,
        SelectToEnd,
        ShowCharacterPalette,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo,
        MoveToStartOfLine,
        MoveToEndOfLine,
        TextChanged,
    ]
);

const KEY_CONTEXT: &str = "Input";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some(KEY_CONTEXT)),
        KeyBinding::new("delete", Delete, Some(KEY_CONTEXT)),
        KeyBinding::new("enter", Enter, Some(KEY_CONTEXT)),
        KeyBinding::new("left", Left, Some(KEY_CONTEXT)),
        KeyBinding::new("right", Right, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(KEY_CONTEXT)),
        KeyBinding::new("home", Home, Some(KEY_CONTEXT)),
        KeyBinding::new("end", End, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-home", SelectToHome, Some(KEY_CONTEXT)),
        KeyBinding::new("shift-end", SelectToEnd, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("shift-cmd-left", SelectToHome, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("shift-cmd-right", SelectToEnd, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-a", Home, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", Home, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-e", End, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", End, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-z", Undo, Some(KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-z", Redo, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-z", Undo, Some(KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-y", Redo, Some(KEY_CONTEXT)),
    ]);
}

pub struct TextField {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    pattern: Option<regex::Regex>,
    validate: Option<Box<dyn Fn(&str) -> bool + 'static>>,

    blink_cursor: Model<BlinkCursor>,
    focus_handle: FocusHandle,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    scroll_offset: Point<Pixels>,

    is_selecting: bool,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, value: SharedString, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let blink_cursor = cx.new_model(|_| BlinkCursor::new());

        let field = Self {
            id: id.into(),
            value,
            placeholder: "".to_string().into(),
            pattern: None,
            validate: None,

            blink_cursor,
            focus_handle: focus_handle.clone(),
            last_layout: None,
            last_bounds: None,
            scroll_offset: point(px(0.0), px(0.0)),

            is_selecting: false,
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
        };

        cx.observe(&field.blink_cursor, |_, _, cx| cx.notify())
            .detach();

        // Blink the cursor when the window is active, pause when it's not.
        cx.observe_window_activation(|field, cx| {
            if cx.is_window_active() {
                let focus_handle = field.focus_handle.clone();
                if focus_handle.is_focused(cx) {
                    field.blink_cursor.update(cx, |blink_cursor, cx| {
                        blink_cursor.start(cx);
                    });
                }
            }
        })
        .detach();

        cx.on_focus(&focus_handle, Self::on_focus).detach();
        cx.on_blur(&focus_handle, Self::on_blur).detach();

        field
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: SharedString, cx: &mut ViewContext<Self>) {
        self.replace_text(value.clone(), cx);
        cx.emit(TextFieldEvent::Change(value));
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.replace_text("", cx);
    }

    pub fn set_placeholder(&mut self, placeholder: SharedString) {
        self.placeholder = placeholder;
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn set_pattern(&mut self, pattern: Option<regex::Regex>) {
        self.pattern = pattern;
    }

    pub fn pattern(&self) -> Option<&regex::Regex> {
        self.pattern.as_ref()
    }

    pub fn set_validate(&mut self, validate: Option<Box<dyn Fn(&str) -> bool + 'static>>) {
        self.validate = validate;
    }

    pub fn validate(&self) -> Option<&dyn Fn(&str) -> bool> {
        self.validate.as_deref()
    }

    pub fn focus(&self, cx: &mut ViewContext<Self>) {
        self.focus_handle.focus(cx);
    }

    fn pause_blink_cursor(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_cursor.update(cx, |cursor, cx| {
            cursor.pause(cx);
        });
    }

    pub(super) fn on_drag_move(&mut self, event: &MouseMoveEvent, cx: &mut ViewContext<Self>) {
        if self.value.is_empty() {
            return;
        }

        if self.last_layout.is_none() {
            return;
        }

        if !self.focus_handle.is_focused(cx) {
            return;
        }

        if !self.is_selecting {
            return;
        }

        let offset = self.offset_of_position(event.position);
        self.select_to(offset, cx);
    }

    pub(crate) fn show_cursor(&self, cx: &WindowContext) -> bool {
        self.focus_handle.is_focused(cx) && self.blink_cursor.read(cx).visible()
    }

    fn on_key_down_for_blink_cursor(&mut self, _: &KeyDownEvent, cx: &mut ViewContext<Self>) {
        self.pause_blink_cursor(cx)
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.value.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.value.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn offset_of_position(&self, position: Point<Pixels>) -> usize {
        let bounds = self.last_bounds.unwrap_or_default();
        let position = position - bounds.origin;

        self.last_layout
            .as_ref()
            .map(|line| match line.index_for_x(position.x) {
                Some(ix) => ix,
                None => {
                    let last_ix = line.len();

                    // If the mouse is on the right side of the last character, move to the end
                    // Otherwise, move to the start of the line
                    if position.x > line.x_for_index(last_ix) {
                        last_ix
                    } else {
                        0
                    }
                }
            })
            .unwrap_or(0)
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.value.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };

        if position.y < bounds.top() {
            return 0;
        }

        if position.y > bounds.bottom() {
            return self.value.len();
        }

        line.closest_index_for_x(position.x - bounds.left())
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.value
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.value.len())
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn select_to(&mut self, offset: usize, cx: &mut ViewContext<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }

        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }

        cx.notify();
    }

    fn move_to(&mut self, offset: usize, cx: &mut ViewContext<Self>) {
        self.selected_range = offset..offset;
        self.pause_blink_cursor(cx);
        cx.notify()
    }

    /// Select the word at the given offset.
    fn select_word(&mut self, offset: usize, cx: &mut ViewContext<Self>) {
        fn is_word(c: char) -> bool {
            c.is_alphanumeric() || matches!(c, '_')
        }

        let mut start = self.offset_to_utf16(offset);
        let mut end = start;
        let prev_text = self
            .text_for_range(0..start, &mut None, cx)
            .unwrap_or_default();
        let next_text = self
            .text_for_range(end..self.value.len(), &mut None, cx)
            .unwrap_or_default();

        let prev_chars = prev_text.chars().rev().peekable();
        let next_chars = next_text.chars().peekable();

        for c in prev_chars {
            if !is_word(c) {
                break;
            }

            start -= c.len_utf16();
        }

        for c in next_chars {
            if !is_word(c) {
                break;
            }

            end += c.len_utf16();
        }

        self.selected_range = self.range_from_utf16(&(start..end));
        cx.notify()
    }

    fn unselect(&mut self, cx: &mut ViewContext<Self>) {
        self.selected_range = self.cursor_offset()..self.cursor_offset();
        cx.notify()
    }

    fn backspace(&mut self, _: &Backspace, cx: &mut ViewContext<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", cx);
        self.pause_blink_cursor(cx);
    }

    fn delete(&mut self, _: &Delete, cx: &mut ViewContext<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", cx);
        self.pause_blink_cursor(cx);
    }

    fn enter(&mut self, _: &Enter, cx: &mut ViewContext<Self>) {
        cx.emit(TextFieldEvent::PressEnter);
        cx.emit(TextFieldEvent::Change(self.value.clone()));
        cx.blur();
    }

    fn left(&mut self, _: &Left, cx: &mut ViewContext<Self>) {
        self.pause_blink_cursor(cx);
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, cx: &mut ViewContext<Self>) {
        self.pause_blink_cursor(cx);
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, cx: &mut ViewContext<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, cx: &mut ViewContext<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, cx: &mut ViewContext<Self>) {
        self.move_to(0, cx);
        self.select_to(self.value.len(), cx)
    }

    fn home(&mut self, _: &Home, cx: &mut ViewContext<Self>) {
        self.pause_blink_cursor(cx);
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, cx: &mut ViewContext<Self>) {
        self.pause_blink_cursor(cx);
        self.move_to(self.value.len(), cx);
    }

    fn select_to_home(&mut self, _: &SelectToHome, cx: &mut ViewContext<Self>) {
        self.select_to(0, cx);
    }

    fn select_to_end(&mut self, _: &SelectToEnd, cx: &mut ViewContext<Self>) {
        self.select_to(self.value.len(), cx);
    }

    fn copy(&mut self, _: &Copy, cx: &mut ViewContext<Self>) {
        if self.selected_range.is_empty() {
            return;
        }

        let selected_text = self.value[self.selected_range.clone()].to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(selected_text));
    }

    fn cut(&mut self, _: &Cut, cx: &mut ViewContext<Self>) {
        if self.selected_range.is_empty() {
            return;
        }

        let selected_text = self.value[self.selected_range.clone()].to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(selected_text));
        self.replace_text_in_range(None, "", cx);
    }

    fn paste(&mut self, _: &Paste, cx: &mut ViewContext<Self>) {
        if let Some(clipboard) = cx.read_from_clipboard() {
            let new_text = clipboard.text().unwrap_or_default().replace('\n', "");
            self.replace_text_in_range(None, &new_text, cx);
        }
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        self.is_selecting = true;
        let offset = self.index_for_mouse_position(event.position);

        // Double click to select word
        if event.button == MouseButton::Left && event.click_count == 2 {
            self.select_word(offset, cx);
            return;
        }

        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else {
            self.move_to(offset, cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut ViewContext<Self>) {
        self.is_selecting = false;
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.blink_cursor.update(cx, |cursor, cx| {
            cursor.start(cx);
        });
        cx.emit(TextFieldEvent::Focus);
    }

    fn on_blur(&mut self, cx: &mut ViewContext<Self>) {
        self.unselect(cx);
        self.blink_cursor.update(cx, |cursor, cx| {
            cursor.stop(cx);
        });
        cx.emit(TextFieldEvent::Blur);
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, cx: &mut ViewContext<Self>) {
        cx.show_character_palette();
    }

    fn is_valid_input(&self, new_text: &str) -> bool {
        if new_text.is_empty() {
            return true;
        }

        if let Some(validate) = &self.validate {
            if !validate(new_text) {
                return false;
            }
        }

        self.pattern
            .as_ref()
            .map(|p| p.is_match(new_text))
            .unwrap_or(true)
    }

    fn replace_text(&mut self, text: impl Into<SharedString>, cx: &mut ViewContext<Self>) {
        let text: SharedString = text.into();
        let range = 0..self.value.chars().map(|c| c.len_utf16()).sum();
        self.replace_text_in_range(Some(range), &text, cx);
    }
}

impl ViewInputHandler for TextField {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _cx: &mut ViewContext<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _cx: &mut ViewContext<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: false,
        })
    }

    fn marked_text_range(&self, _cx: &mut ViewContext<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _cx: &mut ViewContext<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_value: &str,
        cx: &mut ViewContext<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        let pending_text: SharedString =
            (self.value[0..range.start].to_owned() + new_value + &self.value[range.end..]).into();
        if !self.is_valid_input(&pending_text) {
            return;
        }

        self.value = pending_text;
        self.selected_range = range.start + new_value.len()..range.start + new_value.len();
        self.marked_range.take();
        cx.emit(TextFieldEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        cx: &mut ViewContext<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let pending_text: SharedString =
            (self.value[0..range.start].to_owned() + new_text + &self.value[range.end..]).into();
        if !self.is_valid_input(&pending_text) {
            return;
        }

        self.value = pending_text;
        self.marked_range = Some(range.start..range.start + new_text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
        cx.emit(TextFieldEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _cx: &mut ViewContext<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }
}

impl FocusableView for TextField {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<TextFieldEvent> for TextField {}

#[derive(Debug, Clone)]
pub enum TextFieldEvent {
    Change(SharedString),
    PressEnter,
    Focus,
    Blur,
}

impl Render for TextField {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(cx);
        InteractiveContainer::new(
            ContainerKind::Custom {
                bg: cx.theme().background,
                border_color: ContainerKind::Element.border_color(cx),
            },
            self.id.clone(),
            focused,
            false,
        )
        .track_focus(&self.focus_handle)
        .key_context(KEY_CONTEXT)
        .on_action(cx.listener(Self::backspace))
        .on_action(cx.listener(Self::delete))
        .on_action(cx.listener(Self::enter))
        .on_action(cx.listener(Self::left))
        .on_action(cx.listener(Self::right))
        .on_action(cx.listener(Self::select_left))
        .on_action(cx.listener(Self::select_right))
        .on_action(cx.listener(Self::select_all))
        .on_action(cx.listener(Self::select_to_home))
        .on_action(cx.listener(Self::select_to_end))
        .on_action(cx.listener(Self::home))
        .on_action(cx.listener(Self::end))
        .on_action(cx.listener(Self::show_character_palette))
        .on_action(cx.listener(Self::copy))
        .on_action(cx.listener(Self::paste))
        .on_action(cx.listener(Self::cut))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|view, event: &MouseDownEvent, cx| {
                if event.click_count == 2 {
                    view.select_all(&SelectAll, cx);
                }
            }),
        )
        .on_key_down(cx.listener(Self::on_key_down_for_blink_cursor))
        .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
        .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
        .h_flex()
        .w_full()
        .h(cx.theme().input_height)
        .overflow_hidden()
        .child(
            div()
                .id("TextElement")
                .py_px()
                .px_1()
                .flex_grow()
                .overflow_x_hidden()
                .cursor_text()
                .on_drag((), |_, _point, cx| cx.new_view(|_cx| EmptyView)) // This makes sure the events from the field are preferred.
                .child(element::TextElement {
                    field: cx.view().clone(),
                }),
        )
    }
}
