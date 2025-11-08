use egui_snarl::{InPin, NodeId, OutPin, OutPinId, Snarl};
use egui_snarl::ui::{PinInfo, SnarlViewer};
use uom::si::f64::Time;
use uom::si::time::second;
use GasSim::modules::state::{pTmx, GasState};
use crate::nodes::collection::{Node, NodeType};

pub struct Viewer;

impl Viewer {
    fn value_of_outpin(snarl: &Snarl<Node>, out_pin: OutPinId) -> Option<NodeType> {
        match &snarl[out_pin.node] {
            Node::Number(v) => Some(NodeType::Number(*v)),
            Node::Add { sum } => Some(NodeType::Number(*sum)),
            Node::Output => None,
            Node::Boundary(gs) => Some(NodeType::GasState(gs.clone())),
        }
    }
}

impl SnarlViewer<Node> for Viewer {
    fn title(&mut self, node: &Node) -> String {
        match node {
            Node::Number(_) => "Number".into(),
            Node::Add { .. } => "Add".into(),
            Node::Output => "Output".into(),
            Node::Boundary(_) => "Boundary".into(),
        }
    }

    fn show_header(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) {
        let base = match &snarl[node] {
            Node::Number(_) => "Number",
            Node::Add { .. } => "Add",
            Node::Output => "Output",
            Node::Boundary(_) => "Boundary",
        };

        let val_opt: Option<NodeType> = match snarl[node] {
            Node::Number(v) => Some(NodeType::Number(v)),
            Node::Add { sum } => Some(NodeType::Number(sum)),
            Node::Output => inputs
                .get(0)
                .and_then(|pin| pin.remotes.last())
                .and_then(|r| Viewer::value_of_outpin(&*snarl, *r)),
            Node::Boundary(ref gs) => Some(NodeType::GasState(gs.clone())),
        };

        let value_text = val_opt.map(|v| v.header_string()).unwrap_or_else(|| "None".into());

        ui.label(format!("{base}{value_text}"));
    }

    fn inputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Number(_) => 0,
            Node::Add { .. } => 2,
            Node::Output => 1,
            Node::Boundary(_) => 0,
        }
    }

    fn show_input(&mut self, pin: &InPin, ui: &mut egui::Ui, snarl: &mut Snarl<Node>) -> PinInfo {
        // show last connected numeric value (if any)
        let val = pin
            .remotes
            .last()
            .and_then(|remote| Viewer::value_of_outpin(snarl, *remote));
        let value_text: String = match val {
            Some(v) => v.pin_string(),
            None => "None".into(),
        };
        // Add labels for inputs: Add node uses A/B, Output node uses In
        let text = match snarl[pin.id.node] {
            Node::Add { .. } => {
                let name = if pin.id.input == 0 { "A" } else { "B" };
                format!("{name}: {value_text}")
            }
            Node::Output => format!("In: {value_text}"),
            Node::Number(_) => value_text,
            Node::Boundary(_) => value_text,
        };
        ui.label(text);
        PinInfo::circle().with_fill(egui::Color32::GREEN)
    }

    fn outputs(&mut self, node: &Node) -> usize {
        match node {
            Node::Output => 0,
            Node::Number(_) | Node::Add { .. } | Node::Boundary(_) => 1,
        }
    }

    // fn has_body(&mut self, node: &Node) -> bool {
    //     matches!(node, Node::Add { .. })
    // }

    // fn show_body(
    //     &mut self,
    //     node: NodeId,
    //     inputs: &[InPin],
    //     _outputs: &[OutPin],
    //     ui: &mut egui::Ui,
    //     snarl: &mut Snarl<Node>,
    // ) {
    //     let _ = ui; // body just computes; no extra UI needed
    //     // First, compute values using an immutable borrow of snarl
    //     let lhs = inputs
    //         .get(0)
    //         .and_then(|pin| pin.remotes.last())
    //         .and_then(|r| Viewer::value_of_outpin(&*snarl, *r))
    //         .unwrap_or(0.0);
    //     let rhs = inputs
    //         .get(1)
    //         .and_then(|pin| pin.remotes.last())
    //         .and_then(|r| Viewer::value_of_outpin(&*snarl, *r))
    //         .unwrap_or(0.0);
    //     // Then, update the node's cached sum with a mutable borrow
    //     if let Node::Add { sum } = &mut snarl[node] {
    //         *sum = lhs + rhs;
    //     }
    // }

    fn show_output(&mut self, pin: &OutPin, ui: &mut egui::Ui, snarl: &mut Snarl<Node>) -> PinInfo {
        match snarl[pin.id.node] {
            Node::Number(ref mut v) => {
                ui.add(egui::DragValue::new(v));
                PinInfo::circle().with_fill(NodeType::Number(*v).pin_color())
            }
            Node::Add { sum } => {
                ui.label(format!("Sum: {sum:.3}"));
                PinInfo::circle().with_fill(NodeType::Number(sum).pin_color())
            }
            Node::Output => unreachable!("Output has no outputs"),
            Node::Boundary(ref gs) => {
                PinInfo::triangle().with_fill(NodeType::GasState(gs.clone()).pin_color())
            }
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Only allow numeric: from Number/Add to Add/Output
        let from_ok = matches!(snarl[from.id.node], Node::Number(_) | Node::Add { .. });
        let to_ok = matches!(snarl[to.id.node], Node::Add { .. } | Node::Output);
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
        ui.label("Add node");
        if ui.button("Number").clicked() {
            snarl.insert_node(pos, Node::Number(0.0));
            ui.close();
        }
        if ui.button("Add").clicked() {
            snarl.insert_node(pos, Node::Add { sum: 0.0 });
            ui.close();
        }
        if ui.button("Output").clicked() {
            snarl.insert_node(pos, Node::Output);
            ui.close();
        }
        if ui.button("Boundary").clicked() {
            snarl.insert_node(pos, Node::Boundary(
                GasState::from_mass_rate(pTmx::default(), Time::new::<second>(1.0))));
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