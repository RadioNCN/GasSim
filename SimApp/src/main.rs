mod nodes;
mod views;
mod simulation;

use crate::nodes::Node;
use crate::simulation::SimulationEngine;
use eframe::{App, Frame};
use egui::Context;
use egui_snarl::ui::{PinPlacement, SnarlStyle, SnarlViewer, SnarlWidget};
use egui_snarl::{InPinId, OutPinId, Snarl};
use GasSim::modules::PID::{PID_input, PID_para, PID};
use GasSim::modules::PT1::pt1;
use crate::nodes::ControlNodes::ControlNode;
use std::sync::{Arc, Mutex};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GasSim",
        native_options,
        Box::new(|cc| Ok(Box::new(AppSim::new(cc)))),
    )
    .unwrap();
}

struct AppSim {
    snarl: Arc<Mutex<Snarl<Node>>>,
    sim_engine: SimulationEngine,
}
impl App for AppSim {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.sim_engine.is_running() {
                    if ui.button("⏸ Pause").clicked() {
                        self.sim_engine.stop();
                    }
                } else {
                    if ui.button("▶ Start").clicked() {
                        self.sim_engine.start();
                    }
                }
                ui.label(format!("Status: {}",
                    if self.sim_engine.is_running() { "Running" } else { "Paused" }
                ));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(mut snarl) = self.snarl.lock() {
                SnarlWidget::new()
                    .id(egui::Id::new("snarl-min"))
                    .style(SnarlStyle {
                        pin_placement: Some(PinPlacement::Edge),
                        ..Default::default()
                    })
                    .show(&mut *snarl, &mut views::Viewer::Viewer, ui);
            }
        });

        // Request repaint at lower rate when simulation is running
        if self.sim_engine.is_running() {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
    }
}

impl AppSim {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut snarl = Snarl::new();
        // Create two numbers, one adder, one output and connect them
        let pid_para = PID_para {
            P: 0.1,
            I: 1.,
            D: -100.,
            dt: 0.001,
            init_I: 0.,
            offset: 0.,
            dI: (-1., 1.),
            dE: (-1., 1.),
        };
        let pid_in= PID_input{set: 0., act: 0., min:-1000., max:1000.};
        let pid = PID::new(pid_para);
        let PID = snarl.insert_node(egui::pos2(250.0, -50.0), Node::Control(ControlNode::PID(pid,pid_in,0.)));
        let PID_2 = snarl.insert_node(egui::pos2(250.0, 50.0), Node::Control(ControlNode::PID(pid,pid_in,0.)));
        let inp = snarl.insert_node(egui::pos2(0.0, 0.0), Node::Control(ControlNode::Num_input(0.0)));
        let PT1 =snarl.insert_node(egui::pos2(250.0, -150.0), Node::Control(ControlNode::PT1(pt1::new(1000.0, 1.),0.,0.)));
        let PT1_2 =snarl.insert_node(egui::pos2(250.0, 150.0), Node::Control(ControlNode::PT1(pt1::new(1000.0, 1.),0.,0.)));
        let outp = snarl.insert_node(egui::pos2(800.0, -22.5), Node::Control(ControlNode::Num_output(3)));

        let PID_out = OutPinId { node: PID, output: 0 };
        let PID_set = InPinId { node: PID, input: 0 };
        let PID_act = InPinId { node: PID, input: 1 };
        let PID_out_2 = OutPinId { node: PID_2, output: 0 };
        let PID_set_2 = InPinId { node: PID_2, input: 0 };
        let PID_act_2 = InPinId { node: PID_2, input: 1 };
        let NumIn_out = OutPinId { node: inp, output: 0 };
        let PT1_out = OutPinId { node: PT1, output: 0 };
        let PT1_in = InPinId { node: PT1, input: 0 };
        let PT1_out_2 = OutPinId { node: PT1_2, output: 0 };
        let PT1_in_2 = InPinId { node: PT1_2, input: 0 };
        let outp_0 = InPinId { node: outp, input: 0 };
        let outp_1 = InPinId { node: outp, input: 1 };
        let outp_2 = InPinId { node: outp, input: 2 };

        snarl.connect(NumIn_out, PID_set);
        snarl.connect(PT1_out, PID_act);
        snarl.connect(PID_out, PT1_in);
        snarl.connect(NumIn_out, PID_set_2);
        snarl.connect(PT1_out_2, PID_act_2);
        snarl.connect(PID_out_2, PT1_in_2);
        snarl.connect(NumIn_out, outp_1);
        snarl.connect(PID_out, outp_0);
        snarl.connect(PID_out_2, outp_2);

        let snarl = Arc::new(Mutex::new(snarl));
        let sim_engine = SimulationEngine::new(Arc::clone(&snarl), 10); // 10ms timestep

        Self { snarl, sim_engine }
    }
}
