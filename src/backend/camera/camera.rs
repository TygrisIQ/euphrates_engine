use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub yaw: f32,   // degrees
    pub pitch: f32, // degrees
    pub speed: f32,
    pub sensitivity: f32,
    pub fov: f32,
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
impl Camera {
    pub fn new(position: Vec3) -> Self {
        Camera {
            position,
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::Y,
            yaw: -90.0,
            pitch: 0.0,
            speed: 2.5,
            sensitivity: 0.1,
            fov: 45.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn projection_matrix(&self, width: f32, height: f32) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.fov.to_radians(),
            width / height,
            0.1,
            100.0,
        )
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.speed * delta_time;
        match direction {
            CameraMovement::Forward  => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left     => self.position -= self.front.cross(self.up).normalize() * velocity,
            CameraMovement::Right    => self.position += self.front.cross(self.up).normalize() * velocity,
            CameraMovement::Up       => self.position += self.up * velocity,
            CameraMovement::Down     => self.position -= self.up * velocity,
        }
    }

    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw   += x_offset * self.sensitivity;
        self.pitch  = (self.pitch + y_offset * self.sensitivity).clamp(-89.0, 89.0);
        self.update_front();
    }

    fn update_front(&mut self) {
        self.front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
    }
}

