use crate::Helper::norm::norm_FN;
use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct PID<T, Y>
where
    T: Copy + Clone + std::fmt::Debug + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T> + Div<Output = Y> + PartialOrd + Zero,
    Y: Copy + Mul<T, Output = T>,
{
    P: f64,
    I: f64,
    D: f64,
    pub Pro: T,
    pub Int: T,
    pub Der: T,
    err_old: T,
    err_int: T,
    offset: T,
    pub dt: f64,
    dI: (T, T),
    dE: (T, T),
}

impl<T, Y> PID<T, Y>
where
    T: Copy + Clone + std::fmt::Debug + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T> + Div<Output = Y> + PartialOrd + Zero,
    Y: Copy + Mul<T, Output = T>,
{
    pub fn new(para: PID_para<T>) -> Self {
        Self {
            P: para.P,
            I: para.I,
            D: para.D,
            err_old: T::zero(),
            err_int: para.init_I,
            Pro: T::zero(),
            Der: T::zero(),
            Int: para.init_I * para.I * para.dt,
            offset: para.offset,
            dt: para.dt,
            dI: para.dI,
            dE: para.dE,
        }
    }

    pub fn call(&mut self, input: PID_input<T>, reset: bool, reset_out: T) -> T {
        let set = norm_FN(&input.set, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let act = norm_FN(&input.act, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let err = set - act;
        let err_int_old = self.err_int;
        let mut out = reset_out;

        if reset == false {
            self.err_int = self.err_int + err;
            self.Pro = err * self.P;
            self.Int = self.err_int * self.I * self.dt;
            self.Der = (err - self.err_old) * self.D * self.dt;
            let s = self.Pro + self.Int + self.Der;

            let outr = norm_FN(
                &s, &self.dE.0, &self.dE.1,
                &input.min, &input.max);
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
            let outr = norm_FN(
                &out, &self.dE.0, &self.dE.1,
                &input.min, &input.max,
            );
            self.err_int = (outr - self.Pro - self.Der) / (self.I * self.dt);
        }

        if out <= input.min || out >= input.max {
            self.err_int = err_int_old;
        }

        self.err_old = err;
        return out;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PID_input<T> {
    pub set: T,
    pub act: T,
    pub min: T,
    pub max: T,
}

#[derive(Copy, Clone, Debug)]
pub struct PID_para<T> {
    pub P: f64,
    pub I: f64,
    pub D: f64,
    pub dt: f64,
    pub init_I: T,
    pub offset: T,
    pub dI: (T, T),
    pub dE: (T, T),
}