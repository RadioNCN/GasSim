pub mod ControlNodes;
pub mod GasNodes;

use crate::nodes::ControlNodes::ControlNode;
use egui::Ui;
use egui_snarl::ui::PinInfo;
use egui_snarl::{InPin, NodeId, OutPin, Snarl};
use GasNodes::GasNode;

#[derive(Clone, Debug)]
pub enum Node {
    Gas(GasNode),
    Control(ControlNode),
}

pub trait NodeViewer {
    fn title(&self) -> String;
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn show_input(&mut self, pin: &InPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo;
    fn show_output(&mut self, pin: &OutPin, ui: &mut Ui, snarl: &Snarl<Node>) -> PinInfo;
    fn has_body(&mut self, node: &Node) -> bool;
    fn show_body(&mut self, node: NodeId, inputs: &[InPin], outputs: &[OutPin], ui: &mut Ui);
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>);
    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<Node>) -> bool;
    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut egui::Ui, snarl: &mut Snarl<Node>);
    fn has_node_menu(&mut self, _node: &Node) -> bool;
    fn show_node_menu(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    );
}
