mod nodes;
mod views;
use crate::nodes::Node;
use eframe::{App, Frame};
use egui::Context;
use egui_snarl::ui::{PinPlacement, SnarlStyle, SnarlViewer, SnarlWidget};
use egui_snarl::Snarl;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Snarl Test App",
        native_options,
        Box::new(|cc| Ok(Box::new(AppSim::new(cc)))),
    )
    .unwrap();
}

struct AppSim {
    snarl: Snarl<Node>,
}
impl App for AppSim {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            SnarlWidget::new()
                .id(egui::Id::new("snarl-min"))
                .style(SnarlStyle {
                    pin_placement: Some(PinPlacement::Edge),
                    ..Default::default()
                })
                .show(&mut self.snarl, &mut views::Viewer::Viewer, ui);
        });
    }
}

impl AppSim {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut snarl = Snarl::new();
        // Create two numbers, one adder, one output and connect them
        // let n1 = snarl.insert_node(egui::pos2(50.0, 50.0), Node::Number(1.0));
        // let n2 = snarl.insert_node(egui::pos2(50.0, 150.0), Node::Number(2.0));
        // let add = snarl.insert_node(egui::pos2(250.0, 100.0), Node::Add { sum: 0.0 });
        // let out = snarl.insert_node(egui::pos2(450.0, 100.0), Node::PID);
        // let Bound0 = snarl.insert_node(egui::pos2(10.0, 10.0), Node::Gas(GasNode::Boundary(GasState::Air(), 0)));
        // let Bound1 = snarl.insert_node(egui::pos2(500.0, 50.0), Node::Gas(GasNode::Boundary(GasState::Air(), 1)));

        // Connect n1 -> add.in0, n2 -> add.in1, add.out -> out.in0
        // let b0_out = OutPinId { node: Bound0, output: 0 };
        // let b1_in = InPinId { node: Bound1, input: 0 };
        // let add_in0 = InPinId { node: add, input: 0 };
        // let add_in1 = InPinId { node: add, input: 1 };
        // let add_out = OutPinId { node: add, output: 0 };
        // let out_in0 = InPinId { node: out, input: 0 };

        // snarl.connect(b0_out, b1_in);
        // snarl.connect(n2_out, add_in1);
        // snarl.connect(add_out, out_in0);

        Self { snarl }
    }
}
