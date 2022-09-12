#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;
use egui::Color32;

pub mod memory;
pub mod cpu;
pub mod ops_table;
pub mod gameboy;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.vsync = true;
    eframe::run_native(
        "gbemu",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}


struct MyApp {
    screen: [[u8; 160];144],
    screen_image : egui::ColorImage,
    frame_iter : u32,
    gb: gameboy::Gameboy,
}


impl MyApp {
    fn update_color_image(&mut self) {
        for (y, row) in self.screen.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let val;
                match *col {
                    0 => val = 0,
                    1 => val = 85,
                    2 => val = 170,
                    3 => val = 255,
                    _ => val = 0,
                }
                let index = row.len() * y + x;
                self.screen_image.pixels[index] = Color32::from_gray(val);

                // Test pattern: Diagonal rainbow
                //let c = egui::color::Hsva::new((x + y + self.frame_iter as usize) as f32 / (self.screen.len() + row.len()) as f32, 1f32, 1f32, 1f32);
                //let (r, g, b) = (c.to_rgb()[0] * 255f32, c.to_rgb()[1] * 255f32, c.to_rgb()[2] * 255f32);
                //self.screen_image.pixels[index] = Color32::from_rgb(r as u8, g as u8, b as u8);
            }
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        //println!("{}", ROM00); // Just checking that memory consts imported
        let color_image = egui::ColorImage::new([160usize,144usize], egui::Color32::WHITE);
        let mut gameboy = gameboy::Gameboy::new();
        gameboy.reset();
        return Self {
            screen: [[0u8; 160];144],
            screen_image: color_image,
            frame_iter: 0,
            gb: gameboy,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.gb.step();
        self.update_color_image();
        let sct = ctx.load_texture("color_image", self.screen_image.clone());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Hello, World!");
            });
            ui.image(sct.id(), sct.size_vec2());
            //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
        });
        self.frame_iter += 1;
        ctx.request_repaint();
    }
}