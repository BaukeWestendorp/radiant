local fixtures = radiant.group.fixtures;

function on_update(cx)
    for i, fixture in ipairs(fixtures) do
        local t = cx.global_time
        local phase = (fixture.id or i) * 0.13
        local fade = (math.sin((t * 0.4 + phase) * math.pi * 2.0) * 0.5) + 0.5
        local r = (1.0 - fade) * 0.0 + fade * 1.0
        local g = (1.0 - fade) * 1.0 + fade * 0.5
        local b = (1.0 - fade) * 1.0 + fade * 0.0

        radiant:set_attribute_value(tostring(fixture.id), "ColorAdd_R", r)
        radiant:set_attribute_value(tostring(fixture.id), "ColorAdd_G", g)
        radiant:set_attribute_value(tostring(fixture.id), "ColorAdd_B", b)
    end
end
