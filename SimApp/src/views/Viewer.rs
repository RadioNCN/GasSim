use egui::Ui;
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use egui_snarl::ui::{PinInfo, SnarlViewer};
use GasSim::modules::PID::{PID_para, PID};
use GasSim::modules::state::GasState;
use crate::nodes::GasNodes::GasNode;
use crate::nodes::{Node, NodeViewer};
use crate::nodes::ControlNodes::ControlNode;

pub struct Viewer;

impl Viewer {
}

impl SnarlViewer<Node> for Viewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Gas(n) => n.title(),
            Node::Control(n) => n.title(),
            _ => "".to_string()
        }
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Gas(n) => n.inputs(),
            Node::Control(n) => n.inputs(),
            _ => 0
        }
    }

    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &mut Snarl<Node>) -> PinInfo {
        match &snarl[pin.id.node] {
            Node::Gas(n) => n.show_input(pin, ui, snarl),
            Node::Control(n) => n.show_input(pin, ui, snarl),
            _ => PinInfo::circle().with_fill(egui::Color32::RED)
        }
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Gas(n) => n.outputs(),
            Node::Control(n) => n.outputs(),
            _ => 0
        }
    }

    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &mut Snarl<Node>) -> PinInfo {
        match &snarl[pin.id.node] {
            Node::Gas(n) => n.show_output(pin, ui, snarl),
            Node::Control(n) => n.show_output(pin, ui, snarl),
            _ => PinInfo::circle().with_fill(egui::Color32::RED)
        }
    }

    fn has_body(&mut self, node: &Node) -> bool {
        true
    }

    fn show_body(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut Ui, snarl: &mut Snarl<Node>) {
        match snarl[node] {
            Node::Gas(ref mut n) => {
                n.show_body(node, inputs, outputs, ui);
            },
            Node::Control(ref mut n) => {
                n.show_body(node, inputs, outputs, ui);
            }
            _=> println!("Error: Has no body")
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Only allow numeric: from Number/Add to Add/PID
        let from_ok = match snarl[from.id.node] {
            Node::Gas(_) => matches!(snarl[to.id.node], Node::Control(_)),
            Node::Control(_) => matches!(snarl[to.id.node], Node::Control(_)),
        };
        let to_ok = match snarl[to.id.node] {
            Node::Gas(_) => matches!(snarl[from.id.node], Node::Control(_)),
            Node::Control(_) => matches!(snarl[from.id.node], Node::Control(_)),
        };

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

        if ui.button("Bounday").clicked() {
            snarl.insert_node(pos, Node::Gas(GasNode::Boundary(
                GasState::Air(),1)));
            ui.close();
        }
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

    fn show_node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) {
        if ui.button("Remove").clicked() {
            snarl.remove_node(node);
            ui.close();
        }
    }

}