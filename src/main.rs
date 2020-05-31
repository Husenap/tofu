extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;
use self::gl::types::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;

mod tofu;

use cgmath::prelude::*;
use cgmath::*;

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

pub fn main() {
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

    let (shader, vao, ebo, texture1, texture2) = unsafe {
        gl::Enable(gl::DEPTH_TEST);

        let shader = tofu::Shader::new("assets/shaders/basic.vs", "assets/shaders/basic.fs");

        // setup VAO
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mem::size_of_val(&VERTICES)) as GLsizeiptr,
            &VERTICES[0] as *const _ as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(&INDICES) as GLsizeiptr,
            &INDICES[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of_val(&VERTICES[0]) as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            mem::size_of_val(&VERTICES[0]) as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        let texture1 = tofu::Texture::new("assets/images/dubu.jpg");
        let texture2 = tofu::Texture::new("assets/images/twice.png");

        shader.use_program();
        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);

        (shader, vao, ebo, texture1, texture2)
    };

    let brightness: f32 = 0.2;

    let projection: Matrix4<f32> = perspective(Deg(50.0), 1.0, 0.1, 100.0);

    while !window.should_close() {
        process_events(&mut window, &events);

        let time = glfw.get_time() as f32;

        let radius: f32 = 10.0;
        let view: Matrix4<f32> = Matrix4::look_at(
            Point3::new(time.sin(), 0.0, time.cos()) * radius,
            Point3::new(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let view_projection = projection * view;

        unsafe {
            gl::ClearColor(1.0 * brightness, 0.37 * brightness, 0.64 * brightness, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            texture1.bind(gl::TEXTURE0);
            texture2.bind(gl::TEXTURE1);

            shader.set_float("uTime", time);

            for (i, position) in CUBE_POSITIONS.iter().enumerate() {
                let t = time + i as f32;
                let mut model: Matrix4<f32> =
                    Matrix4::from_axis_angle(vec3(0.5, 1.0, (t * 0.73).sin()).normalize(), Rad(t));
                model = Matrix4::from_translation(*position) * model;

                let model_view_projection = view_projection * model;

                shader.set_mat4("uMVP", &model_view_projection);

                gl::DrawElements(
                    gl::TRIANGLES,
                    INDICES.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    window.close();
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
