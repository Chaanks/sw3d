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
        self.scale += Matrix4::from_scale(value);
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }

    pub fn rotate(&mut self, angle: f32, vector: [f32; 3]) {
        self.rotation += Matrix4::from_axis_angle(Vector3::from(vector), cgmath::Deg(angle));
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation += Matrix4::from_angle_x(cgmath::Deg(angle));
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation += Matrix4::from_angle_y(cgmath::Deg(angle));
    }
    
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation += Matrix4::from_angle_z(cgmath::Deg(angle));
    }

    pub fn translation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
    }

}