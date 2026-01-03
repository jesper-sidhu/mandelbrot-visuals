use macroquad::prelude::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MAX_ITER: u32 = 256;

struct View {
    center_x: f64,
    center_y: f64,
    zoom: f64,
}

impl View {
    fn new() -> Self {
        Self {
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
        }
    }

    fn screen_to_complex(&self, x: f32, y: f32) -> (f64, f64) {
        let aspect = WIDTH as f64 / HEIGHT as f64;
        let range = 3.5 / self.zoom;

        let real = self.center_x + (x as f64 / WIDTH as f64 - 0.5) * range * aspect;
        let imag = self.center_y + (y as f64 / HEIGHT as f64 - 0.5) * range;

        (real, imag)
    }
}

fn mandelbrot(c_real: f64, c_imag: f64, max_iter: u32) -> u32 {
    let mut z_real = 0.0;
    let mut z_imag = 0.0;
    let mut iter = 0;

    while z_real * z_real + z_imag * z_imag <= 4.0 && iter < max_iter {
        let temp = z_real * z_real - z_imag * z_imag + c_real;
        z_imag = 2.0 * z_real * z_imag + c_imag;
        z_real = temp;
        iter += 1;
    }

    iter
}

fn color_from_iter(iter: u32, max_iter: u32) -> Color {
    if iter == max_iter {
        BLACK
    } else {
        let t = iter as f32 / max_iter as f32;
        let hue = t * 360.0;

        // Convert HSV to RGB (simplified)
        let s = 0.8;
        let v = if t < 0.5 { t * 2.0 } else { 1.0 };

        let c = v * s;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match (hue / 60.0) as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Color::new(r + m, g + m, b + m, 1.0)
    }
}

fn render_mandelbrot(view: &View) -> Image {
    let mut img = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (c_real, c_imag) = view.screen_to_complex(x as f32, y as f32);
            let iter = mandelbrot(c_real, c_imag, MAX_ITER);
            let color = color_from_iter(iter, MAX_ITER);
            img.set_pixel(x as u32, y as u32, color);
        }
    }

    img
}

#[macroquad::main("Mandelbrot Zoom")]
async fn main() {
    let mut view = View::new();
    let mut texture = Texture2D::from_image(&render_mandelbrot(&view));
    let mut rendering = false;

    loop {
        clear_background(BLACK);

        // Handle mouse click to zoom
        if is_mouse_button_pressed(MouseButton::Left) && !rendering {
            rendering = true;
            let (mx, my) = mouse_position();
            let (new_x, new_y) = view.screen_to_complex(mx, my);
            view.center_x = new_x;
            view.center_y = new_y;
            view.zoom *= 2.0;

            let img = render_mandelbrot(&view);
            texture = Texture2D::from_image(&img);
            rendering = false;
        }

        // Handle right click to zoom out
        if is_mouse_button_pressed(MouseButton::Right) && !rendering {
            rendering = true;
            let (mx, my) = mouse_position();
            let (new_x, new_y) = view.screen_to_complex(mx, my);
            view.center_x = new_x;
            view.center_y = new_y;
            view.zoom /= 2.0;

            let img = render_mandelbrot(&view);
            texture = Texture2D::from_image(&img);
            rendering = false;
        }

        // Reset with R key
        if is_key_pressed(KeyCode::R) && !rendering {
            rendering = true;
            view = View::new();
            let img = render_mandelbrot(&view);
            texture = Texture2D::from_image(&img);
            rendering = false;
        }

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH as f32, HEIGHT as f32)),
                ..Default::default()
            },
        );

        // Draw instructions
        draw_text("Left Click: Zoom In", 10.0, 20.0, 20.0, WHITE);
        draw_text("Right Click: Zoom Out", 10.0, 40.0, 20.0, WHITE);
        draw_text("R: Reset", 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Zoom: {:.1}x", view.zoom), 10.0, 80.0, 20.0, WHITE);

        next_frame().await
    }
}

