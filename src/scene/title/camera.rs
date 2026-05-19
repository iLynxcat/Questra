use raylib::{RaylibHandle, camera::Camera3D, math::Vector3};

use crate::scene::render::lerp_smooth;

pub struct Camera {
    pub raycam: Camera3D,

    position: Vector3,
    target: Vector3,
    fovy: f32,

    pub position_destination: Vector3,
    pub target_destination: Vector3,
    pub fovy_destination: f32,
}

impl Camera {
    pub fn new(position: Vector3, target: Vector3, fovy: f32) -> Self {
        Self {
            raycam: Camera3D::perspective(position, target, Vector3::up(), fovy),
            position,
            target,
            fovy,
            position_destination: position,
            target_destination: target,
            fovy_destination: fovy,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if (self.position_destination - self.position).length() > 0.001 {
            self.position = lerp_smooth(
                self.position,
                self.position_destination,
                0.25,
                rl.get_frame_time(),
            );
        }

        if (self.target_destination - self.target).length() > 0.001 {
            self.target = lerp_smooth(
                self.target,
                self.target_destination,
                0.25,
                rl.get_frame_time(),
            );
        }

        if f32::abs(self.fovy_destination - self.fovy) > 0.001 {
            self.fovy = lerp_smooth(self.fovy, self.fovy_destination, 0.15, rl.get_frame_time())
        }

        self.raycam.position = self.position;
        self.raycam.target = self.target;
        self.raycam.fovy = self.fovy;
    }
}
