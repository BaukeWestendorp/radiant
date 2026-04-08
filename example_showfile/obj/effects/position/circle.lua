function on_update(cx)
    local speed = (cx.options.speed or 1) * 2 * math.pi;
    local angle = cx.options.angle or 25
    local delta_angle = cx.options.delta_angle or 45

    local t = cx.time_seconds

    local fixture_ids = cx.fixture_ids
    for i, fixture_id in ipairs(fixture_ids) do
        local pan = math.cos(speed * t + math.rad(delta_angle) * i) * angle
        local tilt = math.sin(speed * t + math.rad(delta_angle) * i) * angle

        cx:set_parameter(fixture_id, Parameter.dimmer(Value.clamped(1)))
        cx:set_parameter(fixture_id, Parameter.raw("Zoom", Value.clamped(0)))
        cx:set_parameter(fixture_id, Parameter.pan(Value.physical(pan)))
        cx:set_parameter(fixture_id, Parameter.tilt(Value.physical(tilt)))
    end
end
