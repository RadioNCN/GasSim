use crate::Helper::norm::norm_FN;
use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct PID<I, O, Y>
where
    I: Copy
        + Clone
        + std::fmt::Debug
        + Add<Output = I>
        + Sub<Output = I>
        + Div<Output = Y>
        + Div<f64, Output = I>,
    O: Copy
        + Clone
        + std::fmt::Debug
        + Add<Output = O>
        + Sub<Output = O>
        + Mul<f64, Output = O>
        + Div<f64, Output = O>
        + Div<Output = Y>
        + PartialOrd
        + Zero,
    Y: Copy + Mul<O, Output = O>,
{
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub Pro: O,
    pub Int: O,
    pub Der: O,
    err_old: O,
    err_int: O,
    offset: O,
    pub dt: f64,
    dI: (I, I),
    dE: (O, O),
}

impl<I, O, Y> PID<I, O, Y>
where
    I: Copy
        + Clone
        + std::fmt::Debug
        + Add<Output = I>
        + Sub<Output = I>
        + Div<Output = Y>
        + Div<f64, Output = I>,
    O: Copy
        + Clone
        + std::fmt::Debug
        + Add<Output = O>
        + Sub<Output = O>
        + Mul<f64, Output = O>
        + Div<f64, Output = O>
        + Div<Output = Y>
        + PartialOrd
        + Zero,
    Y: Copy + Mul<O, Output = O>,
{
    pub fn new(para: PID_para<I, O>) -> Self {
        Self {
            kp: para.P,
            ki: para.I,
            kd: para.D,
            err_old: O::zero(),
            err_int: para.init_I,
            Pro: O::zero(),
            Der: O::zero(),
            Int: para.init_I * para.I * para.dt,
            offset: para.offset,
            dt: para.dt,
            dI: para.dI,
            dE: para.dE,
        }
    }

    pub fn call(&mut self, input: PID_input<I, O>, reset: bool, reset_out: O) -> O {
        let set = norm_FN(&input.set, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let act = norm_FN(&input.act, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let err = set - act;
        let err_int_old = self.err_int;
        let mut out = reset_out;

        if reset == false {
            self.err_int = self.err_int + err;
            self.Pro = err * self.kp;
            self.Int = self.err_int * self.ki * self.dt;
            self.Der = (err - self.err_old) * self.kd * self.dt;
            let s = self.Pro + self.Int + self.Der;

            let outr = norm_FN(&s, &self.dE.0, &self.dE.1, &input.min, &input.max);
            let lower = input.min + self.offset;
            let upper = input.max - self.offset;
            let clamped = if outr < lower {
                lower
            } else if outr > upper {
                upper
            } else {
                outr
            };
            out = clamped + self.offset;
        } else {
            let outr = norm_FN(&out, &self.dE.0, &self.dE.1, &input.min, &input.max);
            self.err_int = (outr - self.Pro - self.Der) / (self.ki * self.dt);
        }

        if out <= input.min || out >= input.max {
            self.err_int = err_int_old;
        }

        self.err_old = err;
        return out;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PID_input<I, O> {
    pub set: I,
    pub act: I,
    pub min: O,
    pub max: O,
}

#[derive(Copy, Clone, Debug)]
pub struct PID_para<I, O> {
    pub P: f64,
    pub I: f64,
    pub D: f64,
    pub dt: f64,
    pub init_I: O,
    pub offset: O,
    pub dI: (I, I),
    pub dE: (O, O),
}
