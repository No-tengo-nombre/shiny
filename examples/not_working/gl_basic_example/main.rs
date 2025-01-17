mod controller;

use gl;
use glfw;
use glfw::Context;

use controller::Controller;
use cuboid::opengl::components::{Camera, PerspectiveCamera, Material, Renderer3D, Shape};
use cuboid::opengl::{Shader, Window};
use cuboid::opengl::io::CameraController;
use cuboid::utils::{math::linalg, types};

const WINDOW_TITLE: &str = "Basic example";

fn main() {
    let mut delta;
    let mut fps;
    let mut prev_time = 0.0;

    // Initialization of the window
    let (mut window, events, mut glfw_instance) = Window::new()
        .dimensions(1000, 1000)
        .title(WINDOW_TITLE)
        .windowed()
        .build();
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);

    // Define a material
    let shader = Shader::new()
        .vertex("examples/gl_basic_example/resources/shaders/test.vert")
        .fragment("examples/gl_basic_example/resources/shaders/test.frag");
    let material = Material::new().shader(&shader);
    
    // Create the components
    let mut triangle_v: Vec<types::V6> = vec![
        [-0.75, -0.75, 0.0, 1.0, 0.0, 0.0],
        [0.75, -0.75, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.75, 0.0, 0.0, 0.0, 1.0],
    ];
    let triangle_i: Vec<u32> = vec![0, 1, 2];
    let mut cube_v: Vec<types::V6> = vec![
        [-10.5, -10.5, -10.5, 0.0, 0.0, 0.0],
        [-10.5, -10.5, 10.5, 0.0, 0.0, 1.0],
        [-10.5, 10.5, -10.5, 0.0, 1.0, 0.0],
        [-10.5, 10.5, 10.5, 0.0, 1.0, 1.0],
        [10.5, -10.5, -10.5, 1.0, 0.0, 0.0],
        [10.5, -10.5, 10.5, 1.0, 0.0, 1.0],
        [10.5, 10.5, -10.5, 1.0, 1.0, 0.0],
        [10.5, 10.5, 10.5, 1.0, 1.0, 1.0],
    ];
    // Quads indices
    let cube_i: Vec<u32> = vec![
        0, 1, 3, 2,
        0, 4, 6, 2,
        4, 5, 7, 6,
        1, 5, 7, 3,
        0, 1, 5, 4,
        2, 3, 7, 6,
    ];

    let triangle = Shape::new()
        .vertices(&triangle_v)
        .indices(&triangle_i)
        .material(&material)
        .layouts(&[0, 1])
        .usage(gl::DYNAMIC_DRAW)
        .build();

    let cube = Shape::new()
        .vertices(&cube_v)
        .indices(&cube_i)
        .material(&material)
        .layouts(&[0, 1])
        .usage(gl::DYNAMIC_DRAW)
        .build();

    // Add the items to the renderer
    renderer.add_item_with_mode(&cube, gl::QUADS);
    renderer.add_item(&triangle);

    // Define a custom camera
    let mut camera_pos = [0.0, 0.0, 20.0];
    let mut camera_dir = [0.0, 0.0, 1.0];
    let mut camera_up = [0.0, 1.0, 0.0];
    let mut camera_right;

    let mut camera = PerspectiveCamera::new(
        &camera_pos,
        &camera_dir,
        &camera_up,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        1000.0,
    );

    let cam_mov_speed = 0.1;
    let cam_rot_speed = 1.0;

    // Making a custom controller
    let mut wireframe = false;
    let mut controller = Controller::new();

    while !window.should_close() {
        controller.poll_window_events(&mut glfw_instance, &events);
        if controller.esc_pressed {
            window.set_should_close(true);
        }
        let time = glfw_instance.get_time() as f32;
        delta = time - prev_time;
        fps = 1.0 / delta;

        camera_right = linalg::normalize_v3(&linalg::cross_v3(&camera_dir, &camera_up));

        if wireframe != controller.wireframe {
            if controller.wireframe {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                println!("LINE")
            } else {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                println!("FILL")
            }
            wireframe = controller.wireframe;
        }

        // Camera movement control
        if controller.w_pressed {
            camera_pos[0] -= cam_mov_speed * camera_dir[0];
            camera_pos[1] -= cam_mov_speed * camera_dir[1];
            camera_pos[2] -= cam_mov_speed * camera_dir[2];
        }
        if controller.s_pressed {
            camera_pos[0] += cam_mov_speed * camera_dir[0];
            camera_pos[1] += cam_mov_speed * camera_dir[1];
            camera_pos[2] += cam_mov_speed * camera_dir[2];
        }
        if controller.a_pressed {
            camera_pos[0] -= cam_mov_speed * camera_right[0];
            camera_pos[1] -= cam_mov_speed * camera_right[1];
            camera_pos[2] -= cam_mov_speed * camera_right[2];
        }
        if controller.d_pressed {
            camera_pos[0] += cam_mov_speed * camera_right[0];
            camera_pos[1] += cam_mov_speed * camera_right[1];
            camera_pos[2] += cam_mov_speed * camera_right[2];
        }

        // Camera rotation control
        if controller.up_pressed {
            camera_dir = linalg::mat3_mul_v3(
                &linalg::rot_mat3(&camera_right, -cam_rot_speed),
                &camera_dir,
            );
            camera_up =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, -cam_rot_speed), &camera_up);
        }
        if controller.down_pressed {
            camera_dir =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, cam_rot_speed), &camera_dir);
            camera_up =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, cam_rot_speed), &camera_up);
        }
        if controller.left_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat3_y(-cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat3_y(-cam_rot_speed), &camera_up);
        }
        if controller.right_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat3_y(cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat3_y(cam_rot_speed), &camera_up);
        }
        camera.update(&camera_pos, &camera_dir, &camera_up);

        // Random functionality for the mouse buttons
        if controller.r_button_pressed {
            println!("Δt: {} ms  |  FPS: {}", delta * 1000.0, fps);
        }
        if controller.l_button_pressed {
            println!("LEFT");
        }

        // This functionality is to show the use of shaders and uniforms
        let r = ((2.5 * time) / 2.0 + 0.5).sin();
        let g = ((2.5 * time + 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        let b = ((2.5 * time - 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();

        let rot_speed = 10.0;

        let mut new_triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat3_x(rot_speed * delta));
        new_triangle_v = linalg::mat6_mul3(&new_triangle_v, &linalg::rot_mat3_y(rot_speed * delta));
        new_triangle_v = linalg::mat6_mul3(&new_triangle_v, &linalg::rot_mat3_z(rot_speed * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat3_x(rot_speed * delta));
        // triangle.set_vertices(&triangle_v, &[0, 1]);
        triangle.vertices(&new_triangle_v);

        let mut new_cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat3_x(rot_speed * delta));
        new_cube_v = linalg::mat6_mul3(&new_cube_v, &linalg::rot_mat3_y(rot_speed * delta));
        new_cube_v = linalg::mat6_mul3(&new_cube_v, &linalg::rot_mat3_z(rot_speed * delta));
        // cube.set_vertices(&cube_v, &[0, 1]);
        cube.vertices(&new_cube_v);

        // TODO: Make materials handle these uniforms.
        material._shader.set_4f("timeColor", r, g, b, 1.0);

        renderer.clear();
        renderer.render();
        window.swap_buffers();
        prev_time = time;
    }
}
