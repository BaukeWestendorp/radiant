use rd_ui::{
    comp::stateful::{Tab, Tabs, TabsDirection},
    gpui::{Entity, Window, prelude::*},
};

use crate::{
    layout::PreviewPage,
    stories::{
        actions::button::ButtonPreview,
        data::table::TablePreview,
        examples::form::FormPreview,
        forms::{checkbox::CheckboxPreview, input::InputPreview, picker::PickerPreview},
        foundations::{theme::ThemePreview, typography::TypographyPreview},
        navigation::tabs::TabsPreview,
    },
};

pub fn build_root_tabs<V>(window: &mut Window, cx: &mut Context<V>) -> Entity<Tabs>
where
    V: 'static,
{
    let foundations_tabs = cx.new(|cx| build_foundations_tabs(window, cx));
    let foundations_page = cx.new(|_| {
        PreviewPage::new("Foundations", foundations_tabs)
            .with_description("Design primitives and building blocks.")
    });

    let forms_tabs = cx.new(|cx| build_forms_tabs(window, cx));
    let forms_page = cx.new(|_| {
        PreviewPage::new("Forms", forms_tabs).with_description("Form controls and states.")
    });

    let actions_tabs = cx.new(|cx| build_actions_tabs(window, cx));
    let actions_page = cx.new(|_| {
        PreviewPage::new("Actions", actions_tabs)
            .with_description("Interactive controls that trigger behavior.")
    });

    let navigation_tabs = cx.new(|cx| build_navigation_tabs(window, cx));
    let navigation_page = cx.new(|_| {
        PreviewPage::new("Navigation", navigation_tabs)
            .with_description("Navigation-related elements.")
    });

    let data_tabs = cx.new(|cx| build_data_tabs(window, cx));
    let data_page = cx.new(|_| {
        PreviewPage::new("Data", data_tabs).with_description("Data presentation and interaction.")
    });

    let examples_tabs = cx.new(|cx| build_examples_tabs(window, cx));
    let examples_page = cx.new(|_| {
        PreviewPage::new("Examples", examples_tabs)
            .with_description("Previews that show multiple components together.")
    });

    cx.new(|cx| {
        Tabs::new("preview_categories", window, cx)
            .with_direction(TabsDirection::Vertical)
            .with_selected(0)
            .with_tab(Tab::new("Foundations", cx).with_content(foundations_page, cx))
            .with_tab(Tab::new("Forms", cx).with_content(forms_page, cx))
            .with_tab(Tab::new("Actions", cx).with_content(actions_page, cx))
            .with_tab(Tab::new("Navigation", cx).with_content(navigation_page, cx))
            .with_tab(Tab::new("Data", cx).with_content(data_page, cx))
            .with_tab(Tab::new("Examples", cx).with_content(examples_page, cx))
    })
}

fn build_foundations_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("foundations_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(
            Tab::new("Typography", cx)
                .with_content(cx.new(|cx| TypographyPreview::new(window, cx)), cx),
        )
        .with_tab(
            Tab::new("Theme", cx).with_content(cx.new(|cx| ThemePreview::new(window, cx)), cx),
        )
}

fn build_forms_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("forms_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(
            Tab::new("Input", cx).with_content(cx.new(|cx| InputPreview::new(window, cx)), cx),
        )
        .with_tab(
            Tab::new("Checkbox", cx)
                .with_content(cx.new(|cx| CheckboxPreview::new(window, cx)), cx),
        )
        .with_tab(
            Tab::new("Picker", cx).with_content(cx.new(|cx| PickerPreview::new(window, cx)), cx),
        )
}

fn build_actions_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("actions_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(
            Tab::new("Button", cx).with_content(cx.new(|cx| ButtonPreview::new(window, cx)), cx),
        )
}

fn build_navigation_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("navigation_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(Tab::new("Tabs", cx).with_content(cx.new(|cx| TabsPreview::new(window, cx)), cx))
}

fn build_data_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("data_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(
            Tab::new("Table", cx).with_content(cx.new(|cx| TablePreview::new(window, cx)), cx),
        )
}

fn build_examples_tabs(window: &mut Window, cx: &mut Context<Tabs>) -> Tabs {
    Tabs::new("examples_tabs", window, cx)
        .with_direction(TabsDirection::Vertical)
        .with_selected(0)
        .with_tab(Tab::new("Form", cx).with_content(cx.new(|cx| FormPreview::new(window, cx)), cx))
}
