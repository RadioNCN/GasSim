use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use egui_snarl::{NodeId, Snarl};
use crate::nodes::Node;
use crate::nodes::ControlNodes::ControlNode;

/// Simulation state that runs independently of UI
pub struct SimulationEngine {
    snarl: Arc<Mutex<Snarl<Node>>>,
    running: Arc<Mutex<bool>>,
    timestep: Duration,
    start_time: Instant,
}

impl SimulationEngine {
    pub fn new(snarl: Arc<Mutex<Snarl<Node>>>, timestep_ms: u64) -> Self {
        Self {
            snarl,
            running: Arc::new(Mutex::new(false)),
            timestep: Duration::from_millis(timestep_ms),
            start_time: Instant::now(),
        }
    }

    /// Start simulation in background thread
    pub fn start(&self) {
        let snarl = Arc::clone(&self.snarl);
        let running = Arc::clone(&self.running);
        let timestep = self.timestep;
        let start_time = self.start_time;

        *running.lock().unwrap() = true;

        thread::spawn(move || {
            let mut last_update = Instant::now();

            while *running.lock().unwrap() {
                let now = Instant::now();
                let elapsed = now.duration_since(last_update);

                if elapsed >= timestep {
                    // Perform simulation step
                    let current_time = now.duration_since(start_time).as_secs_f64();
                    if let Ok(mut snarl) = snarl.lock() {
                        Self::simulation_step(&mut snarl, current_time);
                    }
                    last_update = now;
                }

                // Sleep for a short time to avoid busy-waiting
                // thread::sleep(Duration::from_micros(100));
            }
        });
    }

    /// Stop simulation
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    /// Single simulation step - processes all nodes
    fn simulation_step(snarl: &mut Snarl<Node>, current_time: f64) {
        // Get all node IDs
        let node_ids: Vec<NodeId> = snarl.node_ids().map(|(id, _)| id).collect();

        // First pass: collect input values for all nodes
        for node_id in &node_ids {
            Self::update_node_inputs(*node_id, snarl, current_time);
        }

        // Second pass: calculate outputs for all nodes
        for node_id in &node_ids {
            if let Some(node) = snarl.get_node_mut(*node_id) {
                match node {
                    Node::Control(control_node) => {
                        Self::process_control_node(control_node);
                    }
                    Node::Gas(_) => {
                        // Gas nodes processing if needed
                    }
                }
            }
        }
    }

    /// Update input values from connected nodes
    fn update_node_inputs(node_id: NodeId, snarl: &mut Snarl<Node>, current_time: f64) {
        // Get input pins for this node
        let input_count = match &snarl[node_id] {
            Node::Control(ControlNode::PID(..)) => 2,
            Node::Control(ControlNode::PT1(..)) => 1,
            Node::Control(ControlNode::Plot(_, n)) => *n,
            _ => 0,
        };

        for input_idx in 0..input_count {
            let in_pin = egui_snarl::InPinId { node: node_id, input: input_idx };

            // Find connected output
            if let Some(out_pin) = snarl.in_pin(in_pin).remotes.first() {
                let input_value = match &snarl[out_pin.node] {
                    Node::Control(ControlNode::PID(_, _, out)) => Some(*out),
                    Node::Control(ControlNode::Num_input(out)) => Some(*out),
                    Node::Control(ControlNode::PT1(_, _, out)) => Some(*out),
                    _ => None,
                };

                if let Some(value) = input_value {
                    // Update the input on target node
                    match &mut snarl[node_id] {
                        Node::Control(ControlNode::PID(_, inp, _)) => {
                            match input_idx {
                                0 => inp.set = value,
                                1 => inp.act = value,
                                _ => {}
                            }
                        }
                        Node::Control(ControlNode::PT1(_, inp, _)) => {
                            *inp = value;
                        }
                        Node::Control(ControlNode::Plot(histories, _)) => {
                            if let Some(history) = histories.get_mut(input_idx) {
                                history.push(current_time, value);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Process a single control node
    fn process_control_node(node: &mut ControlNode) {
        match node {
            ControlNode::PID(pid, input, output) => {
                *output = pid.call(input, false, 0.0);
            }
            ControlNode::PT1(pt1, input, output) => {
                *output = pt1.call(*input);
            }
            ControlNode::Add(A, B, output) => {
                *output = *A + *B;
            }
            _ => {}
        }
    }
}
