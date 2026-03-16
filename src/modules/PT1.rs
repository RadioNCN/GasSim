#[derive(Debug, Clone)]
pub struct pt1 {
    pub t: f64,
    k: f64,
    int_val: f64,
}
impl pt1 {
    pub fn new(T: f64, K: f64) -> pt1 {
        pt1 {
            t: T,
            k: K,
            int_val: 0f64,
        }
    }
    pub fn call(&mut self, u: f64) -> f64 {
        let y = (1.0 / self.t) * self.int_val;
        let kuy = self.k * (u - y);
        self.int_val += kuy;
        return y;
    }
}