---@diagnostic disable-next-line: lowercase-global
Physics = {}
Physics.__index = Physics

--- Physics Engine
---@class Physics
local Physics = {}
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
---@return Physics
---@diagnostic disable-next-line: lowercase-global
function Physics.new(x_gravity_acceleration, y_gravity_acceleration)
    return {}
end

---@param x number
---@param y number
function Physics:set_gravity(x, y)
    -- Set gravity logic
end

function Physics:update()
end

---@param config PhysicsBodyConfig
---@return LuaRigidBodyHandle
function Physics:add_body(config)
    return {} -- placeholder for a real body handle
end

---@param handle LuaRigidBodyHandle
function Physics:remove_body(handle)
    return {} -- placeholder for a real body handle
end

---@return LuaRigidBody[]
function Physics:get_bodies()
    return {} -- return list of rigid bodies
end

---@param handle LuaRigidBodyHandle
---@return LuaRigidBody|nil
function Physics:find_body(handle)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function Physics:apply_force(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function Physics:apply_impulse(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param linvel Point
function Physics:set_linvel(handle, linvel)
    -- set linear velocity
end

---@param handle LuaRigidBodyHandle
---@param angvel number
function Physics:set_angvel(handle, angvel)
    -- set angular velocity
end

---@param handle LuaRigidBodyHandle
---@param strong boolean
function Physics:wake_up(handle, strong)
    -- wake up a body
end

---@param handle LuaRigidBodyHandle
function Physics:sleep(handle)
    -- put body to sleep
end

---@param handle LuaRigidBodyHandle
---@return boolean
function Physics:is_sleeping(handle)
    return true -- or false
end

---@class CastRayRes
---@field handle LuaRigidBodyHandle
---@field distance number
---@param origin Point
---@param dir Point
---@param max_toi number
---@return CastRayRes
function Physics:cast_ray(origin, dir, max_toi)
    return {}
end

---only for sensor
---@return string[]
function Physics:list_ignore_intersection_group()
    return {}
end

---@param name  string
---@param group LuaRigidBodyHandle[]
function Physics:add_ignore_intersection_group(name, group)
    return {}
end

---@param name  string
function Physics:remove_ignore_intersection_group(name)

end

---only for no sensor
---@return string[]
function Physics:list_contact_filter_groups()
    return {}
end

---@param name  string
---@param group LuaRigidBodyHandle[]
function Physics:add_contact_filter_group(name, group)

end

---@param name  string
function Physics:remove_contact_filter_group(name)

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
function Physics:register_collision_event_callback(call_back)
end

---@class LuaContactForceEvent
---@field b1 LuaRigidBodyHandle
---@field b2 LuaRigidBodyHandle
---@field dt number
---@field total_force_magnitude number
---@param call_back fun(LuaContactForceEvent)
function Physics:register_contact_force_event_callback(call_back)
end

---if add_body active_events not empty and call_back is registered
function Physics:event_update()

end