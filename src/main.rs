extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>) {
    framedata[((x + y * WIDTH)*4 + 0) as usize] = color.b;
    framedata[((x + y * WIDTH)*4 + 1) as usize] = color.g;
    framedata[((x + y * WIDTH)*4 + 2) as usize] = color.r;
    framedata[((x + y * WIDTH)*4 + 3) as usize] = color.a;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sdlrs_template", WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();
    let mut framedata: Vec<u8> = vec![0; ((WIDTH*HEIGHT)*4) as usize];

    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    let multiplier: u32 = 255;

    // "board" states for the ca
    let mut curr: Vec<u8> = vec![1; (WIDTH*3) as usize];
    let mut prev: Vec<u8> = vec![0; (WIDTH*3) as usize];

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                    break;
                },
                _ => {}
            }
        }

        for x in 0..WIDTH {
            match curr[(x + WIDTH) as usize] {
                0 => {put_pixel(x, 0, Color::BLACK, &mut framedata)},
                1 => {put_pixel(x, 0, Color::RED, &mut framedata)},
                _ => {}
            }
        }

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
