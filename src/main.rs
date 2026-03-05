use glam::Vec2;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const RENDER_WIDTH: i32 = 640;
const RENDER_HEIGHT: i32 = 360;

/// This is the definitive function, implemented with raw math.
/// It has no dependency on confusing library functions.
fn window_to_world(
    window_pos: Vec2,
    window_dims: Vec2,
    render_dims: Vec2,
    camera: Camera2D,
) -> Vec2 {
    // Step 1: Scale the mouse coordinate from Window Space to Render Texture Space.
    let scale = render_dims / window_dims;
    let texture_coord = window_pos * scale;

    // Step 2: Manually reverse the camera transformation.
    // This is the algebraic inverse of the transformation that happens when you call `begin_mode2D`.
    let cam_target = Vec2::new(camera.target.x, camera.target.y);
    let cam_offset = Vec2::new(camera.offset.x, camera.offset.y);

    // (texture_coord - offset) -> Reverses the offset translation.
    // (... / zoom)             -> Reverses the zoom scaling.
    // (... + target)           -> Reverses the target translation.
    (texture_coord - cam_offset) / camera.zoom + cam_target
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Coordinate Test - Manual Math")
        .build();

    let mut render_texture = rl
        .load_render_texture(&thread, RENDER_WIDTH as u32, RENDER_HEIGHT as u32)
        .unwrap();

    let mut camera = Camera2D {
        offset: Vector2::new(RENDER_WIDTH as f32 / 2.0, RENDER_HEIGHT as f32 / 2.0),
        target: Vector2::new(0.0, 0.0),
        zoom: 1.0,
        rotation: 0.0,
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // --- Update Logic ---
        let dt = rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera.target.y -= 200.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera.target.y += 200.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            camera.target.x -= 200.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            camera.target.x += 200.0 * dt;
        }
        camera.zoom += rl.get_mouse_wheel_move() * 0.1;
        if camera.zoom < 0.1 {
            camera.zoom = 0.1;
        }

        // Get mouse position and manually convert it from raylib::Vector2 to glam::Vec2
        let rl_mouse_pos = rl.get_mouse_position();
        let mouse_window_pos = Vec2::new(rl_mouse_pos.x, rl_mouse_pos.y);

        // Perform the conversion using our definitive function
        let mouse_world_pos = window_to_world(
            mouse_window_pos,
            Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            Vec2::new(RENDER_WIDTH as f32, RENDER_HEIGHT as f32),
            camera,
        );

        // --- Drawing ---
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d2 = d.begin_texture_mode(&thread, &mut render_texture);
            d2.clear_background(Color::DARKGRAY);

            let mut d3 = d2.begin_mode2D(camera);

            for x in -10..=10 {
                d3.draw_line(x * 100, -1000, x * 100, 1000, Color::GRAY);
            }
            for y in -10..=10 {
                d3.draw_line(-1000, y * 100, 1000, y * 100, Color::GRAY);
            }
            d3.draw_circle_v(Vector2::zero(), 10.0, Color::RED); // Origin

            d3.draw_circle(
                mouse_world_pos.x as i32,
                mouse_world_pos.y as i32,
                10.0 / camera.zoom,
                Color::YELLOW,
            );
        }

        let source_rec = Rectangle::new(0.0, 0.0, RENDER_WIDTH as f32, -(RENDER_HEIGHT as f32));
        let dest_rec = Rectangle::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
        d.draw_texture_pro(
            &render_texture,
            source_rec,
            dest_rec,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        d.draw_text(
            &format!("Window Mouse: {:?}", mouse_window_pos),
            10,
            10,
            20,
            Color::WHITE,
        );
        d.draw_text(
            &format!("World Coords: {:?}", mouse_world_pos),
            10,
            40,
            20,
            Color::LIME,
        );
    }
}
