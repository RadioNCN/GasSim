mod nodes;
mod views;
use crate::nodes::Node;
use eframe::{App, Frame};
use egui::Context;
use egui_snarl::ui::{PinPlacement, SnarlStyle, SnarlViewer, SnarlWidget};
use egui_snarl::{InPinId, OutPinId, Snarl};
use GasSim::modules::PID::{PID_input, PID_para, PID};
use GasSim::modules::PT1::pt1;
use crate::nodes::ControlNodes::ControlNode;

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
        let pid_para = PID_para {
            P: 0.1,
            I: 1.,
            D: 1.,
            dt: 0.01,
            init_I: 0.,
            offset: 0.,
            dI: (-1., 1.),
            dE: (-1., 1.),
        };
        let pid_in= PID_input{set: 0., act: 0., min:-10., max:10.};
        let pid = PID::new(pid_para);
        let PID = snarl.insert_node(egui::pos2(250.0, 50.0), Node::Control(ControlNode::PID(pid,pid_in,0.)));
        let inp = snarl.insert_node(egui::pos2(50.0, 50.0), Node::Control(ControlNode::Num_input(0.0)));
        let PT1 =snarl.insert_node(egui::pos2(250.0, 150.0), Node::Control(ControlNode::PT1(pt1::new(1000.0, 10.),0.,0.)));

        let PID_out = OutPinId { node: PID, output: 0 };
        let PID_set = InPinId { node: PID, input: 0 };
        let PID_act = InPinId { node: PID, input: 1 };
        let NumIn_out = OutPinId { node: inp, output: 0 };
        let PT1_out = OutPinId { node: PT1, output: 0 };
        let PT1_in = InPinId { node: PT1, input: 0 };

        snarl.connect(NumIn_out, PID_set);
        snarl.connect(PT1_out, PID_act);
        snarl.connect(PID_out, PT1_in);

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
