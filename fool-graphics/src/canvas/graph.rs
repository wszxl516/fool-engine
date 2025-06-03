use super::{Affine, FontManager, SceneNode, Style};
pub use vello::Scene;
#[derive(Debug, Default, Clone)]
pub struct SceneGraph {
    pub root: SceneNode,
    pub style: Style,
    pub font_mgr: FontManager,
}
impl SceneGraph {
    pub fn center_with_screen_size(&mut self, w: f64, h: f64) {
        let tr = Affine::translate((w / 2.0, h / 2.0));
        let style = self.root.style.clone().with_translation(tr);
        self.style = style;
    }
    pub fn set_root(&mut self, root: SceneNode) {
        self.root.children.clear();
        self.root.add_child(&root);
    }
    pub fn draw(&self, scene: &mut Scene) {
        self.draw_node(&self.root, scene, &self.style);
    }

    fn draw_node(&self, node: &SceneNode, scene: &mut Scene, parent_style: &Style) {
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
