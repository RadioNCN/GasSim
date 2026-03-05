use egui::Ui;
use egui_snarl::{InPin, NodeId, OutPin, OutPinId, Snarl};
use egui_snarl::ui::PinInfo;
use uom::si::f64::{MassRate, Pressure, Ratio, ThermodynamicTemperature, Time};
use uom::si::mass_rate::gram_per_second;
use uom::si::pressure::{bar, pascal};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;
use GasSim::modules::state::GasState;
use crate::nodes::GasNodes::GasNode;
use crate::nodes::Node;




pub enum BoundaryType {
    Boundary(GasState)
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
            BoundaryType::Boundary(_) => egui::Color32::BLUE
        }
    }
    pub fn value_of_outpin(snarl: &Snarl<Node>, out_pin: OutPinId) -> Option<GasState> {
        match &snarl[out_pin.node] {
            Node::Gas(GasNode::Boundary(GS, _)) => Some(GS.clone()),
            _ => None
        }
    }
    pub fn value(&self) -> GasState {
        match self {
            BoundaryType::Boundary(GS) => GS.clone()
        }
    }
}
