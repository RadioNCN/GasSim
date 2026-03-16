use crate::nodes::GasNodes::Boundary_node::BoundaryType;
use crate::nodes::{Node, NodeViewer};
use egui::Ui;
use egui_snarl::ui::PinInfo;
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use uom::si::f64::{MassRate, Pressure, Ratio, ThermodynamicTemperature, Time};
use uom::si::mass_rate::gram_per_second;
use uom::si::pressure::bar;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;
use GasSim::modules::state::GasState;

pub mod Boundary_node;

#[derive(Clone, Debug)]
pub enum GasNode {
    Boundary(GasState, usize),
}

impl NodeViewer for GasNode {
    fn title(&self) -> String {
        match self {
            GasNode::Boundary(GS, n) => "Boundary".to_string(),
        }
    }

    fn inputs(&self) -> usize {
        match self {
            GasNode::Boundary(_, n) => *n,
        }
    }

    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match &snarl[pin.id.node] {
            Node::Gas(GasNode::Boundary(GS, _)) => {
                let input = pin
                    .remotes
                    .last()
                    .and_then(|remote| BoundaryType::value_of_outpin(&*snarl, *remote));
                match input {
                    Some(GS_US) => {
                        GasNode::show_state(ui, &GS_US);
                        PinInfo::square().with_fill(BoundaryType::Boundary(GS.clone()).pin_color())
                    }
                    None => return PinInfo::circle().with_fill(egui::Color32::RED),
                }
            }
            _ => PinInfo::circle().with_fill(egui::Color32::RED),
        }
    }

    fn outputs(&self) -> usize {
        match self {
            GasNode::Boundary(_, _) => 1,
            _ => 0,
        }
    }

    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo {
        match &self {
            GasNode::Boundary(GS, n) => {
                PinInfo::square().with_fill(BoundaryType::Boundary(GS.clone()).pin_color())
            }
            _ => PinInfo::circle().with_fill(egui::Color32::RED),
        }
    }

    fn has_body(&mut self, node: &Node) -> bool {
        true
    }

    fn show_body(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut Ui) {
        match self {
            GasNode::Boundary(GS, n) => {
                ui.add(egui::DragValue::new(n).speed(0.1));
                GasNode::set_state(ui, GS);
            }
            _ => println!("Error: Node is not a Boundary"),
        }
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        // Only allow numeric: from Number/Add to Add/Boundary
        let from_ok = matches!(snarl[from.id.node], Node::Gas(_));
        let to_ok = matches!(snarl[to.id.node], Node::Gas(_));
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

        if ui.button("Boundary").clicked() {
            snarl.insert_node(pos, Node::Gas(GasNode::Boundary(GasState::Air(), 1)));
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

impl GasNode {
    pub fn set_state(ui: &mut egui::Ui, gs: &mut GasState) {
        let mut gsm = gs.to_mass_rate();
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let mut p = gsm.p.get::<bar>();
            ui.horizontal(|ui| {
                ui.label("Pressure / bar");
                ui.add(egui::DragValue::new(&mut p).speed(0.01));
                gsm.p = Pressure::new::<bar>(p);
            });
            let mut T = gsm.T.get::<degree_celsius>();
            ui.horizontal(|ui| {
                ui.label("Temperatur / °C");
                ui.add(egui::DragValue::new(&mut T).speed(0.01));
                gsm.T = ThermodynamicTemperature::new::<degree_celsius>(T);
            });
            let mut m = gsm.m.get::<gram_per_second>();
            ui.horizontal(|ui| {
                ui.label("Mass rate / g s^-1");
                ui.add(egui::DragValue::new(&mut m).speed(0.01));
                gsm.m = MassRate::new::<gram_per_second>(m);
            });
            let mut H2 = gsm.H2.get::<ratio>();
            ui.horizontal(|ui| {
                ui.label("H2 / -");
                ui.add(egui::DragValue::new(&mut H2).speed(0.01));
                gsm.H2 = Ratio::new::<ratio>(H2);
            });
            let mut H2O = gsm.H2O.get::<ratio>();
            ui.horizontal(|ui| {
                ui.label("H2O / -");
                ui.add(egui::DragValue::new(&mut H2O).speed(0.01));
                gsm.H2O = Ratio::new::<ratio>(H2O);
            });
            let mut O2 = gsm.O2.get::<ratio>();
            ui.horizontal(|ui| {
                ui.label("O2 / -");
                ui.add(egui::DragValue::new(&mut O2).speed(0.01));
                gsm.O2 = Ratio::new::<ratio>(O2);
            });
            let mut N2 = gsm.N2.get::<ratio>();
            ui.horizontal(|ui| {
                ui.label("N2 / -");
                ui.add(egui::DragValue::new(&mut N2).speed(0.01));
                gsm.N2 = Ratio::new::<ratio>(N2);
            });
        });
        gs.update_mass_rate(gsm, Time::new::<second>(1.0));
    }
    pub fn show_state(ui: &mut egui::Ui, gs: &GasState) {
        let mut gsm = gs.to_mass_rate();
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            let p = gsm.p.get::<bar>();
            ui.horizontal(|ui| {
                ui.label(format!("{:.3?} / bar", p));
            });
            let T = gsm.T.get::<degree_celsius>();
            ui.horizontal(|ui| {
                ui.label(format!("{:.3?} / °C", T));
            });
            let m = gsm.m.get::<gram_per_second>();
            ui.horizontal(|ui| {
                ui.label(format!("{:.3?} / g s^-1", m));
            });
        });
    }
}
