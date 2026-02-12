pub mod Boundary_node;
pub mod nodes;

use GasSim::modules::state::GasState;
use crate::nodes::Boundary_node::GasNode;
#[derive(Clone, Debug)]
pub enum Node {
    Gas(GasNode),
    Control(GasState, usize)
}