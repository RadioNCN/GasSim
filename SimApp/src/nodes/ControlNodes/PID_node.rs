use crate::nodes::ControlNodes::ControlNode;

pub fn calc(node: &mut ControlNode, ui: &mut egui::Ui) {
    match node {
        ControlNode::PID(pid, inp, out)=> {
            let input = inp;
            let outp= pid.call(input, false, 0.0);
            *out = outp;
        },
        _=> {  }
    }
}
