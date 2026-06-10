// fn render_feature_group(
//     &self,
//     feature_name: &Name,
//     attributes: &[(AttributeName, String)],
//     window: &Window,
//     cx: &Context<Self>,
// ) -> impl IntoElement {
//     let header = div()
//         .font_weight(FontWeight::BOLD)
//         .text_center()
//         .bg(cx.theme().bg_tertiary)
//         .border_b_1()
//         .border_color(cx.theme().border_tertiary)
//         .child(feature_name.to_string());

//     let items = attributes.iter().filter_map(|(attribute_name, pretty_name)| {
//         if *attribute_name == AttributeName::NoFeature {
//             return None;
//         }
//         Some(self.render_logical_channel_item(
//             attribute_name.clone(),
//             pretty_name.clone(),
//             window,
//             cx,
//         ))
//     });

//     v_flex()
//         // FIXME: Use cell size instead.
//         .w(px(80.0))
//         .h_full()
//         .text_sm()
//         .border_color(cx.theme().border_primary)
//         .border_r_1()
//         .bg(cx.theme().bg_primary)
//         .child(header)
//         .child(
//             div().size_full().id(feature_name.to_string()).overflow_y_scroll().children(items),
//         )
// }

// fn render_feature_item(
//     &self,
//     feature_group: &FeatureGroup,
//     feature: &Feature,
//     _window: &Window,
//     cx: &Context<Self>,
// ) -> impl IntoElement {
//     let feature_path = NodePath::new(feature_group.name().clone()).join(feature.name().clone());

// let is_selected =
//     self.fixture.read(cx).as_ref().is_some_and(|(_, path)| *path == feature_path);
// let (bg, border) = if is_selected {
//     (cx.theme().bg_selected, cx.theme().border_selected)
// } else {
//     (cx.theme().bg_secondary, cx.theme().border_secondary)
// };

// div()
//     .id(feature_path.to_string())
//     .text_center()
//     .bg(bg)
//     .border_b_1()
//     .border_color(border)
//     .when(cx.theme().shadow, |e| e.shadow_xs())
//     .hover(|e| e.bg(bg.hover()))
//     .active(|e| e.bg(bg.active()).top(cx.theme().button_depression))
//     .child(feature.name().to_string())
//     .on_click(cx.listener(move |this, _, _, cx| {
//         let snapshot = EngineManager::snapshot(cx);
//         let Some(fixture) = snapshot.selection().fixtures(snapshot.patch()).next() else {
//             return;
//         };

//         this.fixture.write(cx, Some((fixture.clone(), feature_path.clone())));
//     }))
// }
// }
