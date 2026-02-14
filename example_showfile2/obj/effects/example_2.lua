function on_update(cx)
    local fixture_ids = cx.fixture_ids
    for _, fixture_id in ipairs(fixture_ids) do
        cx.set_parameter(fixture_id, Parameter.dimmer(cx.time_seconds % 2.0))
        cx.set_parameter(fixture_id, Parameter.rgb(0.0, 1.0, 0.0))
    end
end
