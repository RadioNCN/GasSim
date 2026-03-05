use crate::Helper::norm;
use crate::Helper::norm::norm_FN;

#[derive(Debug, Copy, Clone)]

pub struct PID {
    P: f64,
    I: f64,
    D: f64,
    pub Pro: f64,
    pub Int: f64,
    pub Der: f64,
    err_old: f64,
    err_int: f64,
    offset: f64,
    pub dt: f64,
    dI: (f64, f64),
    dE: (f64, f64),
}

impl PID {
    pub fn new(para: PID_para) -> Self {
        Self {
            P: para.P,
            I: para.I,
            D: para.D,
            err_old: 0f64,
            err_int: para.init_I,
            Pro: 0f64,
            Der: 0f64,
            Int: para.I * para.init_I * para.dt,
            offset: para.offset,
            dt: para.dt,
            dI: (0.0, 0.0),
            dE: (0.0, 0.0),
        }
    }

    pub fn call(&mut self, input: PID_input, reset: bool, reset_out: f64) -> f64 {
        let set = norm_FN(&input.set, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let act = norm_FN(&input.act, &self.dI.0, &self.dI.1, &self.dE.0, &self.dE.1);
        let err = set - act;
        let err_int_old = self.err_int;
        let mut out = reset_out;

        if reset == false {
            self.err_int += err;
            self.Pro = self.P * err;
            self.Int = self.I * self.err_int * self.dt;
            self.Der = self.D * (err - self.err_old) * self.dt;
            let s = self.Pro + self.Int + self.Der;

            let outr = norm_FN(
                &s, &self.dE.0, &self.dE.1,
                &input.min, &input.max);
            out = outr
                .max(input.min + self.offset)
                .min(input.max - self.offset)
                + self.offset;
        } else {
            let outr = norm_FN(
                &out, &self.dE.0, &self.dE.1,
                &input.min, &input.max,
            );
            self.err_int = (outr - self.Pro - self.Der) / self.I / self.dt;
        }

        if out <= input.min || out >= input.max {
            self.err_int = err_int_old;
        }

        self.err_old = err;
        return out;
    }
}
#[derive(Copy, Clone, Default, Debug)]

pub struct PID_input {
    pub set: f64,
    pub act: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Copy, Clone, Debug)]

pub struct PID_para {
    pub P: f64,
    pub I: f64,
    pub D: f64,
    pub dt: f64,
    pub init_I: f64,
    pub offset: f64,
    pub dI: (f64, f64),
    pub dE: (f64, f64),
}

impl Default for PID_para {
    fn default() -> PID_para {
        PID_para {
            P: 1.,
            I: 1.,
            D: 0.,
            dt: 1e-0,
            init_I: 0.,
            offset: 0.,
            dI: (-1.0, 1.0),
            dE: (-1.0, 1.0),
        }
    }
}