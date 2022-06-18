pub mod cpu;

use cpu::Cpu;
use std::time::{Instant, Duration};
use macroquad::{prelude::*, ui::{widgets::{Editbox, Button}, root_ui, Skin}, hash};

const SCREEN_WIDTH: i32 = 900;
const SCREEN_HEIGHT: i32 = 600;

const LED_SIZE: u16 = 10;
const REGISTER_X: u16 = SCREEN_WIDTH as u16 - 200;
const REGISTER_Y: u16 = 150;

const OUTPUT_X: f32 = SCREEN_WIDTH as f32 - 193.;
const OUTPUT_Y: f32 = REGISTER_Y as f32 + 150.;

fn window_conf() -> Conf {
    Conf {
        window_title: "8-bit CPU".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let ide_style = root_ui().style_builder().font_size(30).build();

    let skin = Skin {
        editbox_style: ide_style,
        ..root_ui().default_skin()
    };

    let output_font = load_ttf_font("./outputFont.ttf").await.expect("Missing font for the output display");

    let mut code = String::new();

    let mut cpu = Cpu::new();

    let mut clock = Instant::now();
    let mut clock_enable = true;

    loop {
        clear_background(GRAY);

        // draws the button for switching to manual clock mode
        let clock_btn = Button::new("Clock Toggle").position(vec2(220., 10.)).size(vec2(100., 50.));
        if clock_btn.ui(&mut root_ui()) {
            clock_enable = !clock_enable;
        }

        // draws the button for assembling the program
        let assemble_btn = Button::new("Assemble Code").position(vec2(220., 70.)).size(vec2(100., 50.));
        if assemble_btn.ui(&mut root_ui()) {
            cpu.assemble(&code);
        }
        
        // draws the button for reseting the cpu
        let reset_btn = Button::new("Reset").position(vec2(220., 130.)).size(vec2(100., 50.));
        if reset_btn.ui(&mut root_ui()) {
            cpu.reset();
        }

        // draws the button for pulsing the cpu
        let pulse_btn = Button::new("Pulse").position(vec2(220., 190.)).size(vec2(100., 50.));
        if pulse_btn.ui(&mut root_ui()) && !clock_enable{
            cpu.pulse();
        }

        // draws the box to write the program in
        let input = Editbox::new(hash!(), vec2(200., 490.)).position(vec2(10., 10.));
        input.ui(&mut root_ui(), &mut code);

        // applies the custom skin to the display
        root_ui().push_skin(&skin);

        // ensures the user doesnt add more instructions/data then there are spots in memory
        let mut temp_string = String::new();
        for (i, l)in code.lines().take(16).enumerate() {
            temp_string += format!("{}{}", l, if i != 15 {"\n"} else {""}).as_str();
        }
        code = temp_string;

        // ensures that the clock pluses semi-regularly
        if (clock.elapsed() > Duration::from_millis(17)) && clock_enable {
            cpu.pulse();
            clock = Instant::now();
            // println!("{}", cpu.pc);
        }

        // LED DRAWING

        // draws the program counter
        draw_led(cpu.pc as u16, (SCREEN_WIDTH - 150) as u16, 50,
            Color::new(0., 0.45, 0., 1.),
            Color::new(0., 1., 0., 1.), 4);
        
        // draws the contents of the bus
        draw_led(cpu.bus as u16, (screen_width() / 2.) as u16 + 31, 20,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the a register
        draw_led(cpu.a.into(), REGISTER_X, REGISTER_Y,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the current alu output
        draw_led(cpu.alu().into(), REGISTER_X, REGISTER_Y + 50,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the b register
        draw_led(cpu.b.into(), REGISTER_X, REGISTER_Y + 100,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the value of the MAR (memory address register)
        draw_led(cpu.mar as u16, 280, 300,
            Color::new(0.7, 0.7, 0., 1.),
            Color::new(0.95, 1., 0., 1.), 4);

        // draws the value at the current memory address
        draw_led(cpu.ram[cpu.mar] as u16, 230, 350,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the opcode section of the instruction register
        draw_led(cpu.ir as u16 >> 4, 230, 400,
            Color::new(0., 0., 0.51, 1.),
            Color::new(0., 0., 1., 1.), 4);

        // draws the addres section of the insruction register
        draw_led(cpu.ir as u16 & 0xF, 326, 400,
            Color::new(0.7, 0.7, 0., 1.),
            Color::new(0.95, 1., 0., 1.), 4);

        // draws the microcode step
        draw_led(cpu.step as u16, 230, 425,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 3);
        
        // draws the microcode step pulse
        draw_led(((0b10000) >> cpu.step) as u16, 302, 425,
            Color::new(0., 1., 0., 1.), 
            Color::new(0., 0.45, 0., 1.), 5);

        // draws the control word
        draw_led(cpu.microcode[cpu.get_micro_loc()], screen_width() as u16 - 390, screen_height() as u16 - 35,
        Color::new(0.51, 0., 0., 1.),
        Color::new(1., 0., 0., 1.), 16);

        // DRAWS THE OUTPUT REGISTER DISPLAY

        // draws the black box for the background of the output register
        draw_rectangle(OUTPUT_X, OUTPUT_Y, 155., 80., BLACK);
        
        // draws the dark red text in all digits to show which segments are not on
        draw_text_ex("888", OUTPUT_X + 5., OUTPUT_Y + 70.,
            TextParams { font: output_font, font_size: 60, color: Color::new(0.25, 0., 0., 1.), ..Default::default() });

        // draws the value of the output register
        draw_text_ex(format!("{:>03}", cpu.output).as_str(),
            OUTPUT_X + 5., OUTPUT_Y + 70.,
            TextParams { font: output_font, font_size: 60, color: Color::new(1., 0., 0., 1.), ..Default::default() });

        next_frame().await
    }
}

fn draw_led(reg: u16, x: u16, y: u16, dim: Color, bright: Color, size: u16) {
    for i in 0..size {
        draw_circle((x + (((LED_SIZE * 2) + 4) * i)).into(), y.into(),
        LED_SIZE.into(),
        if reg & (1 << ((size - 1) - i)) > 0 {
            bright
        } else {
            dim
        });
    }
}
