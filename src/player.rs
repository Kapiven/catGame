#[derive(Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, dir: f32) -> Self {
        Self { x, y, dir}
    }

    pub fn dir_vec(self) -> (f32, f32) {
        (self.dir.cos(), self.dir.sin())
    }

    pub fn plane_vec(self) -> (f32, f32) {
        // Plano de cámara (FOV ≈ 66°)
        (-self.dir.sin() * 0.66, self.dir.cos() * 0.66)
    }
}
