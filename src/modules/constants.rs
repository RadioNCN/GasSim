#![allow(non_snake_case)]

use uom::si::dynamic_viscosity::pascal_second;
use uom::si::electric_charge::coulomb;
use uom::si::f64::*;
use uom::si::heat_capacity::joule_per_kelvin;
use uom::si::length::angstrom;
use uom::si::molar_heat_capacity::joule_per_kelvin_mole;
use uom::si::molar_mass::gram_per_mole;
use uom::si::molar_volume::cubic_decimeter_per_mole;
use uom::si::thermodynamic_temperature::degree_celsius;

#[derive(Clone, Debug)]
pub struct NatConst {}
/// Physical Constants without Units
impl NatConst {
    const FARADAY: f64 = 96485.33289;
    const AVOGADRO: f64 = 6.022140857e23;
    const BOLTZMANN: f64 = 1.38064852e-23;
}
/// Physical Constants with Units
impl NatConst {
    pub fn R() -> MolarHeatCapacity {
        MolarHeatCapacity::new::<joule_per_kelvin_mole>(8.314f64)
    }
    pub fn kB() -> HeatCapacity {
        HeatCapacity::new::<joule_per_kelvin>(Self::BOLTZMANN)
    }
    pub fn e() -> ElectricCharge {
        ElectricCharge::new::<coulomb>(1.602176634e-19)
    }
}
/// Molar Mass
impl NatConst {
    pub fn M_H2() -> MolarMass {
        MolarMass::new::<gram_per_mole>(1.00784 * 2.)
    }
    pub fn M_H2O() -> MolarMass {
        MolarMass::new::<gram_per_mole>(18.01528)
    }
    pub fn M_O2() -> MolarMass {
        MolarMass::new::<gram_per_mole>(31.9988)
    }
    pub fn M_N2() -> MolarMass {
        MolarMass::new::<gram_per_mole>(28.0134)
    }
}
/// Molar Volume
impl NatConst {
    pub fn V_H2() -> MolarVolume {
        MolarVolume::new::<cubic_decimeter_per_mole>(22.4)
    }
    pub fn V_H2O() -> MolarVolume {
        MolarVolume::new::<cubic_decimeter_per_mole>(22.4)
    }
    pub fn V_O2() -> MolarVolume {
        MolarVolume::new::<cubic_decimeter_per_mole>(22.4)
    }
    pub fn V_N2() -> MolarVolume {
        MolarVolume::new::<cubic_decimeter_per_mole>(22.4)
    }
}
/// Normal Conditions
impl NatConst {
    pub fn T_25() -> ThermodynamicTemperature {
        ThermodynamicTemperature::new::<degree_celsius>(25.)
    }
}
#[derive(Clone, Debug)]
pub struct FitConst {}
/// 25C Viscosity Constants
impl FitConst {
    pub fn mu_25_H2() -> DynamicViscosity {
        DynamicViscosity::new::<pascal_second>(8.964e-6)
    }
    pub fn mu_25_H2O() -> DynamicViscosity {
        DynamicViscosity::new::<pascal_second>(1.026e-5)
    }
    pub fn mu_25_O2() -> DynamicViscosity {
        DynamicViscosity::new::<pascal_second>(2.056e-5)
    }
    pub fn mu_25_N2() -> DynamicViscosity {
        DynamicViscosity::new::<pascal_second>(1.800e-5)
    }
}
/// NIST Fit Constants
impl FitConst {
    pub const fn fit_nist_H2() -> [f64; 8] {
        [
            33.066178, -11.363417, 11.432816, -2.772874, -0.158558, -9.980797, 172.707974, 0.0,
        ]
    }
    pub const fn fit_nist_O2() -> [f64; 8] {
        [
            31.32234, -20.23531, 57.86644, -36.50624, -0.007374, -8.903471, 246.7945, 0.0,
        ]
    }
    pub const fn fit_nist_N2() -> [f64; 8] {
        [
            28.98641, 1.853978, -9.647459, 16.63537, 0.000117, -8.671914, 226.4168, 0.0,
        ]
    }
    pub const fn fit_nist_H2O() -> [f64; 8] {
        [
            30.09200, 6.832514, 6.793435, -2.534480, 0.082139, -250.8810, 223.3967, -241.8264,
        ]
    }
}
/// Effective Molecule Diameters
impl FitConst {
    pub fn sigma_H2() -> Length {
        Length::new::<angstrom>(2.93 / 1.3158665329244235)
    } // 1/1.3158665329244235
    pub fn sigma_H2O() -> Length {
        Length::new::<angstrom>(2.641 / 0.6971553487521269)
    } // 1/0.6971553487521269
    pub fn sigma_O2() -> Length {
        Length::new::<angstrom>(3.467 / 1.122136879560862)
    } // 1//1.122136879560862
    pub fn sigma_N2() -> Length {
        Length::new::<angstrom>(3.7 / 1.1585688927269846)
    } // 1/1.1585688927269846
}
