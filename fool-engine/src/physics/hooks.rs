use std::collections::HashSet;

use egui::ahash::HashMap;
use rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use super::types::LuaRigidBodyHandle;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LuaPhysicsHooks {
    contact_pair_group: HashMap<String, HashSet<RigidBodyHandle>>,
    intersection_pair_group: HashMap<String, HashSet<RigidBodyHandle>>,
}
impl LuaPhysicsHooks {
    pub fn add_ignore_intersection_group(&mut self, name: String, group: Vec<LuaRigidBodyHandle>) {
        let group = group
            .iter()
            .map(|g| g.0)
            .collect::<HashSet<RigidBodyHandle>>();
        self.intersection_pair_group.insert(name, group);
    }
    pub fn add_contact_filter_group(&mut self, name: String, group: Vec<LuaRigidBodyHandle>) {
        let group = group
            .iter()
            .map(|g| g.0)
            .collect::<HashSet<RigidBodyHandle>>();
        self.contact_pair_group.insert(name, group);
    }

    pub fn list_ignore_intersection_group(&self) -> Vec<String> {
        self.intersection_pair_group.keys().cloned().collect()
    }
    pub fn list_contact_filter_groups(&self) -> Vec<String> {
        self.contact_pair_group.keys().cloned().collect()
    }

    pub fn remove_ignore_intersection_group(&mut self, name: String) {
        self.intersection_pair_group.remove(&name);
    }
    pub fn remove_contact_filter_group(&mut self, name: String) {
        self.contact_pair_group.remove(&name);
    }
}

impl PhysicsHooks for LuaPhysicsHooks {
    fn filter_contact_pair(&self, context: &PairFilterContext) -> Option<SolverFlags> {
        if self.contact_pair_group.is_empty() {
            return Some(SolverFlags::COMPUTE_IMPULSES);
        }
        if let (Some(h1), Some(h2)) = (context.rigid_body1, context.rigid_body2) {
            for (_, group) in &self.contact_pair_group {
                if group.contains(&h1) && group.contains(&h2) {
                    return Some(SolverFlags::empty());
                }
            }
        }
        Some(SolverFlags::COMPUTE_IMPULSES)
    }
    fn filter_intersection_pair(&self, context: &PairFilterContext) -> bool {
        if self.intersection_pair_group.is_empty() {
            return true;
        }
        println!("filter_intersection_pair");
        if let (Some(h1), Some(h2)) = (context.rigid_body1, context.rigid_body2) {
            for (_, group) in &self.intersection_pair_group {
                if group.contains(&h1) && group.contains(&h2) {
                    return false;
                }
            }
        }
        true
    }
    fn modify_solver_contacts(&self, _context: &mut ContactModificationContext) {}
}
