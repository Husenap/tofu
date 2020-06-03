use cgmath::*;
use cgmath::{Matrix4, Vector3};

use glfw::{Action, CursorMode, Key, MouseButton};

const Z_NEAR: f32 = 0.01;
const Z_FAR: f32 = 100.0;
const MOVEMENT_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: f32 = 0.1;
const LOOK_BUTTON: MouseButton = MouseButton::Button2;

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
    target_yaw: f32,
    target_pitch: f32,
    was_mouse_pressed: bool,
    last_mouse_pos: Vector2<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            projection: Transform::one(),
            view: Transform::one(),
            view_projection: Transform::one(),
            velocity: Vector3::zero(),
            movement: Vector3::zero(),
            forward: vec3(0.0, 0.0, -1.0),
            right: vec3(1.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            target_yaw: -90.0,
            target_pitch: 0.0,
            was_mouse_pressed: false,
            last_mouse_pos: Vector2::zero(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.pitch += (self.target_pitch - self.pitch) * delta_time * 15.0;
        self.yaw += (self.target_yaw - self.yaw) * delta_time * 15.0;
        self.update_direction_vectors();

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

    pub fn process_input(&mut self, window: &mut glfw::Window) {
        self.process_mouse_input(window);
        self.process_key_input(window);
    }

    fn process_mouse_input(&mut self, window: &mut glfw::Window) {
        if self.was_mouse_pressed && window.get_mouse_button(LOOK_BUTTON) == Action::Release {
            window.set_cursor_mode(CursorMode::Normal);
            self.was_mouse_pressed = false;
            return;
        }

        let (cx, cy) = window.get_cursor_pos();
        let cursor_pos = vec2(cx as f32, cy as f32);

        if !self.was_mouse_pressed && window.get_mouse_button(LOOK_BUTTON) == Action::Press {
            window.set_cursor_mode(CursorMode::Disabled);
            self.was_mouse_pressed = true;
            self.last_mouse_pos = cursor_pos;
            return;
        }

        if !self.was_mouse_pressed {
            return;
        }

        let cursor_delta = cursor_pos - self.last_mouse_pos;
        self.last_mouse_pos = cursor_pos;

        self.target_yaw += cursor_delta.x * MOUSE_SENSITIVITY;
        self.target_pitch += -cursor_delta.y * MOUSE_SENSITIVITY;
        self.target_pitch = self.target_pitch.min(89.99).max(-89.99);

        self.update_direction_vectors();
    }

    fn update_direction_vectors(&mut self) {
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
            5.0
        } else if window.get_key(Key::LeftControl) == Action::Press {
            0.1
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
