use crate::nodes::GasNodes::GasNode;
use crate::nodes::Node;
use egui_snarl::{OutPinId, Snarl};
use GasSim::modules::state::GasState;

pub enum BoundaryType {
    Boundary(GasState),
}

impl BoundaryType {
    pub fn header_string(&self) -> String {
        match self {
            BoundaryType::Boundary(GS) => format!(": {:.3?}", GS.TP()),
        }
    }
    pub fn pin_string(&self) -> String {
        match self {
            BoundaryType::Boundary(GS) => format!(": {:?}", GS.to_mass_rate()),
        }
    }
    pub fn pin_color(&self) -> egui::Color32 {
        match self {
            BoundaryType::Boundary(_) => egui::Color32::BLUE,
        }
    }
    pub fn value_of_outpin(snarl: &Snarl<Node>, out_pin: OutPinId) -> Option<GasState> {
        match &snarl[out_pin.node] {
            Node::Gas(GasNode::Boundary(GS, _)) => Some(GS.clone()),
            _ => None,
        }
    }
    pub fn value(&self) -> GasState {
        match self {
            BoundaryType::Boundary(GS) => GS.clone(),
        }
    }
}
