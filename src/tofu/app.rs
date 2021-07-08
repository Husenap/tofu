extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::ffi::{c_void, CStr};
use std::ptr;
use std::sync::mpsc::Receiver;

use cgmath::prelude::*;
use cgmath::*;

use crate::tofu;

const DEFAULT_SCREEN_WIDTH: u32 = 1600;
const DEFAULT_SCREEN_HEIGHT: u32 = 900;
const FOV: f32 = 50.0;

pub struct App {
    camera: tofu::Camera,
    screen_width: u32,
    screen_height: u32,
    gbuffer: tofu::Framebuffer,
}

impl Default for App {
    fn default() -> App {
        App {
            camera: tofu::Camera::new(),
            screen_width: DEFAULT_SCREEN_WIDTH,
            screen_height: DEFAULT_SCREEN_HEIGHT,
            gbuffer: tofu::Framebuffer::default(),
        }
    }
}

extern "system" fn gl_debug_output(
    source: gl::types::GLenum,
    type_: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut c_void,
) {
    if id == 131_169 || id == 131_185 || id == 131_218 || id == 131_204 {
        // ignore these non-significant error codes
        return;
    }

    println!("---------------");
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };
    println!("Debug message ({}): {}", id, message);
    match source {
        gl::DEBUG_SOURCE_API => println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => println!("Source: Other"),
        _ => println!("Source: Unknown enum value"),
    }

    match type_ {
        gl::DEBUG_TYPE_ERROR => println!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => println!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => println!("Type: Other"),
        _ => println!("Type: Unknown enum value"),
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH => println!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => println!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => println!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
        _ => println!("Severity: Unknown enum value"),
    }
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn run(&mut self) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(
                self.screen_width,
                self.screen_height,
                "Tofu",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window!");

        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::None);
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        let (fbw, fbh) = window.get_framebuffer_size();
        self.screen_width = fbw as u32;
        self.screen_height = fbh as u32;

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(gl_debug_output), ptr::null());
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                ptr::null(),
                gl::TRUE,
            );

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::Enable(gl::DEPTH_TEST);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }

        let model =
            tofu::Model::load_from_file("assets/models/3d_other_ufnscjdga/ufnscjdga_LOD0.obj");
        //let model = tofu::Model::new("assets/models/normal_test/normal_test.obj");

        let fullscreen_quad = tofu::Model::get_fullscreen_triangle();

        let gbuffer_shader =
            tofu::Shader::new("assets/shaders/basic.vs", "assets/shaders/gbuffer.fs");

        let copy_shader = tofu::Shader::new("assets/shaders/copy.vs", "assets/shaders/copy.fs");
        unsafe {
            copy_shader.use_program();
            copy_shader.set_int("uPositionTexture", 0);
            copy_shader.set_int("uNormalTexture", 1);
            copy_shader.set_int("uAlbedoRoughnessTexture", 2);
        }

        self.gbuffer = tofu::Framebuffer::new(
            self.screen_width,
            self.screen_height,
            [
                tofu::framebuffer::RenderTargetDescription {
                    internal_format: gl::RGBA16F,
                    format: gl::RGBA,
                    data_type: gl::FLOAT,
                },
                tofu::framebuffer::RenderTargetDescription {
                    internal_format: gl::RGBA16F,
                    format: gl::RGBA,
                    data_type: gl::FLOAT,
                },
                tofu::framebuffer::RenderTargetDescription {
                    internal_format: gl::RGBA,
                    format: gl::RGBA,
                    data_type: gl::UNSIGNED_BYTE,
                },
            ]
            .to_vec(),
        );

        self.camera.set_position(Point3::new(0.0, 1.0, 7.0));
        self.camera.make_perspective(
            FOV,
            DEFAULT_SCREEN_WIDTH as f32 / DEFAULT_SCREEN_HEIGHT as f32,
        );

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
                self.gbuffer.bind_as_target();
                gl::ClearColor(0.0, 0.0, 0.0, 1000.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                for i in -1..2 {
                    let mut model_matrix = Matrix4::from_scale(1.0);
                    model_matrix = Matrix4::from_angle_y(Rad(time * 0.25)) * model_matrix;
                    model_matrix =
                        Matrix4::from_translation(vec3(i as f32 * 5.0, 0.0, 0.0)) * model_matrix;

                    let normal_matrix = Transform::inverse_transform(&model_matrix)
                        .unwrap()
                        .transpose();

                    let model_view_projection_matrix =
                        self.camera.get_view_projection() * model_matrix;

                    gbuffer_shader.use_program();

                    gbuffer_shader.set_float("uTime", time);
                    gbuffer_shader.set_mat4("uModelMatrix", &model_matrix);
                    gbuffer_shader.set_mat4("uNormalMatrix", &normal_matrix);
                    gbuffer_shader
                        .set_mat4("uModelViewProjectionMatrix", &model_view_projection_matrix);

                    model.draw(&gbuffer_shader);
                }
                self.gbuffer.unbind_as_target();

                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                copy_shader.use_program();
                self.gbuffer.bind_as_source();
                fullscreen_quad.draw(&copy_shader);
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
                        self.screen_width = width as u32;
                        self.screen_height = height as u32;

                        self.gbuffer.resize(self.screen_width, self.screen_height);
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

        self.camera.process_input(window);
    }
}
