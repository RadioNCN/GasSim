mod nodes;
mod views;
use nodes::collection::Node;

use eframe::{App, Frame};
use egui::Context;
use egui_snarl::{InPin, OutPin, Snarl};
use egui_snarl::ui::{PinInfo, SnarlViewer, SnarlWidget, SnarlStyle, PinPlacement};
use egui_snarl::{InPinId, NodeId, OutPinId};

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
                .style(SnarlStyle { pin_placement: Some(PinPlacement::Edge), ..Default::default() })
                .show(&mut self.snarl, &mut views::Viewer::Viewer, ui);
        });
    }
}

impl AppSim {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut snarl = Snarl::new();
        // Create two numbers, one adder, one output and connect them
        let n1 = snarl.insert_node(egui::pos2(50.0, 50.0), Node::Number(1.0));
        let n2 = snarl.insert_node(egui::pos2(50.0, 150.0), Node::Number(2.0));
        let add = snarl.insert_node(egui::pos2(250.0, 100.0), Node::Add { sum: 0.0 });
        let out = snarl.insert_node(egui::pos2(450.0, 100.0), Node::Output);

        // Connect n1 -> add.in0, n2 -> add.in1, add.out -> out.in0
        let n1_out = OutPinId { node: n1, output: 0 };
        let n2_out = OutPinId { node: n2, output: 0 };
        let add_in0 = InPinId { node: add, input: 0 };
        let add_in1 = InPinId { node: add, input: 1 };
        let add_out = OutPinId { node: add, output: 0 };
        let out_in0 = InPinId { node: out, input: 0 };

        snarl.connect(n1_out, add_in0);
        snarl.connect(n2_out, add_in1);
        snarl.connect(add_out, out_in0);

        Self { snarl }
    }
}