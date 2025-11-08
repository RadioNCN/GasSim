use GasSim::modules::state::GasState;

#[derive(Clone, Debug)]
pub enum Node {
    Number(f64),
    Add { sum: f64 }, // two numeric inputs, one numeric output (cached sum)
    Output,           // one numeric input, no outputs
    Boundary(GasState)
}
pub enum NodeType {
    Number(f64),
    Bool(bool),
    GasState(GasState)
}
impl NodeType {
    pub fn header_string(&self) -> String {
        match self {
            NodeType::Number(n) => format!(": {:.3?}", n),
            NodeType::Bool(b) => format!(": {:?}", b),
            NodeType::GasState(s) => format!("")
        }
    }
    pub fn pin_string(&self) -> String {
        match self {
            NodeType::Number(n) => format!("{:.3?}", n),
            NodeType::Bool(b) => format!("{:?}", b),
            NodeType::GasState(s) => format!("")
        }
    }
    pub fn pin_color(&self) -> egui::Color32 {
        match self {
            NodeType::Number(_) => egui::Color32::GRAY,
            NodeType::Bool(_) => egui::Color32::YELLOW,
            NodeType::GasState(_) => egui::Color32::BLUE
        }
    }  
}