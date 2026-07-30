use rd_ui::{
    AutoInput, EnumerableValue, Input, InputDelegate, InputEvent, InputState, Labelled,
    LayoutDirection, Slider, SliderValue,
    gpui::{App, Entity, FocusHandle, Focusable, Window, prelude::*},
    h_flex, v_flex,
};

use crate::{NetId, PortAddress, SubNetId, UniverseId};
pub struct PortAddressInput {
    absolute: Entity<InputState<Slider<u16>>>,
    net: Entity<InputState<Slider<NetId>>>,
    sub: Entity<InputState<Slider<SubNetId>>>,
    uni: Entity<InputState<Slider<UniverseId>>>,

    focus_handle: FocusHandle,
}

impl PortAddressInput {
    pub fn new(
        initial_value: PortAddress,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<InputState<Self>>,
    ) -> Self {
        let absolute = u16::build_input(initial_value.as_u16(), window, cx);
        let net = NetId::build_input(initial_value.net(), window, cx);
        let sub = SubNetId::build_input(initial_value.sub_net(), window, cx);
        let uni = UniverseId::build_input(initial_value.universe(), window, cx);

        cx.subscribe(&absolute, move |this, _, event, cx| {
            cx.emit(match event {
                InputEvent::Focus => InputEvent::Focus,
                InputEvent::Blur => InputEvent::Blur,
                InputEvent::Submit(_) => InputEvent::Submit(this.value_or_default(cx)),
                InputEvent::Change(_) => InputEvent::Change(this.value_or_default(cx)),
            });

            match event {
                InputEvent::Change(abs) | InputEvent::Submit(abs) => {
                    let Ok(pa) = PortAddress::from_absolute(*abs) else { return };

                    if this.net.read(cx).value(cx) != Some(pa.net()) {
                        this.net.update(cx, |net, cx| net.set_value(Some(pa.net()), cx));
                    }
                    if this.sub.read(cx).value(cx) != Some(pa.sub_net()) {
                        this.sub
                            .update(cx, |sub_net, cx| sub_net.set_value(Some(pa.sub_net()), cx));
                    }
                    if this.uni.read(cx).value(cx) != Some(pa.universe()) {
                        this.uni
                            .update(cx, |universe, cx| universe.set_value(Some(pa.universe()), cx));
                    }
                }
                _ => {}
            }
        })
        .detach();

        cx.subscribe(&net, move |this, _, event, cx| match event {
            InputEvent::Change(net) | InputEvent::Submit(net) => {
                let Some(sub) = this.sub.read(cx).value(cx).clone() else { return };
                let Some(uni) = this.uni.read(cx).value(cx).clone() else { return };
                let pa = PortAddress::new(*net, sub, uni);
                let new_abs = pa.as_u16();

                if this.absolute.read(cx).value(cx) != Some(new_abs) {
                    this.absolute.update(cx, |abs, cx| abs.set_value(Some(new_abs), cx));
                }
            }
            _ => {}
        })
        .detach();

        cx.subscribe(&sub, move |this, _, event, cx| match event {
            InputEvent::Change(sub) | InputEvent::Submit(sub) => {
                let Some(net) = this.net.read(cx).value(cx).clone() else { return };
                let Some(uni) = this.uni.read(cx).value(cx).clone() else { return };
                let pa = PortAddress::new(net, *sub, uni);
                let new_abs = pa.as_u16();

                if this.absolute.read(cx).value(cx) != Some(new_abs) {
                    this.absolute.update(cx, |abs, cx| abs.set_value(Some(new_abs), cx));
                }
            }
            _ => {}
        })
        .detach();

        cx.subscribe(&uni, move |this, _, event, cx| match event {
            InputEvent::Change(uni) | InputEvent::Submit(uni) => {
                let Some(net) = this.net.read(cx).value(cx).clone() else { return };
                let Some(sub) = this.sub.read(cx).value(cx).clone() else { return };
                let pa = PortAddress::new(net, sub, *uni);
                let new_abs = pa.as_u16();

                if this.absolute.read(cx).value(cx) != Some(new_abs) {
                    this.absolute.update(cx, |abs, cx| abs.set_value(Some(new_abs), cx));
                }
            }
            _ => {}
        })
        .detach();

        Self { absolute, net, sub, uni, focus_handle }
    }
}

impl Focusable for PortAddressInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl InputDelegate for PortAddressInput {
    type Value = PortAddress;

    fn new_element(
        state: Entity<InputState<Self>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let delegate = state.read(cx).delegate();

        v_flex()
            .w_full()
            .gap_2()
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(Labelled::new(
                        "Net",
                        Input::new(delegate.net.clone()).into_any_element(),
                        LayoutDirection::Vertical,
                    ))
                    .child(Labelled::new(
                        "Sub-Net",
                        Input::new(delegate.sub.clone()).into_any_element(),
                        LayoutDirection::Vertical,
                    ))
                    .child(Labelled::new(
                        "Universe",
                        Input::new(delegate.uni.clone()).into_any_element(),
                        LayoutDirection::Vertical,
                    )),
            )
            .child(Labelled::new(
                "Absolute",
                Input::new(delegate.absolute.clone()).into_any_element(),
                LayoutDirection::Vertical,
            ))
    }

    fn value_or_default(&self, cx: &App) -> Self::Value {
        let absolute_val = self.absolute.read(cx).delegate().value_or_default(cx);
        PortAddress::from_absolute(absolute_val).unwrap_or_default()
    }
}

impl AutoInput for PortAddress {
    type Delegate = PortAddressInput;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let input = PortAddressInput::new(initial_value, cx.focus_handle(), window, cx);
            InputState::new(input, window, cx)
        })
    }
}

impl EnumerableValue for PortAddress {
    fn enumerated_value(&self, offset: usize) -> Self {
        let absolute = self.as_u16() as usize + offset;
        PortAddress::from_absolute(absolute as u16).unwrap_or_default()
    }
}

impl SliderValue for NetId {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        NetId(value as u8)
    }

    fn min_value() -> Option<Self> {
        Some(NetId::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(NetId::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(NetId(1))
    }
}

impl AutoInput for NetId {
    type Delegate = Slider<NetId>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl EnumerableValue for NetId {
    fn enumerated_value(&self, offset: usize) -> Self {
        let new_value = self.0 as usize + offset;
        NetId(new_value as u8)
    }
}

impl SliderValue for SubNetId {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        SubNetId(value as u8)
    }

    fn min_value() -> Option<Self> {
        Some(SubNetId::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(SubNetId::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(SubNetId(1))
    }
}

impl AutoInput for SubNetId {
    type Delegate = Slider<SubNetId>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl EnumerableValue for SubNetId {
    fn enumerated_value(&self, offset: usize) -> Self {
        let new_value = self.0 as usize + offset;
        SubNetId(new_value as u8)
    }
}

impl SliderValue for UniverseId {
    fn to_f64(&self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Self {
        UniverseId(value as u8)
    }

    fn min_value() -> Option<Self> {
        Some(UniverseId::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(UniverseId::MAX)
    }

    fn step_value() -> Option<Self> {
        Some(UniverseId(1))
    }
}

impl AutoInput for UniverseId {
    type Delegate = Slider<UniverseId>;

    fn build_input(
        initial_value: Self,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<InputState<Self::Delegate>> {
        cx.new(move |cx| {
            let slider =
                Slider::new(cx.focus_handle(), window, cx).with_value(Some(initial_value), cx);
            InputState::new(slider, window, cx)
        })
    }
}

impl EnumerableValue for UniverseId {
    fn enumerated_value(&self, offset: usize) -> Self {
        let new_value = self.0 as usize + offset;
        UniverseId(new_value as u8)
    }
}
