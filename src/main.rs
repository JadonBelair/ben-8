pub mod cpu;

use cpu::Cpu;
use egui::Rect;
use std::time::{Instant, Duration};
use macroquad::prelude::*;

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

    let output_font = load_ttf_font("./outputFont.ttf").await.expect("Missing font for the output display");

    let mut clock_speed_val = 100.;
    let mut clock_speed = Duration::from_millis(clock_speed_val as u64);
    let mut pulse_length = clock_speed / 2;

    let mut view_ram = false;

    let mut code = String::new();

    let mut cpu = Cpu::new();

    let mut pulse_start = Instant::now();

    let mut clock = Instant::now();
    let mut clock_enable = true;

    loop {
        clear_background(GRAY);

        // used for defining all egui components
        egui_macroquad::ui(|egui_ctx| {
            // draws the code editor to the screen
            egui::Window::new(if !view_ram {"Edit Code"} else {"View Ram Contents"})
            .fixed_rect(Rect { min: [10., 10.].into(), max: [210., 500.].into() })
            .show(egui_ctx, |ui| {
                if !view_ram {     
                    let code_box = egui::TextEdit::multiline(&mut code)
                        .font(egui::FontId { size: 30., ..Default::default()})
                        .desired_rows(16);
                    ui.add(code_box);
                } else {
                    let mut ram_contents = String::new();
                    for (i, n) in cpu.ram.iter().enumerate() {
                        ram_contents += format!("{n:08b} {n:#04X}{}", if i != 15 {"\n"} else {""}).as_str();
                    }

                    let ram = egui::Label::new(egui::RichText::new(ram_contents).size(30.));
                    ui.add(ram);
                }

                if ui.button("Switch Mode").clicked() {
                    view_ram = !view_ram;
                };
            });
            
            // draws the control buttons to the screen
            egui::Window::new("Controls").fixed_pos((230., 10.)).resizable(false)
            .show(egui_ctx, |ui| {
                let clock_btn = egui::Button::new(
                    egui::WidgetText::RichText(
                        egui::RichText::new("Clock Toggle").size(30.)
                    ));

                let assemble_btn = egui::Button::new(
                    egui::WidgetText::RichText(
                        egui::RichText::new("Assemble Code").size(30.)
                    ));

                let reset_btn = egui::Button::new(
                    egui::WidgetText::RichText(
                        egui::RichText::new("Reset").size(30.)
                    ));

                let pulse_btn = egui::Button::new(
                    egui::WidgetText::RichText(
                        egui::RichText::new("Pulse").size(30.)
                    ));

                if ui.add(clock_btn).clicked() {
                    clock_enable = !clock_enable;
                }
                
                if ui.add(assemble_btn).clicked() {
                    cpu.assemble(&code);
                }

                if ui.add(reset_btn).clicked() {
                    cpu.reset();
                }

                if ui.add(pulse_btn).clicked() && !clock_enable {
                    cpu.pulse();
                    pulse_start = Instant::now();
                }

                // draws the CPU clock speed slider and its label next to each other
                ui.horizontal(|ui| {
                    ui.label("CPU Speed");
                    let speed = egui::DragValue::new(&mut clock_speed_val).clamp_range(16..=1000);
                    ui.add(speed);
                });
            });
        });

        // draws the ui
        egui_macroquad::draw();

        if clock_speed_val as u128 != clock_speed.as_millis() {
            clock_speed = Duration::from_millis(clock_speed_val as u64);
            pulse_length = clock_speed / 2;
        }

        // ensures the user doesnt add more instructions/data then there are spots in memory
        let mut temp_string = String::new();
        for (i, l)in code.lines().take(16).enumerate() {
            temp_string += format!("{}{}", l, if i != 15 {"\n"} else {""}).as_str();
        }
        code = temp_string;

        // ensures that the clock pluses semi-regularly
        if (clock.elapsed() > clock_speed) && clock_enable {
            cpu.pulse();
            clock = Instant::now();
            pulse_start = Instant::now();
        }

        // LED DRAWING

        // draws the clock pulse
        draw_led(if pulse_start.elapsed() < pulse_length {1} else {0}, 340, 165,
            Color::new(0., 0., 0.51, 1.),
            Color::new(0., 0., 1., 1.), 1);

        // draws the program counter
        draw_led(cpu.pc as u16, (SCREEN_WIDTH - 150) as u16, 50,
            Color::new(0., 0.45, 0., 1.),
            Color::new(0., 1., 0., 1.), 4);
        
        // draws the contents of the bus
        draw_led(cpu.bus as u16, (screen_width() / 2.) as u16 + 31, 20,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the zero flag
        draw_led(if cpu.zf {1} else {0}, SCREEN_WIDTH as u16 - 56, REGISTER_Y - 40,
            Color::new(0., 0.45, 0., 1.),
            Color::new(0., 1., 0., 1.), 1);
        
        // draws the carry flag
        draw_led(if cpu.cf {1} else {0}, SCREEN_WIDTH as u16 - 32, REGISTER_Y - 40,
            Color::new(0., 0.45, 0., 1.),
            Color::new(0., 1., 0., 1.), 1);

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
        draw_led(cpu.mar as u16, 295, 300,
            Color::new(0.7, 0.7, 0., 1.),
            Color::new(0.95, 1., 0., 1.), 4);

        // draws the value at the current memory address
        draw_led(cpu.ram[cpu.mar] as u16, 245, 350,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 8);

        // draws the opcode section of the instruction register
        draw_led(cpu.ir as u16 >> 4, 245, 400,
            Color::new(0., 0., 0.51, 1.),
            Color::new(0., 0., 1., 1.), 4);

        // draws the addres section of the insruction register
        draw_led(cpu.ir as u16 & 0xF, 341, 400,
            Color::new(0.7, 0.7, 0., 1.),
            Color::new(0.95, 1., 0., 1.), 4);

        // draws the microcode step
        draw_led(cpu.step as u16, 245, 425,
            Color::new(0.51, 0., 0., 1.),
            Color::new(1., 0., 0., 1.), 3);
        
        // draws the microcode step pulse
        draw_led(((0b10000) >> cpu.step) as u16, 317, 425,
            Color::new(0., 1., 0., 1.), 
            Color::new(0., 0.45, 0., 1.), 5);

        // draws the control word
        draw_led(cpu.microcode[cpu.get_micro_loc()], SCREEN_WIDTH as u16 - 390, SCREEN_HEIGHT as u16 - 35,
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
