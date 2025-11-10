#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::f64::consts::PI;
use std::ops::{Add, Sub};
use uom::si::dynamic_viscosity::{pascal_second};
use uom::si::kinematic_viscosity::square_meter_per_second;
use uom::si::f64::*;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::molar_heat_capacity::joule_per_kelvin_mole;
use uom::si::pressure::{bar};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
use crate::modules::constants::{FitConst, NatConst};

#[derive(Clone, Debug)]
pub struct pTmx {
    pub p: Pressure,
    pub T: ThermodynamicTemperature,
    pub m: MassRate,
    pub H2: Ratio,
    pub H2O: Ratio,
    pub O2: Ratio,
    pub N2: Ratio,
}

impl Default for pTmx {
    fn default() -> Self {
        Self {
            p: Pressure::new::<bar>(1.0),
            T: ThermodynamicTemperature::new::<kelvin>(333.15),
            m: MassRate::new::<kilogram_per_second>(0.0),
            H2: Ratio::new::<ratio>(0.0),
            H2O: Ratio::new::<ratio>(0.0),
            O2: Ratio::new::<ratio>(0.0),
            N2: Ratio::new::<ratio>(0.0),
        }
    }
}
#[derive(Debug)]
pub struct pTvy {
    pub p: Pressure,
    pub T: ThermodynamicTemperature,
    pub v: VolumeRate,
    pub H2: Ratio,
    pub H2O: Ratio,
    pub O2: Ratio,
    pub N2: Ratio,
}
#[derive(Clone, Debug)]
pub struct GasState{
    pub p: Pressure,
    pub T: ThermodynamicTemperature,
    pub dt: Time,
    pub H2: AmountOfSubstance,
    pub H2O: AmountOfSubstance,
    pub O2: AmountOfSubstance,
    pub N2: AmountOfSubstance,
}
impl GasState{
    pub fn from_mass_rate(pTmx: pTmx, dt: Time) -> GasState{
        let M: Mass = pTmx.m*dt;
        let H2: AmountOfSubstance = M * pTmx.H2 / NatConst::M_H2();
        let H2O: AmountOfSubstance = M * pTmx.H2O / NatConst::M_H2O();
        let O2: AmountOfSubstance = M *pTmx.O2 / NatConst::M_O2();
        let N2: AmountOfSubstance = M * pTmx.N2 / NatConst::M_N2();
        Self{p:pTmx.p, T:pTmx.T, H2, H2O, O2, N2, dt}
    }
    pub fn update_mass_rate(&mut self, pTmx: pTmx, dt: Time){
        let M: Mass = pTmx.m*dt;
        self.p = pTmx.p;
        self.T = pTmx.T;
        self.H2 = M * pTmx.H2 / NatConst::M_H2();
        self.H2O = M * pTmx.H2O / NatConst::M_H2O();
        self.O2 = M *pTmx.O2 / NatConst::M_O2();
        self.N2 = M * pTmx.N2 / NatConst::M_N2();
    }
    
    pub fn from_volume_rate(pTvy: pTvy, dt: Time) -> GasState{
        let V: Volume = pTvy.v*dt;
        let H2: AmountOfSubstance = V*pTvy.H2/NatConst::V_H2();
        let H2O: AmountOfSubstance = V*pTvy.H2O/NatConst::V_H2O();
        let O2: AmountOfSubstance = V*pTvy.O2/NatConst::V_O2();
        let N2: AmountOfSubstance = V*pTvy.N2/NatConst::V_N2();
        Self{p:pTvy.p, T:pTvy.T, H2, H2O, O2, N2, dt}
    }
    pub fn to_mass_rate(&self) -> pTmx {
        let Mx = self.M_ges();
        pTmx{p: self.p, T: self.T, m: Mx.M/self.dt,
            H2: Mx.H2, H2O: Mx.H2O, O2: Mx.O2, N2: Mx.N2
        }
    }

    pub fn to_volume_rate(&self) -> pTvy {
        let Vy = self.V_ges();
        pTvy{p: self.p, T: self.T, v: Vy.V/self.dt,
            H2: Vy.H2, H2O: Vy.H2O, O2: Vy.O2, N2: Vy.N2
        }
    }
    pub fn roh(&self)-> MassDensity{
        let M= self.M_ges().M;
        let V = self.V_ges().V;
        M/V
    }
    pub fn cp(&self) -> HeatCapacity {
        let ncp_l_H2: HeatCapacity = self.H2 * self.cp_mole(self.T, FitConst::fit_nist_H2());
        let ncp_l_H2O: HeatCapacity = self.H2O * self.cp_mole(self.T, FitConst::fit_nist_H2O());
        let ncp_l_O2: HeatCapacity = self.O2 * self.cp_mole(self.T, FitConst::fit_nist_O2());
        let ncp_l_N2: HeatCapacity = self.N2 * self.cp_mole(self.T, FitConst::fit_nist_N2());
        ncp_l_H2 + ncp_l_H2O + ncp_l_O2 + ncp_l_N2
    }

    pub fn cv(&self) -> HeatCapacity {
        let ncv_l_H2: HeatCapacity = self.H2 * (self.cp_mole(self.T, FitConst::fit_nist_H2()) - NatConst::R());
        let ncv_l_H2O: HeatCapacity = self.H2O * (self.cp_mole(self.T, FitConst::fit_nist_H2O()) - NatConst::R());
        let ncv_l_O2: HeatCapacity = self.O2 * (self.cp_mole(self.T, FitConst::fit_nist_O2()) - NatConst::R());
        let ncv_l_N2: HeatCapacity = self.N2 * (self.cp_mole(self.T, FitConst::fit_nist_N2()) - NatConst::R());
        ncv_l_H2 + ncv_l_H2O + ncv_l_O2 + ncv_l_N2
    }

    pub fn kappa(&self) -> Ratio {
        self.cp()/self.cv()
    }

    pub fn cp_spec(&self) -> SpecificHeatCapacity {
        let M= self.M_ges().M;
        let cp = self.cp();
        cp/M
    }
    pub fn cv_spec(&self) -> SpecificHeatCapacity {
        let M= self.M_ges().M;
        let cv = self.cv();
        cv/M
    }

    pub fn rH(&self) -> Ratio {
        let n = self.n_ges();
        let p_H2O: Pressure = self.H2O /n * self.p;
        p_H2O/self.p_sat()
    }

    pub fn TP(&self) -> ThermodynamicTemperature {
        let Tc = self.T.get::<degree_celsius>();
        let a = 7.5;
        let b = 237.3;
        let SDD = 6.1078 * 10f64.powf((a * Tc) / (b + Tc));
        let DD = self.rH().value * SDD;
        let v = (DD / 6.1078).log(10f64);
        let TP = b * v / (a - v);
        ThermodynamicTemperature::new::<degree_celsius>(TP)
    }

    /// Lennard-Jones-Potentials:
    /// https://www.accessengineeringlibrary.com/content/book/9780070116825/back-matter/appendix2?
    pub fn vis_dyn(&self)-> DynamicViscosity{
        let n = self.n_ges();
        let y =[self.H2/n, self.H2O/n, self.O2/n, self.N2/n];
        let M = [
            NatConst::M_H2(), NatConst::M_H2O(), 
            NatConst::M_O2(), NatConst::M_N2()
        ];
        // let mu_part = [
        //     self.vis_dyn_part(self.H2, NatConst::M_H2(), self.cnat.sigma_H2),
        //     self.vis_dyn_part(self.H2O, NatConst::M_H2()O, self.cnat.sigma_H2O),
        //     self.vis_dyn_part(self.O2, NatConst::M_O2(), self.cnat.sigma_O2),
        //     self.vis_dyn_part(self.N2, NatConst::M_N2(), self.cnat.sigma_N2)
        // ];
        // let mu_part = [
        //     self.vis_dyn_fit(self.cnat.fit_mu_H2),
        //     self.vis_dyn_fit(self.cnat.fit_mu_H2O),
        //     self.vis_dyn_fit(self.cnat.fit_mu_O2),
        //     self.vis_dyn_fit(self.cnat.fit_mu_N2)
        // ];
        let mu_part = [
            self.vis_dyn_suth(FitConst::mu_25_H2(), FitConst::fit_nist_H2()),
            self.vis_dyn_suth(FitConst::mu_25_H2O(), FitConst::fit_nist_H2O()),
            self.vis_dyn_suth(FitConst::mu_25_O2(), FitConst::fit_nist_O2()),
            self.vis_dyn_suth(FitConst::mu_25_N2(), FitConst::fit_nist_N2())
        ];

        let mut mu= DynamicViscosity::new::<pascal_second>(0.);
        for i in 0..4{
            let mut mu_j: f64 = 0.0;
            for j in 0..4{
                if j != i{
                    let phi = self.wilke_phi(mu_part[i], mu_part[j], M[i], M[j]);
                    mu_j += y[j].value * phi
                }else {mu_j += y[j].value;}
            }
            mu += (y[i]*mu_part[i])/mu_j;
        }
        mu

    }

    pub fn vis_kin(&self) -> KinematicViscosity{
        let mu = self.vis_dyn().get::<pascal_second>();
        let roh = self.roh().get::<kilogram_per_cubic_meter>();
        KinematicViscosity::new::<square_meter_per_second>(mu /roh)
    }

    pub fn pH2(&self) -> Pressure{
        self.H2/self.n_ges()*self.p
    }
    pub fn pH2O(&self) -> Pressure{
        self.H2O/self.n_ges()*self.p
    }
    pub fn pO2(&self) -> Pressure{
        self.O2/self.n_ges()*self.p
    }
    pub fn pN2(&self) -> Pressure{
        self.H2/self.n_ges()*self.p
    }

    /// d: Effective molekule diameter based on Leonard-Jones Potentials
    fn vis_dyn_part(&self, n: AmountOfSubstance, M: MolarMass, d: Length) -> DynamicViscosity {
        let m: Mass = n * M;
        let V: Volume = n * NatConst::R() * self.T / self.p;
        let roh: MassDensity = if V.value>0. {m/V}
        else{MassDensity::new::<kilogram_per_cubic_meter>(0.)};
        let v: Velocity = ((8.*NatConst::R()*self.T)/(PI*M)).sqrt(); //Mittlere Molekülgeschwindigkeit aus Maxwell-Bolzmann-Verteilung
        let l: Length = (NatConst::kB()*self.T)/(2f64.sqrt() * PI * d*d * self.p); //Mittlere freie Weglänge für ideale Gase
        1./3. * roh * v * l
    }

    /// Fitting Polynom based on Canteras
    /// https://cantera.org/stable/cxx/d8/d58/classCantera_1_1GasTransport.html#a267d20cdea7486fe84e722ee82d84b20
    /// https://cantera.org/stable/python/transport.html#cantera.Transport.get_viscosity_polynomial
    fn vis_dyn_fit(&self, fit: [f64; 5]) -> DynamicViscosity {
        let T = self.T.value;
        let mut sum =0.;
        fit.iter().enumerate().for_each(|(n, a)|{sum += a * T.ln().powi(n as i32);});
        sum *= T.powf(0.25);
        let mu = sum*sum;
        DynamicViscosity::new::<pascal_second>(mu)
    }

    /// Sutherlands formula
    fn vis_dyn_suth(&self, mu0: DynamicViscosity, nist_fit: [f64; 8]) -> DynamicViscosity {
        let S0 = self.S0_part(nist_fit, NatConst::T_25()).value;
        mu0 * (self.T / NatConst::T_25()).value.powf(3./2.)
            * (NatConst::T_25().value + S0)/(self.T.value + S0)
    }

    /// Wilke-Wechelwirkungsfaktor
    fn wilke_phi(&self, mu_a: DynamicViscosity, mu_b: DynamicViscosity, M_a: MolarMass, M_b: MolarMass)-> f64{
        if mu_b.is_normal() {
            1./8f64.sqrt()*(1.+(mu_a/mu_b).value.sqrt() * (M_b/M_a).value.powf(0.25)).powi(2)/(1.+ (M_a/M_b).value).sqrt()
        }else { 0. }
    }

    fn S0_part(&self, fit: [f64; 8], T: ThermodynamicTemperature) -> MolarHeatCapacity{
        let T = T.get::<kelvin>()/1000.;
        let S0 = fit[0]*T.ln() + fit[1]*T
            + fit[2]*T.powi(2)/2.
            + fit[3]*T.powi(3)/3.
            - fit[4]/(2.*T.powi(2))
            + fit[6];
        MolarHeatCapacity::new::<joule_per_kelvin_mole>(S0)
    }

    /// Shomate Equation with fit data from
    /// https://webbook.nist.gov/chemistry/
    fn cp_mole(&self, T: ThermodynamicTemperature, f: [f64; 8]) -> MolarHeatCapacity {
        let T = T.get::<kelvin>()/1000.;
        let cp = f[0] + f[1]*T + f[2]*T.powi(2) + f[3]*T.powi(3) + f[4]/T.powi(2);
        MolarHeatCapacity::new::<joule_per_kelvin_mole>(cp)
    }
    fn n_ges(&self) -> AmountOfSubstance{
        self.H2 + self.H2O + self.O2 + self.N2
    }
    fn M_ges(&self) -> MassFraction{
        let xH2: Mass = self.H2*NatConst::M_H2();
        let xH2O: Mass = self.H2O*NatConst::M_H2O();
        let xO2: Mass = self.O2*NatConst::M_O2();
        let xN2: Mass = self.N2*NatConst::M_N2();
        let M: Mass = xH2 + xH2O + xO2 + xN2;
        MassFraction{M, H2: xH2/M, H2O: xH2O/M, O2: xO2/M, N2:xN2/M}
    }
    fn V_ges(&self) -> VolumeFraction {
        let yH2: Volume = self.H2*NatConst::V_H2();
        let yH2O: Volume = self.H2O*NatConst::V_H2O();
        let yO2: Volume = self.O2*NatConst::V_O2();
        let yN2: Volume = self.N2*NatConst::V_N2();
        let V: Volume = yH2 + yH2O + yO2 + yN2;

        // todo check Volume calc
        // let n = self.n_ges();
        // let V = n * NatConst::R() * self.T / self.p;
        VolumeFraction{V, H2:yH2/V, H2O:yH2O/V, O2:yO2/V, N2:yN2/V}
    }

    /// Parameter based on https://doi.org/10.1115/1.3687121
    /// Valid vor T [344; 373]K
    fn p_sat(&self) -> Pressure{
        let A = 5.08354;
        let B = 1663.125;
        let C = -45.622;
        let T = self.T.get::<kelvin>();

        let p= 10f64.powf(A- (B/(T+C)));
        Pressure::new::<bar>(p)
    }


}

impl Add for GasState{
    type Output = GasState;
    fn add(self, rhs: Self) -> Self::Output {
        let Vl = self.V_ges();
        let Vr = rhs.V_ges();
        let pl: Pressure = self.p * Vl.V/(Vl.V + Vr.V);
        let pr: Pressure = self.p * Vr.V/(Vl.V + Vr.V);
        let p: Pressure = pl+pr;

        let cp_l = self.cp().value;
        let cp_r = rhs.cp().value;

        let Tl: ThermodynamicTemperature = (cp_l) / (cp_l + cp_r) * self.T;
        let Tr: ThermodynamicTemperature = (cp_r) / (cp_l + cp_r) * rhs.T;
        let T= Tl.value + Tr.value;
        
        let nH2l = self.H2 / self.dt;
        let nH2Ol = self.H2O / self.dt;
        let nO2l = self.O2 / self.dt;
        let nN2l = self.N2 / self.dt;

        let nH2r = rhs.H2 / rhs.dt;
        let nH2Or = rhs.H2O / rhs.dt;
        let nO2r = rhs.O2 / rhs.dt;
        let nN2r = rhs.N2 / rhs.dt;
        
        let (dt, H2, H2O, O2, N2)
            = if self.dt.value > rhs.dt.value {
            (
                self.dt, 
                (nH2l + nH2r)*self.dt,
                (nH2Ol + nH2Or)*self.dt,
                (nO2l + nO2r)*self.dt,
                (nN2l + nN2r)*self.dt,
            )
        }else {
            (
                rhs.dt,
                (nH2l + nH2r)*rhs.dt,
                (nH2Ol + nH2Or)*rhs.dt,
                (nO2l + nO2r)*rhs.dt,
                (nN2l + nN2r)*rhs.dt,
            )
        };
        
        Self{
            p,
            T: ThermodynamicTemperature::new::<kelvin>(T),
            dt,
            H2,
            H2O,
            O2,
            N2,
        }
    }
}

impl Sub for GasState{
    /// Hightly experimental!!!!!!!
    type Output = GasState;
    fn sub(self, rhs: Self) -> Self::Output {
        let Vl = self.V_ges();
        let Vr = rhs.V_ges();
        let pl: Pressure = self.p * Vl.V/(Vl.V + Vr.V);
        let pr: Pressure = self.p * Vr.V/(Vl.V + Vr.V);
        let p: Pressure = pl-pr;
        // 
        // let cp_l = self.cp().value;
        // let cp_r = rhs.cp().value;
        // 
        // let Tl: ThermodynamicTemperature = (cp_l) / (cp_l + cp_r) * self.T;
        // let Tr: ThermodynamicTemperature = (cp_r) / (cp_l + cp_r) * rhs.T;
        // let T= Tl.value - Tr.value;

        let nH2l = self.H2 / self.dt;
        let nH2Ol = self.H2O / self.dt;
        let nO2l = self.O2 / self.dt;
        let nN2l = self.N2 / self.dt;

        let nH2r = rhs.H2 / rhs.dt;
        let nH2Or = rhs.H2O / rhs.dt;
        let nO2r = rhs.O2 / rhs.dt;
        let nN2r = rhs.N2 / rhs.dt;

        let (dt, H2, H2O, O2, N2)
            = if self.dt.value > rhs.dt.value {
            (
                self.dt,
                (nH2l - nH2r)*self.dt,
                (nH2Ol - nH2Or)*self.dt,
                (nO2l - nO2r)*self.dt,
                (nN2l - nN2r)*self.dt,
            )
        }else {
            (
                rhs.dt,
                (nH2l - nH2r)*rhs.dt,
                (nH2Ol - nH2Or)*rhs.dt,
                (nO2l - nO2r)*rhs.dt,
                (nN2l - nN2r)*rhs.dt,
            )
        };

        Self{
            p,
            T: self.T,
            dt,
            H2,
            H2O,
            O2,
            N2,
        }
    }
}

impl Default for GasState{
    fn default() -> Self {
        Self{
            p: Pressure::new::<bar>(1.),
            T: ThermodynamicTemperature::new::<kelvin>(298.15),
            dt: Default::default(),
            H2: Default::default(),
            H2O: Default::default(),
            O2: Default::default(),
            N2: Default::default(),
        }
    }
}

struct MassFraction {M:Mass, H2:Ratio, H2O:Ratio, O2:Ratio, N2:Ratio}
struct VolumeFraction {V:Volume, H2:Ratio, H2O:Ratio, O2:Ratio, N2:Ratio}

