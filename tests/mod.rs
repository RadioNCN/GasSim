use GasSim::Helper::norm::norm_FN;

#[test]
fn test_norm_FN_uom(){
    use GasSim::Helper::norm;
    use uom::si::f64::{Ratio, MassRate, Time};
    use uom::si::ratio::ratio;
    use uom::si::time::second;
    use uom::si::mass_rate::gram_per_second;

    let x = MassRate::new::<gram_per_second>(50.);
    let dx = (MassRate::new::<gram_per_second>(0.), MassRate::new::<gram_per_second>(100.));
    let dy = (Ratio::new::<ratio>(0.), Ratio::new::<ratio>(1.));

    let y = norm_FN(&x, &dx.0, &dx.1, &dy.0, &dy.1);
    println!("{:?}", y);
    assert!(y.value-0.5 <1e-12)
}

#[test]
fn test_norm_FN_f64(){
    use GasSim::Helper::norm;
    use uom::si::f64::{Ratio, MassRate, Time};
    use uom::si::ratio::ratio;
    use uom::si::time::second;
    use uom::si::mass_rate::gram_per_second;

    let x = 50.;
    let dx = (0., 100.);
    let dy = (0., 1.);

    let y = norm_FN(&x, &dx.0, &dx.1, &dy.0, &dy.1);
    println!("{:?}", y);
    assert!(y-0.5 <1e-12)
}

#[test]
fn test_norm_FN_mix(){
    use GasSim::Helper::norm;
    use uom::si::f64::{Ratio, MassRate, Time};
    use uom::si::ratio::ratio;
    use uom::si::time::second;
    use uom::si::mass_rate::gram_per_second;

    let x = 50.;
    let dx = (0., 100.);
    let dy = (Ratio::new::<ratio>(0.), Ratio::new::<ratio>(1.));

    let y = norm_FN(&x, &dx.0, &dx.1, &dy.0, &dy.1);
    println!("{:?}", y);
    assert!(y.value-0.5 <1e-12)
}