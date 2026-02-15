function on_update(cx)
    local speed = 1.0
    local duty_cycle = 0.1
    local width = 1.0
    local base = 0.01
    local amplitude = 1.0

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
