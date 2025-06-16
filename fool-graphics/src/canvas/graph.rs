use super::ImageManager;
use super::{Affine, FontManager, SceneNode, Style};
pub use vello::Scene;

#[derive(Debug, Default, Clone)]
pub struct SceneGraph {
    pub root: SceneNode,
    pub style: Style,
    pub font_mgr: FontManager,
    pub img_mgr: ImageManager,
    pub default_size: (f64, f64),
    pub scale: Option<f64>,
}
impl SceneGraph {
    pub fn center_with_screen_size(&mut self, w: f64, h: f64) {
        self.default_size = (w, h);
    }
    pub fn reset(&mut self) {
        self.root.clear_children();
    }
    pub fn set_scale(&mut self, scale: Option<f64>) {
        self.scale = scale;
    }

    pub fn set_root(&mut self, root: SceneNode) {
        self.root.children.clear();
        self.root.add_child(&root);
    }
    pub fn draw(&self, scene: &mut Scene) -> anyhow::Result<()> {
        let mut style = self.style.clone();
        let scale = self.scale.unwrap_or(1.0);
        let (win_w, win_h) = (self.default_size.0, self.default_size.1);
        let scaling = Affine::scale(scale);
        let to_screen_center = Affine::translate((win_w / 2.0, win_h / 2.0));
        let transform = to_screen_center * scaling;
        style.translation = transform;
        self.draw_node(&self.root, scene, &style)
    }

    fn draw_node(
        &self,
        node: &SceneNode,
        scene: &mut Scene,
        parent_style: &Style,
    ) -> anyhow::Result<()> {
        let mut current_style = parent_style.clone();
        if let Some(drawable) = &node.drawable {
            let d = drawable.build(&node.style);
            current_style = if node.apply_parent_style {
                parent_style * &d.style
            } else {
                d.style.clone()
            };
            d.drawable.draw(
                scene,
                &current_style,
                self.font_mgr.clone(),
                self.img_mgr.clone(),
            )?;
        }
        let mut children_refs: Vec<&SceneNode> = node.children.iter().collect();
        children_refs.sort_by_key(|c| c.style.z_index);
        for child in &children_refs {
            self.draw_node(child, scene, &current_style)?;
        }
        Ok(())
    }
}
