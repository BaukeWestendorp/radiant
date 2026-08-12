use rd_ui::{
    comp::{Section, article, h1, h2, h3, h4, h5, h6, link, sub},
    gpui::{Window, prelude::*},
    v_flex,
};

pub struct TypographyPreview {}

impl TypographyPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TypographyPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .p_2()
            .child(
                Section::new(
                    "Headings",
                    article()
                        .child(h1("Heading 1", cx))
                        .child(h2("Heading 2", cx))
                        .child(h3("Heading 3", cx))
                        .child(h4("Heading 4", cx))
                        .child(h5("Heading 5", cx))
                        .child(h6("Heading 6", cx)),
                )
                .size_full(),
            )
            .child(
                Section::new(
                    "Body",
                    article()
                        .child("Primary body text used for regular interface text.")
                        .child(sub("Secondary supporting text for context and help.", cx))
                        .child(link(
                            "Visit a cool site!",
                            "https://baukewestendorp.nl/",
                            window,
                            cx,
                        )),
                )
                .size_full(),
            )
    }
}
