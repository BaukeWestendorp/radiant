function on_update(cx)
    local fixture_ids = cx.fixture_ids
    local t = cx.time_seconds
    for i, fixture_id in ipairs(fixture_ids) do
        local phase = (i / #fixture_ids) * math.pi * 2
        local speed = 0.7 + 0.5 * math.sin(t * 0.13 + phase)
        local dimmer = 0.6 + 0.4 * math.sin(t * speed + phase + math.sin(t * 0.23 + i))
        local r = 0.7 + 0.3 * math.sin(t * 1.2 + phase)
        local g = 0.7 + 0.3 * math.sin(t * 0.9 - phase + math.sin(t * 0.5 + i))
        local b = 0.7 + 0.3 * math.sin(t * 1.7 + phase * 1.5)
        local w = 0
        cx:set_parameter(fixture_id, Parameter.dimmer(dimmer))
        cx:set_parameter(fixture_id, Parameter.rgbw(r, g, b, w))
    end
end
