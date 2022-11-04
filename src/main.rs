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

fn get_bit(n: u8, b: u8) -> u8 {
    (n >> b) & 1
}

fn run_rule(rule: u8, curr: &mut Vec<u8>, prev: &mut Vec<u8>) {
    let mut value = 0;
    for i in 0..prev.len()-1 {
        if i != 0 {
            let state: u8 = ((prev[i-1]&1) << 2) | ((prev[i]&1) << 1) |  (prev[i+1]&1);
            value = get_bit(rule, state);

        }
        if i < curr.len() {
            curr[i] = value;
        }
    }
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

    // "board" states for the ca
    let mut curr: Vec<u8> = vec![0; (WIDTH*3) as usize];
    let mut prev: Vec<u8> = vec![0; (WIDTH*3) as usize];

    // setting the middle tile to 1
    curr[(WIDTH + WIDTH/2) as usize] = 1;

    let mut step = 0;
    let mut rule = 0;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                    break;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => { 
                    step = 0;
                    rule += 1;
                    curr = vec![0; (WIDTH*3) as usize];
                    prev = vec![0; (WIDTH*3) as usize];
                    curr[(WIDTH + WIDTH/2) as usize] = 1;
                    println!("Rule {}", rule);
                    break;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => { 
                    step = 0;
                    if rule > 0 {rule -= 1;}
                    curr = vec![0; (WIDTH*3) as usize];
                    prev = vec![0; (WIDTH*3) as usize];
                    curr[(WIDTH + WIDTH/2) as usize] = 1;
                    println!("Rule {}", rule);
                    break;
                },
                _ => {}
            }
        }

        prev.clear();
        prev.extend_from_slice(&curr);

        // draw board state
        for x in 0..WIDTH {
            match curr[(x + WIDTH) as usize] {
                0 => {put_pixel(x, step, Color::BLACK, &mut framedata)},
                1 => {put_pixel(x, step, Color::WHITE, &mut framedata)},
                _ => {}
            }
        }

        // update board state
        if step < HEIGHT-1 { step += 1; 

            run_rule(rule, &mut curr, &mut prev);
        }

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }


}
