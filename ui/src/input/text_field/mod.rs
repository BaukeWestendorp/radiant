use std::ops::{Range, Sub};

use crate::theme::ActiveTheme;
use gpui::*;
use prelude::FluentBuilder;
use text_element::TextElement;

mod text_element;

const KEY_CONTEXT: &str = "Input";

actions!(input, [Left, Right, Backspace, Delete, Enter]);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", Left, Some(KEY_CONTEXT)),
        KeyBinding::new("right", Right, Some(KEY_CONTEXT)),
        KeyBinding::new("backspace", Backspace, Some(KEY_CONTEXT)),
        KeyBinding::new("delete", Delete, Some(KEY_CONTEXT)),
        KeyBinding::new("enter", Enter, Some(KEY_CONTEXT)),
    ]);
}

pub struct TextField {
    text: SharedString,
    placeholder: SharedString,

    pub(super) utf16_cursor_offset: usize,
    pub(super) utf16_selection: Option<Range<usize>>,

    pub(super) focus_handle: FocusHandle,
}

impl TextField {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            text: "".into(),
            placeholder: "".into(),
            utf16_cursor_offset: 0,
            utf16_selection: None,
            focus_handle,
        }
    }

    pub fn set_text(&mut self, text: SharedString) {
        self.text = text;
    }

    pub fn text(&self) -> &SharedString {
        &self.text
    }

    pub fn set_placeholder(&mut self, placeholder: SharedString) {
        self.placeholder = placeholder;
    }

    pub fn placeholder(&self) -> &SharedString {
        &self.placeholder
    }

    fn move_to(&mut self, utf16_offset: usize, cx: &mut Context<Self>) {
        self.utf16_cursor_offset = utf16_offset.clamp(0, self.text.len());
        cx.notify();
    }

    fn select(&mut self, utf16_range: Range<usize>) {
        self.utf16_selection = Some(utf16_range);
    }

    fn unselect(&mut self) {
        self.utf16_selection = None;
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

    fn cursor_char_offset(&self) -> usize {
        self.char_offset_from_utf16(self.utf16_cursor_offset)
    }

    fn cursor_char_range(&self) -> Range<usize> {
        let char_offset = self.cursor_char_offset();
        char_offset..char_offset
    }
}

impl TextField {
    fn handle_left(&mut self, _: &Left, _window: &mut Window, cx: &mut Context<Self>) {
        let new_char_offset = self.cursor_char_offset().saturating_sub(1);
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    fn handle_right(&mut self, _: &Right, _window: &mut Window, cx: &mut Context<Self>) {
        let new_char_offset = self.cursor_char_offset().saturating_add(1);
        let new_utf16_offset = self.char_offset_to_utf16(new_char_offset);
        self.move_to(new_utf16_offset, cx);
        cx.notify();
    }

    fn handle_backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        let utf16_range = self.utf16_cursor_offset.saturating_sub(1)..self.utf16_cursor_offset;
        self.replace_text_in_range(Some(utf16_range), "", window, cx);
    }

    fn handle_delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        let utf16_range = self.utf16_cursor_offset..self.utf16_cursor_offset.saturating_add(1);
        self.replace_text_in_range(Some(utf16_range), "", window, cx);
    }

    fn handle_enter(&mut self, _: &Enter, _window: &mut Window, _cx: &mut Context<Self>) {}
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
        unimplemented!("text marking is not implemented")
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        unimplemented!("text marking is not implemented")
    }

    fn replace_text_in_range(
        &mut self,
        utf16_range: Option<std::ops::Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let char_range = match utf16_range {
            Some(utf16_range) => {
                self.char_offset_from_utf16(utf16_range.start)
                    ..self.char_offset_from_utf16(utf16_range.end)
            }
            _ => self.cursor_char_range(),
        };

        let new_text =
            self.text[0..char_range.start].to_owned() + text + &self.text[char_range.end..];

        // Move the cursor to the end of the inserted text.
        self.utf16_cursor_offset = self.char_offset_to_utf16(char_range.start) + text.len();

        self.set_text(new_text.into());

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
        unimplemented!("text marking is not implemented")
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
        _point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        todo!()
    }
}

impl Render for TextField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);

        div()
            .id("input")
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .bg(cx.theme().background)
            .p_1()
            .border_1()
            .border_color(cx.theme().border_color)
            .when(focused, |e| {
                e.bg(cx.theme().background_focused).border_color(cx.theme().border_color_focused)
            })
            .rounded(cx.theme().radius)
            .child(TextElement::new(cx.entity().clone()))
            .on_action(cx.listener(Self::handle_left))
            .on_action(cx.listener(Self::handle_right))
            .on_action(cx.listener(Self::handle_backspace))
            .on_action(cx.listener(Self::handle_delete))
            .on_action(cx.listener(Self::handle_enter))
    }
}
