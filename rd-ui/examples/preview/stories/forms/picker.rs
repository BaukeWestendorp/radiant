use rd_ui::{
    comp::{
        Disableable, IconVariant, Label, Labelled,
        stateful::{Picker, PickerItem, PickerKind},
    },
    gpui::{Entity, Window, prelude::*},
    v_flex,
};

pub struct PickerPreview {
    picker_inline: Entity<Picker<NoBoundEnum>>,
    picker_dropdown: Entity<Picker<ImplDisplayEnum>>,
    picker_dropdown_searchable: Entity<Picker<LongLabelEnum>>,
    picker_disabled: Entity<Picker<ImplDisplayEnum>>,
}

impl PickerPreview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            picker_inline: cx.new(|cx| {
                Picker::new(
                    "picker-inline-selected",
                    vec![
                        PickerItem::new("Alpha", NoBoundEnum::Alpha).with_icon(IconVariant::Folder),
                        PickerItem::new("Beta", NoBoundEnum::Beta).with_icon(IconVariant::Star),
                        PickerItem::new("Gamma", NoBoundEnum::Gamma).with_icon(IconVariant::Drama),
                    ],
                    window,
                    cx,
                )
                .with_kind(PickerKind::Inline)
                .with_selection(Some(1))
            }),
            picker_dropdown: cx.new(|cx| {
                Picker::new(
                    "picker-dropdown",
                    vec![
                        PickerItem::from(ImplDisplayEnum::One),
                        PickerItem::from(ImplDisplayEnum::Two),
                        PickerItem::from(ImplDisplayEnum::Three),
                    ],
                    window,
                    cx,
                )
                .with_kind(PickerKind::Dropdown { searchable: false })
                .with_selection(Some(2))
            }),
            picker_dropdown_searchable: cx.new(|cx| {
                Picker::new(
                    "picker-dropdown-searchable",
                    vec![
                        PickerItem::from(LongLabelEnum::FirstVeryLongValue)
                            .with_icon(IconVariant::GitCompare),
                        PickerItem::from(LongLabelEnum::SecondVeryLongValue)
                            .with_icon(IconVariant::Folder),
                        PickerItem::from(LongLabelEnum::ThirdVeryLongValue)
                            .with_icon(IconVariant::Star),
                        PickerItem::from(LongLabelEnum::FourthVeryLongValue)
                            .with_icon(IconVariant::Check),
                        PickerItem::from(LongLabelEnum::FifthVeryLongValue)
                            .with_icon(IconVariant::Heart),
                        PickerItem::from(LongLabelEnum::SixthVeryLongValue)
                            .with_icon(IconVariant::Book),
                        PickerItem::from(LongLabelEnum::SeventhVeryLongValue)
                            .with_icon(IconVariant::AudioLines),
                        PickerItem::from(LongLabelEnum::EighthVeryLongValue)
                            .with_icon(IconVariant::Music),
                        PickerItem::from(LongLabelEnum::NinthVeryLongValue)
                            .with_icon(IconVariant::ChessKnight),
                    ],
                    window,
                    cx,
                )
                .with_kind(PickerKind::Dropdown { searchable: true })
            }),
            picker_disabled: cx.new(|cx| {
                Picker::new(
                    "picker-disabled",
                    vec![
                        PickerItem::from(ImplDisplayEnum::One),
                        PickerItem::from(ImplDisplayEnum::Two),
                        PickerItem::from(ImplDisplayEnum::Three),
                    ],
                    window,
                    cx,
                )
                .with_kind(PickerKind::Inline)
                .with_selection(Some(2))
                .with_disabled(true, cx)
            }),
        }
    }
}

impl Render for PickerPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_2()
            .child(Label::new("inline", self.picker_inline.clone()).with_label("Inline"))
            .child(Label::new("dropdown", self.picker_dropdown.clone()).with_label("Dropdown"))
            .child(
                Label::new("dropdown_searchable", self.picker_dropdown_searchable.clone())
                    .with_label("Dropdown & Searchable"),
            )
            .child(Label::new("disabled", self.picker_disabled.clone()).with_label("Disabled"))
    }
}

#[derive(Clone)]
enum NoBoundEnum {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Clone)]
enum ImplDisplayEnum {
    One,
    Two,
    Three,
}

#[derive(Clone)]
enum LongLabelEnum {
    FirstVeryLongValue,
    SecondVeryLongValue,
    ThirdVeryLongValue,
    FourthVeryLongValue,
    FifthVeryLongValue,
    SixthVeryLongValue,
    SeventhVeryLongValue,
    EighthVeryLongValue,
    NinthVeryLongValue,
}

impl std::fmt::Display for ImplDisplayEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImplDisplayEnum::One => write!(f, "One"),
            ImplDisplayEnum::Two => write!(f, "Two"),
            ImplDisplayEnum::Three => write!(f, "Three"),
        }
    }
}

impl std::fmt::Display for LongLabelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LongLabelEnum::FirstVeryLongValue => write!(f, "First very long value"),
            LongLabelEnum::SecondVeryLongValue => write!(f, "Second very long value"),
            LongLabelEnum::ThirdVeryLongValue => write!(f, "Third very long value"),
            LongLabelEnum::FourthVeryLongValue => write!(f, "Fourth very long value"),
            LongLabelEnum::FifthVeryLongValue => write!(f, "Fifth very long value"),
            LongLabelEnum::SixthVeryLongValue => write!(f, "Sixth very long value"),
            LongLabelEnum::SeventhVeryLongValue => write!(f, "Seventh very long value"),
            LongLabelEnum::EighthVeryLongValue => write!(f, "Eighth very long value"),
            LongLabelEnum::NinthVeryLongValue => write!(f, "Ninth very long value"),
        }
    }
}
