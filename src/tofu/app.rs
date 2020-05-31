extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;
use self::gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

use cgmath::prelude::*;
use cgmath::*;

use crate::tofu;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 1200;

#[allow(dead_code)]
pub struct Vertex {
    pos: Vector3<f32>,
    uv: Vector2<f32>,
}

const VERTICES: [Vertex; 8] = [
    Vertex {
        pos: vec3(-0.5, 0.5, -0.5),
        uv: vec2(0.0, 1.0),
    },
    Vertex {
        pos: vec3(0.5, 0.5, -0.5),
        uv: vec2(1.0, 1.0),
    },
    Vertex {
        pos: vec3(-0.5, -0.5, -0.5),
        uv: vec2(0.0, 0.0),
    },
    Vertex {
        pos: vec3(0.5, -0.5, -0.5),
        uv: vec2(1.0, 0.0),
    },
    Vertex {
        pos: vec3(-0.5, 0.5, 0.5),
        uv: vec2(1.0, 1.0),
    },
    Vertex {
        pos: vec3(0.5, 0.5, 0.5),
        uv: vec2(0.0, 1.0),
    },
    Vertex {
        pos: vec3(-0.5, -0.5, 0.5),
        uv: vec2(1.0, 0.0),
    },
    Vertex {
        pos: vec3(0.5, -0.5, 0.5),
        uv: vec2(0.0, 0.0),
    },
];

#[rustfmt::skip]
const INDICES: [u32; 36] = [
    0, 1, 2, 1, 3, 2,
    0, 4, 5, 0, 5, 1,
    2, 3, 7, 2, 7, 6,
    6, 5, 4, 6, 7, 5,
    4, 0, 6, 0, 2, 6,
    1, 5, 3, 5, 7, 3,
];

const CUBE_POSITIONS: [Vector3<f32>; 10] = [
    vec3(0.0, 0.0, 0.0),
    vec3(2.0, 5.0, -15.0),
    vec3(-1.5, -2.2, -2.5),
    vec3(-3.8, -2.0, -12.3),
    vec3(2.4, -0.4, -3.5),
    vec3(-1.7, 3.0, -7.5),
    vec3(1.3, -2.0, -2.5),
    vec3(1.5, 2.0, -2.5),
    vec3(1.5, 0.2, -1.5),
    vec3(-1.3, 1.0, -1.5),
];

pub struct App {
    camera: tofu::Camera,
}

impl App {
    pub fn new() -> App {
        App {
            camera: tofu::Camera::new(),
        }
    }

    pub fn run(&mut self) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                "Tofu",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window!");

        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::None);
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let shader = tofu::Shader::new("assets/shaders/basic.vs", "assets/shaders/basic.fs");
        let texture1 = tofu::Texture::new("assets/images/dubu.jpg");
        let texture2 = tofu::Texture::new("assets/images/twice.png");

        let monkey = tofu::Model::new("assets/models/monkey.fbx");

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);

            shader.use_program();
            shader.set_int("texture1", 0);
            shader.set_int("texture2", 1);
        };

        self.camera.set_position(Point3::new(0.0, 0.0, 5.0));
        self.camera.make_perspective(78.0, 1.0);

        let mut last_frame = glfw.get_time() as f32;
        let mut delta_time;

        while !window.should_close() {
            self.process_events(&events);
            self.process_input(&mut window);

            let time = glfw.get_time() as f32;
            delta_time = time - last_frame;
            last_frame = time;

            self.camera.update(delta_time);

            unsafe {
                gl::ClearColor(1.0 * 0.2, 0.37 * 0.2, 0.64 * 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                shader.use_program();
                monkey.use_model();

                texture1.bind(gl::TEXTURE0);
                texture2.bind(gl::TEXTURE1);

                shader.set_float("uTime", time);

                for (i, position) in CUBE_POSITIONS.iter().enumerate() {
                    let t = time + i as f32;
                    let mut model: Matrix4<f32> = Matrix4::from_axis_angle(
                        vec3(0.5, 1.0, (t * 0.73).sin()).normalize(),
                        Rad(t),
                    );
                    model = Matrix4::from_translation(*position) * model;

                    let model_view_projection = self.camera.get_view_projection() * model;

                    shader.set_mat4("uModel", &model);
                    shader.set_mat4("uModelViewProjection", &model_view_projection);

                    monkey.draw();
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }

        window.close();
    }

    fn process_events(&mut self, events: &Receiver<(f64, glfw::WindowEvent)>) {
        for (_, event) in glfw::flush_messages(events) {
            if let glfw::WindowEvent::FramebufferSize(width, height) = event {
                if width != 0 && height != 0 {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                        self.camera
                            .make_perspective(50.0, width as f32 / height as f32);
                    }
                }
            }
        }
    }

    fn process_input(&mut self, window: &mut glfw::Window) {
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true)
        }

        self.camera.process_input(&window);
    }
}
