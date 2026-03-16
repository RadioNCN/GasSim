use crate::nodes::{Node, NodeViewer};
use egui::Ui;
use egui_snarl::ui::PinInfo;
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use GasSim::modules::{PID::{PID_input, PID_para, PID}, PT1::pt1};
use crate::nodes::GasNodes::Boundary_node::BoundaryType;
use crate::nodes::GasNodes::GasNode;

pub mod PID_node;

#[derive(Clone, Debug)]
pub enum ControlNode {
    PID(PID<f64, f64, f64>, PID_input<f64, f64>, f64),
    Num_input(f64),
    Num_output(usize),
    PT1(pt1,f64, f64)
}

impl NodeViewer for ControlNode {
    fn title(&self) -> String {
        match self {
            ControlNode::PID(_,_,_) => "PID".to_string(),
            ControlNode::Num_input(_) => "Number Input".to_string(),
            ControlNode::Num_output(_) => "Number Output".to_string(),
            ControlNode::PT1(_,_,_) => "PT1".to_string(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            ControlNode::PID(_,_,_) => 2,
            ControlNode::Num_input(_) => 0,
            ControlNode::Num_output(n) => *n,
            ControlNode::PT1(_,_,_) => 1,
        }
    }
    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match self {
            ControlNode::PID(_,inp,_) =>{
                let input = pin.remotes.last().and_then(|remote|
                    match &snarl[remote.node]{
                        Node::Control(ControlNode::PID(_,_, out)) => Some(out.clone()),
                        Node::Control(ControlNode::Num_input(out)) => Some(out.clone()),
                        Node::Control(ControlNode::PT1(_,_, out)) => Some(out.clone()),
                        _ => None
                    });

                match input {
                    Some(input) => {
                        ControlNode::show_state(ui, &input);
                        match pin.id.input{
                            0 => inp.set = input,
                            1 => inp.act = input,
                            _ => { todo!() },
                        }

                        PinInfo::circle().with_fill(egui::Color32::WHITE)
                    },
                    None => return PinInfo::circle().with_fill(egui::Color32::RED),
                }
                     },
            ControlNode::Num_input(_) => {
                PinInfo::circle().with_fill(egui::Color32::WHITE)
            }
            ControlNode::Num_output(_) => {
                let input = pin
                    .remotes
                    .last()
                    .and_then(|remote|
                                  match &snarl[remote.node]{
                                      Node::Control(ControlNode::PID(_,_, out)) => Some(out.clone()),
                                      Node::Control(ControlNode::Num_input(out)) => Some(out.clone()),
                                      Node::Control(ControlNode::PT1(_,_, out)) => Some(out.clone()),
                                      _ => None
                                  });
                match input {
                    Some(n) => {
                        ControlNode::show_state(ui, &n);
                        PinInfo::circle().with_fill(egui::Color32::WHITE)
                    }
                    None => return PinInfo::circle().with_fill(egui::Color32::RED),
                }
            }
            ControlNode::PT1(_,inp,_) => {
                let input = pin
                    .remotes
                    .last()
                    .and_then(|remote|
                        match &snarl[remote.node]{
                            Node::Control(ControlNode::PID(_,_, out)) => Some(out.clone()),
                            Node::Control(ControlNode::Num_input(out)) => Some(out.clone()),
                            _ => None
                        });
                match input {
                    Some(n) => {
                        ControlNode::show_state(ui, &n);
                        *inp=n;
                        PinInfo::circle().with_fill(egui::Color32::WHITE)
                    }
                    None => return PinInfo::circle().with_fill(egui::Color32::RED),
                } },
            _ => PinInfo::circle().with_fill(egui::Color32::RED),
        }
    }
    fn outputs(&self) -> usize {
        match self {
            ControlNode::PID(_,_,_) => 1,
            ControlNode::Num_input(_) => 1,
            ControlNode::Num_output(_) => 0,
            ControlNode::PT1(_,_,_) => 1,
        }
    }
    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match &self {
            ControlNode::PID(_, ..) => PinInfo::circle().with_fill(egui::Color32::WHITE),
            ControlNode::Num_input(outp) => {
                PinInfo::circle().with_fill(egui::Color32::WHITE) },
            ControlNode::Num_output(n) => PinInfo::circle().with_fill(egui::Color32::WHITE),
            ControlNode::PT1(_,_,outp) => {
                ui.label(format!("{:.3?}", outp));
                PinInfo::circle().with_fill(egui::Color32::WHITE) },
             _ => PinInfo::circle().with_fill(egui::Color32::RED),
        }
    }
    fn has_body(&mut self, node: &Node) -> bool {
        true
    }
    fn show_body(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut Ui) {
        match self {
            ControlNode::PID(pid,input, out) => {
                ui.label("Kp");
                ui.add(egui::DragValue::new(&mut pid.kp).speed(0.01));
                ui.label("Ki");
                ui.add(egui::DragValue::new(&mut pid.ki).speed(0.01));
                ui.label("Kd");
                ui.add(egui::DragValue::new(&mut pid.kd).speed(0.01));
                ui.label("min");
                ui.add(egui::DragValue::new(&mut input.min).speed(0.01));
                ui.label("max");
                ui.add(egui::DragValue::new(&mut input.max).speed(0.01));
                // ui.label(format!("Output: {:.3}", out));
                // Calculation is done in simulation thread, not here
            }
            ControlNode::Num_input(inp) => {
                ui.add(egui::DragValue::new(inp).speed(0.001));
            }
            ControlNode::Num_output(n) => {
                ui.add(egui::DragValue::new(n).speed(0.1));
            }
            ControlNode::PT1(pt1, inp, out) => {
                ui.label("T");
                ui.add(egui::DragValue::new(&mut pt1.t).speed(0.01));
                // ui.label(format!("Input: {:.3}", inp));
                // ui.label(format!("Output: {:.3}", out));
                // Calculation is done in simulation thread, not here
            }
            _ => {}
        }
    }
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let from_ok = matches!(snarl[from.id.node], Node::Control(_));
        let to_ok = matches!(snarl[to.id.node], Node::Control(_));
        if !(from_ok && to_ok) {
            return;
        }
        // enforce one wire per input
        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }
        snarl.connect(from.id, to.id);
    }
    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<Node>) -> bool {
        true
    }
    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut egui::Ui, snarl: &mut Snarl<Node>) {
        ui.label("Add Node");

        if ui.button("PID").clicked() {
            let pid_para = PID_para {
                P: 1.,
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
            snarl.insert_node(pos, Node::Control(ControlNode::PID(pid,pid_in, 0.)));
            ui.close();
        }

        if ui.button("Input").clicked() {
            snarl.insert_node(pos, Node::Control(ControlNode::Num_input(0.0)));
            ui.close();
        }

        if ui.button("Output").clicked() {
            snarl.insert_node(pos, Node::Control(ControlNode::Num_output(1)));
            ui.close();
        }
    }
    fn has_node_menu(&mut self, _node: &Node) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) {
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close();
        }
    }
}

impl ControlNode {
    pub fn show_state(ui: &mut egui::Ui, n: &f64) {
        ui.label(format!("{:.3?}", n));
    }
}
