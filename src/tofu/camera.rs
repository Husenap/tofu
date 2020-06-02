use cgmath::*;
use cgmath::{Matrix4, Vector3};

use glfw::{Action, Key, MouseButton};

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;
const MOVEMENT_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.1;

pub struct Camera {
    position: Point3<f32>,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    view_projection: Matrix4<f32>,
    velocity: Vector3<f32>,
    movement: Vector3<f32>,
    forward: Vector3<f32>,
    right: Vector3<f32>,
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    last_cursor_pos: Option<Vector2<f32>>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            projection: Transform::one(),
            view: Transform::one(),
            view_projection: Transform::one(),
            velocity: vec3(0.0, 0.0, 0.0),
            movement: vec3(0.0, 0.0, 0.0),
            forward: vec3(0.0, 0.0, -1.0),
            right: vec3(1.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            last_cursor_pos: None,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity = self.velocity.lerp(self.movement, delta_time * 10.0);
        self.movement = vec3(0.0, 0.0, 0.0);

        let current_velocity = self.velocity * delta_time * MOVEMENT_SPEED;
        self.translate(
            current_velocity.x * self.right
                + current_velocity.y * self.up
                + current_velocity.z * self.forward,
        );

        self.view = Matrix4::look_at(self.position, self.position + self.forward, self.up);
        self.view_projection = self.projection * self.view;
    }

    pub fn process_input(&mut self, window: &glfw::Window) {
        self.process_mouse_input(window);
        self.process_key_input(window);
    }

    fn process_mouse_input(&mut self, window: &glfw::Window) {
        let (cx, cy) = window.get_cursor_pos();
        let cursor_pos = vec2(cx as f32, cy as f32);

        let last_cursor_pos = if let Some(last_pos) = self.last_cursor_pos {
            last_pos
        } else {
            cursor_pos
        };

        self.last_cursor_pos = Some(cursor_pos);

        if window.get_mouse_button(MouseButton::Button2) != Action::Press {
            return;
        }

        let cursor_delta = cursor_pos - last_cursor_pos;

        self.yaw += cursor_delta.x * MOUSE_SENSITIVITY;
        self.pitch += -cursor_delta.y * MOUSE_SENSITIVITY;
        self.pitch = self.pitch.min(89.99).max(-89.99);

        self.forward = vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
        self.right = self.forward.cross(vec3(0.0, 1.0, 0.0)).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }

    fn process_key_input(&mut self, window: &glfw::Window) {
        if window.get_key(Key::W) == Action::Press {
            self.movement.z += 1.0;
        }
        if window.get_key(Key::S) == Action::Press {
            self.movement.z -= 1.0;
        }
        if window.get_key(Key::A) == Action::Press {
            self.movement.x -= 1.0;
        }
        if window.get_key(Key::D) == Action::Press {
            self.movement.x += 1.0;
        }
        if window.get_key(Key::Q) == Action::Press {
            self.movement.y -= 1.0;
        }
        if window.get_key(Key::E) == Action::Press {
            self.movement.y += 1.0;
        }

        let modified_speed = if window.get_key(Key::LeftShift) == Action::Press {
            2.0
        } else if window.get_key(Key::LeftControl) == Action::Press {
            0.25
        } else {
            1.0
        };

        if !self.movement.is_zero() {
            self.movement = self.movement.normalize_to(modified_speed);
        }
    }

    pub fn make_perspective(&mut self, fovy: f32, aspect_ratio: f32) {
        self.projection = perspective(Deg(fovy), aspect_ratio, Z_NEAR, Z_FAR);
    }

    pub fn set_position(&mut self, new_position: Point3<f32>) {
        self.position = new_position;
    }
    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
    }

    pub fn get_view_projection(&self) -> &Matrix4<f32> {
        &self.view_projection
    }
}
