extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

// Huh, so mod is sorta like #include...
mod common;
mod shader;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::path::Path;

use shader::Shader;

extern crate image;
use image::GenericImage;

extern crate cgmath;
use cgmath::{Matrix4, Vector3, vec3, Deg, perspective, Point3};
use cgmath::prelude::*;

// settings 
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Cube", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    let mut deltaTime: f32;
    let mut lastFrame: f32 = 0.0;

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // let (ourShader, VBO, VAO, texture1, texture2) = unsafe {
    //     gl::Enable(gl::DEPTH_TEST);

    //     let ourShader = Shader::new(
    //         "src/vertshader.glsl",
    //         "src/fragshader.glsl"
    //     );

    // };

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        };
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}