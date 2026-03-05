use std::ops::{Add, Div, Mul, Sub};
pub fn norm_FN<I, Y, O>(x: &I, min_i: &I, max_i: &I, min_o: &O, max_o: &O) -> O
where
    I: Copy + Add<Output = I> + Sub<Output = I> + Div<Output = Y> + Div<f64, Output = I>,
    O: Copy + Add<Output = O> + Sub<Output = O> + Div<f64, Output = O>,
    Y: Copy + Mul<O, Output = O>,
{
    let y = (*x - (*max_i + *min_i) / 2.) / (*max_i - *min_i);
    return y * (*max_o - *min_o) + (*max_o + *min_o) / 2.;
}
