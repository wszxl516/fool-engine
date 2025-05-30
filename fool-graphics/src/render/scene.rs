use vello::Scene;

#[derive(Clone, Default)]
pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    pub fn clear(&mut self) {
        self.scene.reset();
    }
    pub fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn build(&self) -> Scene {
        self.scene.clone()
    }
}
