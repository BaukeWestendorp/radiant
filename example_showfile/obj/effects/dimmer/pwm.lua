function on_update(cx)
    local speed = cx.options.speed or 1.0
    local duty_cycle = cx.options.duty_cycle or 0.25
    local width = cx.options.width or 1.0
    local base = cx.options.base or 0.0
    local amplitude = cx.options.amplitude or 1.0

    local fixture_ids = cx.fixture_ids
    local t = cx.time_seconds
    for i, fixture_id in ipairs(fixture_ids) do
        local phase = (i - 1) * (1.0 / width) / #fixture_ids
        local pwm_time = (speed * t + phase) % 1.0
        local on = pwm_time < duty_cycle
        local intensity = base
        if on then
            intensity = base + amplitude
        end
        cx:set_parameter(fixture_id, Parameter.dimmer(intensity))
    end
end
