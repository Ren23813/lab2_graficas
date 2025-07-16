mod life;
mod framebuffer;

use raylib::prelude::*;
use life::Life;
use framebuffer::Framebuffer;

const CELL_SIZE: i32 = 4;  
const WIDTH: i32 = 200;
const HEIGHT: i32 = 150;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
        .title("Game of Life")
        .build();

    let mut fb = Framebuffer::new(WIDTH*3, HEIGHT*3, Color::BLACK);
    fb.set_current_color(Color::PERU); //Perú es clave 👁️🔐

    let mut world = Life::new(WIDTH as usize, HEIGHT as usize);
    world.randomize(0.25);  // % de células vivas al inicio

  //

    while !rl.window_should_close() {
        fb.clear();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if world.cells[(y as usize) * (WIDTH as usize) + (x as usize)] {
                    for py in 0..CELL_SIZE {
                        for px in 0..CELL_SIZE {
                            fb.set_pixel(x * CELL_SIZE + px, y * CELL_SIZE + py);
                        }
                    }
                }
            }
        }

        // Muestra en pantalla con cambio de buffers
        fb.swap_buffers(&mut rl, &thread);

        world.step();

        // Control de velocidad (aprox. 10 FPS)
        rl.set_target_fps(10);
    }
}
