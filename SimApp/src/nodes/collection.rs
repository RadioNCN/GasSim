#[derive(Clone, Debug)]
pub enum Node {
    Number(f64),
    Add { sum: f64 }, // two numeric inputs, one numeric output (cached sum)
    Output,           // one numeric input, no outputs
}