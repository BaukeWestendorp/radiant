function on_update(cx)
    local speed = 4.0
    local amplitude = 1.0
    local width = 1.0
    local base = 0.0

    local fixture_ids = cx.fixture_ids
    local t = cx.time_seconds
    for i, fixture_id in ipairs(fixture_ids) do
        local phase = (i - 1) * (2 * math.pi * width) / #fixture_ids
        local wave = math.sin(speed * t + phase)
        local intensity = base + ((wave * 0.5 + 0.5) * amplitude)
        cx:set_parameter(fixture_id, Parameter.dimmer(intensity))
    end
end
