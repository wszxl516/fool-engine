use super::{FontManager, SceneNode, Style};
pub struct SceneGraph {
    pub root: SceneNode,
    pub font_mgr: FontManager,
}
impl SceneGraph {
    pub fn draw(&self, scene: &mut vello::Scene) {
        self.draw_node(&self.root, scene, &self.root.style);
    }

    fn draw_node(&self, node: &SceneNode, scene: &mut vello::Scene, parent_style: &Style) {
        let mut current_style = parent_style.clone();
        if let Some(drawable) = &node.drawable {
            let d = drawable.build(&node.style);
            current_style = if node.apply_parent_style {
                parent_style * &d.style
            } else {
                d.style.clone()
            };
            d.drawable
                .draw(scene, &current_style, self.font_mgr.clone());
        }
        let mut children_refs: Vec<&SceneNode> = node.children.iter().collect();
        children_refs.sort_by_key(|c| c.style.z_index);
        for child in &children_refs {
            self.draw_node(child, scene, &current_style);
        }
    }
}
