extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

use cgmath::prelude::*;
use cgmath::*;

use crate::tofu;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;
const FOV: f32 = 50.0;

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

        // let texture_diffuse = tofu::Texture::new("assets/models/walther_01/walther_01_diffuse.png");
        // let texture_normal = tofu::Texture::new("assets/models/walther_01/walther_01_normal.png");
        // let texture_arm = tofu::Texture::new("assets/models/walther_01/walther_01_arm.png");
        // let model_asset = tofu::Model::new("assets/models/walther_01/walther_01.fbx");

        let texture_albedo = tofu::Texture::new("assets/models/backpack/1001_albedo.jpg");
        let texture_normal = tofu::Texture::new("assets/models/backpack/1001_normal.png");
        let texture_arm = tofu::Texture::new("assets/models/backpack/1001_arm.jpg");
        let model_asset = tofu::Model::new("assets/models/backpack/backpack.fbx");

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);

            shader.use_program();
            shader.set_int("uAlbedoTexture", 0);
            shader.set_int("uNormalTexture", 1);
            shader.set_int("uARMTexture", 2);
        };

        self.camera.set_position(Point3::new(0.0, 1.0, 7.0));
        self.camera
            .make_perspective(FOV, SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32);

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
                model_asset.use_model();

                texture_albedo.bind(gl::TEXTURE0);
                texture_normal.bind(gl::TEXTURE1);
                texture_arm.bind(gl::TEXTURE2);

                shader.set_float("uTime", time);

                let model = Matrix4::from_scale(1.0);
                //model = Matrix4::from_angle_x(Deg(-90.0)) * model;
                //model = Matrix4::from_angle_y(Deg(180.0)) * model;
                //model = Matrix4::from_translation(vec3(4.0, 0.0, 0.0)) * model;

                let inverse_model = Transform::inverse_transform(&model).unwrap().transpose();

                let model_view_projection = self.camera.get_view_projection() * model;

                shader.set_mat4("uModel", &model);
                shader.set_mat4("uInverseModel", &inverse_model);
                shader.set_mat4("uModelViewProjection", &model_view_projection);

                model_asset.draw();
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
                            .make_perspective(FOV, width as f32 / height as f32);
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
