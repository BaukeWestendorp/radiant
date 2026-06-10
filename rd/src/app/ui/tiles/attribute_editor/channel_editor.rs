// use gpui::{Entity, FontWeight, Window, div, prelude::*, px};
// use rd_engine::{
//     gdtf::{attr::AttributeName, dmx::ChannelFunction},
//     patch::Fixture,
// };
// use rd_ui::{ActiveTheme, Tab, Tabs, TabsState, TabsVariant, h_flex};

// pub struct ChannelEditor {
//     selection: Entity<Option<(Fixture, AttributeName)>>,

//     cf_page_tabs: Entity<TabsState>,
// }

// impl ChannelEditor {
//     pub fn new(
//         selection: Entity<Option<(Fixture, AttributeName)>>,
//         cx: &mut Context<Self>,
//     ) -> Self {
//         Self {
//             selection,
//             cf_page_tabs: cx.new(|_| TabsState::new().with_selected(("cf-page-tab", 0usize))),
//         }
//     }

//     fn render_cf_page(
//         &self,
//         channel_functions: &[ChannelFunction],
//         cx: &Context<Self>,
//     ) -> impl IntoElement {
//         let encoders = channel_functions.iter().map(|cf| self.render_encoder(cf, cx));

//         h_flex()
//             .w_full()
//             // FIXME: Use cell size
//             .h(px(80.0))
//             .gap_1()
//             .p_1()
//             .children(encoders)
//     }

//     fn render_encoder(&self, cf: &ChannelFunction, cx: &Context<Self>) -> impl IntoElement {
//         let header = div()
//             .px_1()
//             .border_b_1()
//             .border_color(cx.theme().border_primary)
//             .font_weight(FontWeight::BOLD)
//             .child(cf.name().to_string());

//         div()
//             .size_full()
//             .bg(cx.theme().bg_secondary)
//             .border_1()
//             .border_color(cx.theme().border_primary)
//             .rounded(cx.theme().radius)
//             .child(header)
//     }
// }

// impl Render for ChannelEditor {
//     fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         let Some(logical_channel) = self
//             .selection
//             .read(cx)
//             .as_ref()
//             .and_then(|(f, attr)| f.dmx_mode().logical_channel(attr))
//         else {
//             return div();
//         };

//         let cf_page_tabs = logical_channel.channel_functions().chunks(4).enumerate().map(
//             |(ix, channel_functions)| {
//                 Tab::new(
//                     ("cf-page-tab", ix),
//                     format!("Page {}", ix + 1),
//                     self.render_cf_page(channel_functions, cx).into_any_element(),
//                 )
//             },
//         );

//         div().size_full().text_sm().child(
//             Tabs::new("cf-page-tabs", self.cf_page_tabs.clone(), TabsVariant::Top)
//                 .tabs(cf_page_tabs),
//         )

// let cf_picker = v_flex()
//     .w(px(120.0))
//     .h_full()
//     .child(
//         div()
//             .font_weight(FontWeight::BOLD)
//             .text_center()
//             .bg(cx.theme().bg_tertiary)
//             .border_b_1()
//             .border_color(cx.theme().border_tertiary)
//             .child(
// logical_channel
//     .attribute(gdtf)
//     .clone()
//     .map(|attr| attr.pretty_name().to_string())
//     .unwrap_or("Ch Fn".to_owned()),
//             ),
//     )
//     .child(
//         div().id("cf-picker").size_full().overflow_scroll().children(
//             logical_channel
//                 .channel_functions()
//                 .iter()
//                 .filter(|cf| {
//                     cf.attribute(gdtf)
//                         .is_some_and(|a| *a.name() != AttributeName::NoFeature)
//                 })
//                 .map({
//                     let selected_cf = self.selected_cf.clone();
//                     move |cf| {
//                         let is_selected = selected_cf
//                             .read(cx)
//                             .as_ref()
//                             .is_some_and(|sel| cf.name() == sel);
//                         let (bg, border) = if is_selected {
//                             (cx.theme().bg_selected, cx.theme().border_selected)
//                         } else {
//                             (cx.theme().bg_secondary, cx.theme().border_secondary)
//                         };

//                         let cf_name = cf.name().clone();
//                         div()
//                             .id(cf_name.to_string())
//                             .text_center()
//                             .bg(bg)
//                             .border_b_1()
//                             .border_color(border)
//                             .when(cx.theme().shadow, |e| e.shadow_xs())
//                             .hover(|e| e.bg(bg.hover()))
//                             .active(|e| e.bg(bg.active()).top(cx.theme().button_depression))
//                             .child(cf_name.to_string())
//                             .on_mouse_down(MouseButton::Left, {
//                                 let selected_cf = selected_cf.clone();
//                                 move |_, _, cx| {
//                                     selected_cf.write(cx, Some(cf_name.clone()));
//                                 }
//                             })
//                     }
//                 }),
//         ),
//     );

// let channel_sets = self
//     .selected_cf
//     .read(cx)
//     .as_ref()
//     .and_then(|cf_name| logical_channel.channel_function(&cf_name))
//     .map(|cf| cf.channel_sets())
//     .map(|channel_sets| {
//         div().flex().flex_wrap().flex_shrink_1().gap_2().p_2().children(
//             channel_sets
//                 .iter()
//                 .filter(|cs| cs.name().is_some_and(|name| !name.as_str().is_empty()))
//                 .map(|cs| {
//                     let cs_name = cs.name().unwrap().to_string();
//                     let value = ClampedValue::from(cs.dmx_from());
//                     let attribute_name = attribute_name.clone();
//                     Button::new(cs_name.clone()).child(cs_name.clone()).on_click(
//                         move |_, _, cx| {
//                             let fixtures = FixtureCollection::Multiple(
//                                 EngineManager::snapshot(cx)
//                                     .selection()
//                                     .fixture_ids()
//                                     .to_vec(),
//                             );

//                             EngineManager::execute(
//                                 cx,
//                                 Command::ProgrammerSet {
//                                     fixtures,
//                                     attribute: attribute_name.clone(),
//                                     value: AttributeValue::Clamped(value),
//                                 },
//                             );
//                         },
//                     )
//                 }),
//         )
//     });

// h_flex()
//     .size_full()
//     .text_sm()
//     .child(
//         div()
//             .h_full()
//             .border_r_1()
//             .border_color(cx.theme().border_primary)
//             .child(cf_picker),
//     )
//     .child(
//         Scrollable::new("channel-sets", self.channel_sets_scrollable.clone())
//             .children(channel_sets)
//             .size_full(),
//     )
//     }
// }
