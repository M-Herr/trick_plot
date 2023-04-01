#![allow(unused)]
#![feature(buf_read_has_data_left)]

use std::default;
use std::str;
use std::env;
use std::fs;
use std::fs::File;
use std::error::Error;

use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;
use std::mem;
use eframe::CreationContext;
use eframe::egui;
use egui::plot::{
    Arrows, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, HLine,
    Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint, PlotPoints, Points, Polygon,
    Text, VLine,
};


pub mod trick_var_defs;

use crate::trick_var_defs::*;


fn main() {

    let trick_type_defs = TrickVarDefs::default();

    //Collect input arguments: should be 
   
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native("Plotter", 
    native_options,
    Box::new(|cc| Box::new(MyEguiApp::new(cc))));

}


fn parse_configs(args: &[String]) -> InputArgs {
    let name = args[1].clone();
    let path = args[2].clone();

    InputArgs {name, path}
}



#[derive(Debug, Default)]
struct PlotXAxis {
    var_name: String,
}

#[derive(Debug)]
struct PlotYAxis {
    var_name: String,
}

impl Default for PlotYAxis {
    fn default() -> Self {
        let var_name = "sys.exec.out.time";
        PlotYAxis {
            var_name: var_name.into()
        }
    }
}


#[derive(Default, Debug, PartialEq)]
enum AxisSelect {
    #[default]
    First,
    Second,
    Third
}
#[derive(Default)]
struct MyEguiApp {
    pub values: Vec<PlotPoint>,
    pub trick_data: TrickData,
    pub x_axis: String,
    pub y_axis: String,
    pub x_selected: usize,
    pub y_selected: usize,
}


impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        
        MyEguiApp { 
        ..Default::default() 
        }

    }

    fn load_trick_data(&mut self) {

        let args: Vec<String> = env::args().collect();
        let input = parse_configs(&args);
    
        let mut trick_data = TrickData::new(input);
    
        println!("{}, {}", trick_data.log_file.header_file_name, trick_data.log_file.log_file_name);
        println!("{}", trick_data.log_file.full_path);
    
        trick_data.read();    

        self.trick_data = trick_data.clone();
        print!("{:#?}", self.trick_data.descriptors);
    }

    fn update_x_axis_values(&mut self) {

    }

    fn update_y_axis_values(&mut self) {
        
    }


    fn add_values(&mut self) {
        self.values.resize(self.trick_data.data[0].data.len(), PlotPoint {x: 0.0, y: 0.0});

        //x axis is time
        let x_vals = vec![0.0; 0];
        
        let mut x_axis_vals: Vec<f64> = vec![0.0; 0];
        let mut y_axis_vals: Vec<f64> = vec![0.0; 0];

        for i in 0..self.trick_data.descriptors.len() {

            if self.x_axis == self.trick_data.descriptors[i].name {
                for j in 0..self.trick_data.data[i].data.len() {
                    x_axis_vals.push(self.trick_data.data[i].data[j]);
                }
            }

            if self.y_axis == self.trick_data.descriptors[i].name {
                for j in 0..self.trick_data.data[i].data.len() {
                    y_axis_vals.push(self.trick_data.data[i].data[j]);
                }
            }
        }

        
        if(!x_axis_vals.is_empty() &&  !y_axis_vals.is_empty()) {
            for i in 0..self.trick_data.data[0].data.len() {
                self.values[i] = PlotPoint {x: x_axis_vals[i], y:y_axis_vals[i]}
            }
        }

    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });
            ui.menu_button("Load Data", |ui| {
                if ui.button("Load Sample Data").clicked() {
                    self.load_trick_data();
                    self.add_values();
                }
            })
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.end_row();

        if(!self.trick_data.descriptors.is_empty()) {
            
            egui::ComboBox::from_label("Select X Axis:")
                .selected_text(format!("{:?}", self.trick_data.descriptors[self.x_selected].name.clone()))
                .show_ui(ui, |ui| {
                    for i in 0..self.trick_data.descriptors.len() {
                        let value = ui.selectable_value(&mut self.trick_data.descriptors[i].name.clone(), self.trick_data.descriptors[self.x_selected].name.clone(), self.trick_data.descriptors[i].name.clone());
                        if (value.clicked()) {
                            self.x_selected = i;
                            self.x_axis = self.trick_data.descriptors[i].name.clone();
                            self.add_values();
                        }
                    }

                });


                egui::ComboBox::from_label("Select Y Axis:")
                .selected_text(format!("{:?}", self.trick_data.descriptors[self.y_selected].name.clone()))
                .show_ui(ui, |ui| {
                    for i in 0..self.trick_data.descriptors.len() {
                        let value = ui.selectable_value(&mut self.trick_data.descriptors[i].name.clone(), self.trick_data.descriptors[self.y_selected].name.clone(), self.trick_data.descriptors[i].name.clone());
                        if (value.clicked()) {
                            self.y_selected = i;
                            self.y_axis = self.trick_data.descriptors[i].name.clone();
                            self.add_values();
                        }
                    }
                });

            
        }
        egui::warn_if_debug_build(ui);

        let mut plot = egui::plot::Plot::new("A test plot!")
            .legend(Legend::default());
        
        plot.show(ui, |plot_ui| {
            plot_ui.line(egui::plot::Line::new(
                egui::plot::PlotPoints::Owned(Vec::from_iter(self.values.iter().copied())),
            ));

        })

    });

   }
}