use rapier2d::prelude::*;
mod event;
mod hooks;
pub mod types;
use mlua::{Function, UserData, UserDataMethods, Value};
use rapier2d::na::Vector2;
use types::{BodyData, LuaPoint, LuaRigidBody, LuaRigidBodyHandle, Shape2D};
pub struct Physics {
    pipeline: PhysicsPipeline,
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}

impl Physics {
    pub fn new(x: f32, y: f32) -> Self {
        let gravity = vector![x, y];
        let bodies = RigidBodySet::new();
        let colliders = ColliderSet::new();
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies,
            colliders,
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }
    pub fn update<E, H>(&mut self, event_handler: &E, physics_hooks: &H)
    where
        E: EventHandler,
        H: PhysicsHooks,
    {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            physics_hooks,
            event_handler,
        );
    }
    pub fn add_body(
        &mut self,
        user_data: u128,
        position: LuaPoint<f32>,
        shape: Shape2D,
        body_type: RigidBodyType,
        rotation: Option<Real>,
        linear_damping: Real,
        angular_damping: Real,
        gravity_scale: Real,
        additional_mass: Real,
        mass: Real,
        can_sleep: bool,
        sleeping: bool,

        restitution: Real,
        friction: Real,
        density: Real,
        is_sensor: bool,
        active_events: ActiveEvents,
        active_hooks: ActiveHooks,
    ) -> RigidBodyHandle {
        let p = Vector2::new(position.x, position.y);
        let body = RigidBodyBuilder::new(body_type)
            .translation(p)
            .additional_mass(additional_mass)
            .gravity_scale(gravity_scale)
            .can_sleep(can_sleep)
            .sleeping(sleeping)
            .linear_damping(linear_damping)
            .angular_damping(angular_damping)
            .user_data(user_data)
            .ccd_enabled(true);
        let body = if let Some(r) = rotation {
            body.rotation(r)
        } else {
            body.lock_rotations()
        }
        .build();

        let handle = self.bodies.insert(body);

        let collider = self
            .build_collider(shape)
            .restitution(restitution)
            .friction(friction)
            .density(density)
            .sensor(is_sensor)
            .user_data(user_data)
            .mass(mass)
            .active_events(active_events)
            .active_hooks(active_hooks)
            .build();

        self.colliders
            .insert_with_parent(collider, handle, &mut self.bodies);
        handle
    }
    pub fn build_collider(&self, shape: Shape2D) -> ColliderBuilder {
        match shape {
            Shape2D::Cuboid { width, height } => ColliderBuilder::cuboid(width / 2.0, height / 2.0),
            Shape2D::Ball { radius } => ColliderBuilder::ball(radius),
            Shape2D::CapsuleY { height, radius } => {
                ColliderBuilder::capsule_y(height / 2.0, radius)
            }
            Shape2D::CapsuleX { width, radius } => ColliderBuilder::capsule_x(width / 2.0, radius),
            Shape2D::RoundCuboid {
                width,
                height,
                border_radius,
            } => ColliderBuilder::round_cuboid(width / 2.0, height / 2.0, border_radius),
            Shape2D::Triangle { a, b, c } => {
                ColliderBuilder::triangle(a.into(), b.into(), c.into())
            }
            Shape2D::Convex { points } => {
                let points: Vec<Point<f32>> = points.into_iter().map(|p| p.into()).collect();
                ColliderBuilder::convex_hull(&points).expect("Convex hull generation failed")
            }
        }
    }
    pub fn cast_ray(
        &self,
        origin: Vector<Real>,
        dir: Vector<Real>,
        max_toi: Real,
    ) -> Option<(RigidBodyHandle, Real)> {
        let ray = Ray::new(origin.into(), dir.into());
        self.query_pipeline
            .cast_ray(
                &self.bodies,
                &self.colliders,
                &ray,
                max_toi,
                true,
                QueryFilter::default(),
            )
            .map(|(h, v)| (self.colliders.get(h).unwrap().parent().unwrap(), v))
    }
}

pub struct LuaPhysics {
    pub physics: Physics,
    pub collision_event: Option<Function>,
    pub contact_force_event: Option<Function>,
    pub event: event::LuaPhyEventHandler,
    pub hooks: hooks::LuaPhysicsHooks,
}

impl LuaPhysics {
    pub fn new(x: f32, y: f32) -> Self {
        let physics = Physics::new(x, y);
        Self {
            physics,
            collision_event: None,
            contact_force_event: None,
            event: Default::default(),
            hooks: Default::default(),
        }
    }

    pub fn get_bodies(&self) -> Vec<LuaRigidBody> {
        self.physics
            .bodies
            .iter()
            .map(|(_, body)| LuaRigidBody(body.clone()))
            .collect()
    }
}

impl UserData for LuaPhysics {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_gravity", |_lua, this, (x, y): (f32, f32)| {
            this.physics.gravity = vector![x, y];
            Ok(())
        });
        methods.add_method_mut("update", |_lua, this, ()| {
            this.physics.update(&this.event, &this.hooks);
            Ok(())
        });
        methods.add_method("event_update", |lua, this, (): ()| {
            let collision_event = this.collision_event.clone();
            if let Some(func) = collision_event {
                this.event.handle_collision_event(|event| {
                    match lua.scope(|scope| {
                        let args = scope.create_userdata(*event)?;
                        func.call::<()>(args)
                    }) {
                        Ok(_) => {}
                        Err(err) => log::error!("run callback collision_event failed: {}", err),
                    }
                })
            }
            let contact_force_event = this.contact_force_event.clone();
            if let Some(func) = contact_force_event {
                this.event.handle_contact_force_event(|event| {
                    match lua.scope(|scope| {
                        let args = scope.create_userdata(*event)?;
                        func.call::<()>(args)
                    }) {
                        Ok(_) => {}
                        Err(err) => log::error!("run callback contact_force_event failed: {}", err),
                    }
                })
            }
            this.event.reset_all();
            Ok(())
        });
        methods.add_method("get_bodies", |lua, this, ()| {
            let bodies = this.get_bodies();
            let table = lua.create_table()?;
            for (i, b) in bodies.into_iter().enumerate() {
                table.set(i + 1, b)?;
            }
            Ok(table)
        });
        methods.add_method("find_body", |_lua, this, handle: LuaRigidBodyHandle| {
            let body = this.physics.bodies.get(handle.0);
            let b = body.map(|b| LuaRigidBody(b.clone()));
            Ok(b)
        });
        methods.add_method_mut(
            "apply_force",
            |_, this, (handle, force): (LuaRigidBodyHandle, LuaPoint<f32>)| {
                if let Some(rb) = this.physics.bodies.get_mut(handle.0) {
                    rb.add_force(vector![force.x, force.y], true);
                }
                Ok(())
            },
        );
        methods.add_method_mut(
            "apply_impulse",
            |_, this, (handle, impulse): (LuaRigidBodyHandle, LuaPoint<f32>)| {
                if let Some(rb) = this.physics.bodies.get_mut(handle.0) {
                    rb.apply_impulse(vector![impulse.x, impulse.y], true);
                }
                Ok(())
            },
        );
        methods.add_method_mut("add_body", |_, this, data: BodyData| {
            let handle = this.physics.add_body(
                data.user_data,
                data.position,
                data.shape,
                data.body_type,
                data.rotation,        // rotation
                data.linear_damping,  // linear_damping
                data.angular_damping, // angular_damping
                data.gravity_scale,   // gravity_scale
                data.additional_mass, // additional_mass
                data.mass,            // mass
                data.can_sleep,       // can_sleep
                data.sleeping,        // sleeping
                data.restitution,     // restitution
                data.friction,        // friction
                data.density,         // density
                data.is_sensor,       // is_sensor
                data.active_events.into(),
                data.active_hooks.into(),
            );
            Ok(LuaRigidBodyHandle(handle))
        });
        methods.add_method_mut("remove_body", |_, this, handle: LuaRigidBodyHandle| {
            this.physics.bodies.remove(
                handle.0,
                &mut this.physics.island_manager,
                &mut this.physics.colliders,
                &mut this.physics.impulse_joints,
                &mut this.physics.multibody_joints,
                true,
            );
            Ok(())
        });
        methods.add_method_mut(
            "set_linvel",
            |_, this, (handle, linvel): (LuaRigidBodyHandle, LuaPoint<f32>)| {
                match this.physics.bodies.get_mut(handle.0) {
                    Some(b) => {
                        b.set_linvel(vector![linvel.x, linvel.y], true);
                    }
                    None => {}
                }
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_angvel",
            |_, this, (handle, angvel): (LuaRigidBodyHandle, f32)| {
                let body = this.physics.bodies.get_mut(handle.0);
                match body {
                    Some(b) => {
                        b.set_angvel(angvel, true);
                    }
                    None => {}
                }
                Ok(())
            },
        );
        methods.add_method_mut(
            "wake_up",
            |_, this, (handle, strong): (LuaRigidBodyHandle, bool)| {
                let body = this.physics.bodies.get_mut(handle.0);
                match body {
                    Some(b) => {
                        b.wake_up(strong);
                    }
                    None => {}
                }
                Ok(())
            },
        );
        methods.add_method_mut("sleep", |_, this, handle: LuaRigidBodyHandle| {
            let body = this.physics.bodies.get_mut(handle.0);
            match body {
                Some(b) => {
                    b.sleep();
                }
                None => {}
            }
            Ok(())
        });
        methods.add_method("is_sleeping", |_, this, handle: LuaRigidBodyHandle| {
            let body = this.physics.bodies.get(handle.0);
            match body {
                Some(b) => Ok(b.is_sleeping()),
                None => Ok(false),
            }
        });
        methods.add_method(
            "cast_ray",
            |lua, this, (origin, dir, max_toi): (LuaPoint<f32>, LuaPoint<f32>, f32)| match this
                .physics
                .cast_ray(
                    Vector::new(origin.x, origin.y).normalize(),
                    Vector::new(dir.x, dir.y).normalize(),
                    max_toi,
                ) {
                Some(res) => {
                    let table = lua.create_table()?;
                    table.set("handle", LuaRigidBodyHandle(res.0))?;
                    table.set("distance", res.1)?;
                    Ok(Value::Table(table))
                }
                None => Ok(Value::Nil),
            },
        );
        methods.add_method_mut(
            "register_collision_event_callback",
            |_lua, this, func: Function| {
                this.collision_event = Some(func);
                Ok(())
            },
        );
        methods.add_method_mut(
            "register_contact_force_event_callback",
            |_lua, this, func: Function| {
                this.contact_force_event = Some(func);
                Ok(())
            },
        );
        // intersection group methods
        methods.add_method("list_ignore_intersection_group", |_lua, this, ()| {
            Ok(this.hooks.list_ignore_intersection_group())
        });
        methods.add_method_mut(
            "add_ignore_intersection_group",
            |_lua, this, (name, group): (String, Vec<LuaRigidBodyHandle>)| {
                this.hooks.add_ignore_intersection_group(name, group);
                Ok(())
            },
        );
        methods.add_method_mut(
            "remove_ignore_intersection_group",
            |_lua, this, name: String| {
                this.hooks.remove_ignore_intersection_group(name);
                Ok(())
            },
        );

        // contact group methods
        methods.add_method("list_contact_filter_groups", |_lua, this, ()| {
            Ok(this.hooks.list_contact_filter_groups())
        });
        methods.add_method_mut(
            "add_contact_filter_group",
            |_lua, this, (name, group): (String, Vec<LuaRigidBodyHandle>)| {
                this.hooks.add_contact_filter_group(name, group);
                Ok(())
            },
        );
        methods.add_method_mut("remove_contact_filter_group", |_lua, this, name: String| {
            this.hooks.remove_contact_filter_group(name);
            Ok(())
        });
    }
}
