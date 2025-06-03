---@diagnostic disable-next-line: lowercase-global
physics = {}
physics.__index = physics

--- Physics
---@class physics
local physics_mt = {}
physics_mt.__index = physics_mt

---@class PhysicsBodyConfig
---@field user_data number
---@field position table
---@field position.x? number
---@field position.y? number
---@field shape table
---@field shape.Ball? table
---@field shape.Ball.radius? number
---@field shape.Cuboid? table
---@field shape.Cuboid.width? number
---@field shape.Cuboid.height? number
---@field body_type string @"Dynamic"|"Fixed"|"KinematicPositionBased"|"KinematicVelocityBased"
---@field rotation? number | nil ---nil for lock_rotations
---@field linear_damping? number
---@field angular_damping? number
---@field gravity_scale? number
---@field additional_mass? number
---@field mass? number
---@field can_sleep? boolean
---@field sleeping? boolean
---@field restitution? number
---@field friction? number
---@field density? number
---@field is_sensor? boolean
---@field active_events? string @"collision_events"|"contact_force_events"|"all"
---@field active_hooks? string @"filter_contact_pairs"|"filter_intersection_pair"|"all"
---
---@class LuaRigidBodyHandle

---@class LuaRigidBody
---@field pos Point --- x, y
---@field angle number
---@field linvel Point --- x, y
---@field angvel number
---@field mass number
---@field is_fixed boolean
---@field user_data number

---@param x_gravity_acceleration number
---@param y_gravity_acceleration number
---@return physics
---@diagnostic disable-next-line: lowercase-global
function physics_init(x_gravity_acceleration, y_gravity_acceleration)
    local self = setmetatable({}, physics_mt)
    -- Initialize physics here using the gravity values
    return self
end

---@param x number
---@param y number
function physics_mt:set_gravity(x, y)
    -- Set gravity logic
end

function physics_mt:update()
end

---@param config PhysicsBodyConfig
---@return LuaRigidBodyHandle
function physics_mt:add_body(config)
    return {} -- placeholder for a real body handle
end

---@param handle LuaRigidBodyHandle
function physics_mt:remove_body(handle)
    return {} -- placeholder for a real body handle
end

---@return LuaRigidBody[]
function physics_mt:get_bodies()
    return {} -- return list of rigid bodies
end

---@param handle LuaRigidBodyHandle
---@return LuaRigidBody|nil
function physics_mt:find_body(handle)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function physics_mt:apply_force(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function physics_mt:apply_impulse(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param linvel Point
function physics_mt:set_linvel(handle, linvel)
    -- set linear velocity
end

---@param handle LuaRigidBodyHandle
---@param angvel number
function physics_mt:set_angvel(handle, angvel)
    -- set angular velocity
end

---@param handle LuaRigidBodyHandle
---@param strong boolean
function physics_mt:wake_up(handle, strong)
    -- wake up a body
end

---@param handle LuaRigidBodyHandle
function physics_mt:sleep(handle)
    -- put body to sleep
end

---@param handle LuaRigidBodyHandle
---@return boolean
function physics_mt:is_sleeping(handle)
    return true -- or false
end

---@class CastRayRes
---@field handle LuaRigidBodyHandle
---@field distance number
---@param origin Point
---@param dir Point
---@param max_toi number
---@return CastRayRes
function physics_mt:cast_ray(origin, dir, max_toi)
    return {}
end

---only for sensor
---@return string[]
function physics_mt:list_ignore_intersection_group()
    return {}
end

---@param name  string
---@param group LuaRigidBodyHandle[]
function physics_mt:add_ignore_intersection_group(name, group)
    return {}
end

---@param name  string
function physics_mt:remove_ignore_intersection_group(name)

end

---only for no sensor
---@return string[]
function physics_mt:list_contact_filter_groups()
    return {}
end

---@param name  string
---@param group LuaRigidBodyHandle[]
function physics_mt:add_contact_filter_group(name, group)

end

---@param name  string
function physics_mt:remove_contact_filter_group(name)

end

---@class CollisionEvent
---@field b1 LuaRigidBodyHandle
---@field b2 LuaRigidBodyHandle
---@field sensor boolean
---@field removed boolean
-- Called every frame when collision state changes
-- One of Started or Stopped will be non-nil
---@class LuaCollisionEvent
---@field started? CollisionEvent
---@field stopped? CollisionEvent

---@param call_back fun(LuaCollisionEvent)
function physics_mt:register_collision_event_callback(call_back)
end

---@class LuaContactForceEvent
---@field b1 LuaRigidBodyHandle
---@field b2 LuaRigidBodyHandle
---@field dt number
---@field total_force_magnitude number
---@param call_back fun(LuaContactForceEvent)
function physics_mt:register_contact_force_event_callback(call_back)
end

---if add_body active_events not empty and call_back is registered
function physics_mt:event_update()

end