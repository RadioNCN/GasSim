use egui::Ui;
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use egui_snarl::ui::PinInfo;
use GasSim::modules::PID::{PID_para, PID};
use GasSim::modules::state::GasState;
use crate::nodes::{Node, NodeViewer};
use crate::nodes::GasNodes::GasNode;

pub mod PID_node;

#[derive(Clone, Debug)]
pub enum ControlNode {
    PID(PID<f64, f64, f64>),
    Num_input(usize),
    Num_output(usize),
}

impl NodeViewer for ControlNode {
    fn title(&self) -> String {
        match self {
            ControlNode::PID(_) => "PID".to_string(),
            ControlNode::Num_input(_) => "Number Input".to_string(),
            ControlNode::Num_output(_) => "Number Output".to_string(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            ControlNode::PID(_) => 2,
            ControlNode::Num_input(_) => 0,
            ControlNode::Num_output(n) => *n,
        }
    }
    fn show_input(&self, pin: &InPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match &snarl[pin.id.node] {
            Node::Control(ControlNode::PID(_)) => {
                PinInfo::circle().with_fill(egui::Color32::WHITE) },
            Node::Control(ControlNode::Num_input(_)) => PinInfo::circle().with_fill(egui::Color32::WHITE),
            Node::Control(ControlNode::Num_output(_)) => PinInfo::circle().with_fill(egui::Color32::WHITE),
            _ => PinInfo::circle().with_fill(egui::Color32::RED)
        }
    }
    fn outputs(&self) -> usize {
        match self {
            ControlNode::PID(_) => 1,
            ControlNode::Num_input(n) => *n,
            ControlNode::Num_output(_) => 0,
        }
    }
    fn show_output(&self, pin: &OutPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match &self {
            ControlNode::PID(_) => PinInfo::circle().with_fill(egui::Color32::WHITE),
            ControlNode::Num_input(_) => { PinInfo::circle().with_fill(egui::Color32::WHITE) },
            ControlNode::Num_output(n) => PinInfo::circle().with_fill(egui::Color32::WHITE),
        }
    }
    fn has_body(&mut self, node: &Node) -> bool {
        true
    }
    fn show_body(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut Ui) {
        match self {
            ControlNode::PID(pid) => {
                ui.label("Kp");
                ui.add(egui::DragValue::new(&mut pid.kp).speed(0.01));
                ui.label("Ki");
                ui.add(egui::DragValue::new(&mut pid.ki).speed(0.01));
                ui.label("Kd");
                ui.add(egui::DragValue::new(&mut pid.kd).speed(0.01));
            }
            ControlNode::Num_input(n) => {ui.add(egui::DragValue::new(n).speed(0.1));},
            ControlNode::Num_output(n) => {ui.add(egui::DragValue::new(n).speed(0.1));},
            _ => {}
        }
    }
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let from_ok = matches!(snarl[from.id.node],  Node::Control(_));
        let to_ok = matches!(snarl[to.id.node],  Node::Control(_));
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
            let pid_para =PID_para{P:1., I:1., D:1., dt:0.01, init_I:0., offset:0., dI:(-1.,1.), dE:(-1.,1.)};
            let pid = PID::new(pid_para);
            snarl.insert_node(pos, Node::Control(ControlNode::PID(pid)));
            ui.close();
        }

        if ui.button("Input").clicked() {
            snarl.insert_node(pos, Node::Control(ControlNode::Num_input(1)));
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

    fn show_node_menu(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut egui::Ui, snarl: &mut Snarl<Node>,
    ) {
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close();
        }
    }

}