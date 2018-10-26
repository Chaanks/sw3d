use cgmath::{
    SquareMatrix,
    Matrix4,
    Vector3,
};


pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Matrix4<f32>,
    pub scale: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self {
        let position = Vector3::new(0.0, 0.0, 0.0);
        let rotation =  Matrix4::identity();
        let scale = Matrix4::identity();

        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn scale(&mut self, value: f32) {
        self.scale = Matrix4::from_scale(value);
    }
}