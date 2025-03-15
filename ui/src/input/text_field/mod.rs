use crate::theme::ActiveTheme;
use gpui::*;
use prelude::FluentBuilder;
use text_element::TextElement;

mod text_element;

const KEY_CONTEXT: &str = "Input";

pub struct TextField {
    pub(super) focus_handle: FocusHandle,
    text: SharedString,
    placeholder: SharedString,
}

impl TextField {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        Self { focus_handle, text: "".into(), placeholder: "".into() }
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
}

impl EntityInputHandler for TextField {
    fn text_for_range(
        &mut self,
        _range: std::ops::Range<usize>,
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
        range: Option<std::ops::Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match range {
            Some(range) => {
                let (start, end) = self.text.split_at(range.start);
                let (_mid, end) = end.split_at(range.end);
                self.text = format!("{start}{text}{end}").into();
            }
            None => {
                self.text = text.to_string().into();
            }
        }
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range: Option<std::ops::Range<usize>>,
        _new_text: &str,
        _new_selected_range: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        unimplemented!("text marking is not implemented")
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
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
    }
}
