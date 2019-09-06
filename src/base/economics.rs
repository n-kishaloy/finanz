extern crate util;
use util::{approx};

pub mod core{

    pub fn elasticity(fx:fn(f64)->f64, fy:fn(f64)->f64, z:f64, ztol:f64)->f64{
        let fx0 = fx(z); let fy0 = fy(z);
        let z = z + ztol;
        fy(z)/fy0/fx(z)*fx0
    }


}  




#[cfg(test)] mod eco_test {
    use super::*;

    #[test] fn bas() {
        assert!(approx(core::elasticity(|x| x.powi(2), |x| 2.0*x, 1.0, 1e-9), 1.00, 1e-7))
    }

}
