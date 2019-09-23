extern crate ndarray;

use std::error::Error;
use util;
use ndarray::{Array1, ArrayView1, array};


fn main()->Result<(), Box<dyn Error>> {

    use finanz::tval;

    let r = 0.08;
    let _y = match tval::irr(array![2.0, 0.5, 0.75, 1.35].view()){
        Ok(x) => x, _ => -999.0
    }*100.0;
    let _x = tval::irr(array![-2.0, 0.5, 0.75, 1.35].view()).unwrap();
    let z = tval::irr(array![-2.0, 0.5, 0.75, 1.35].view())?; // requires Error
    println!("Hello, world! {:.20}, {:.20}, {:.5}", 
        z, tval::t_bill_r(150.0, 98_000., 100_000.)
        ,f64::ln(10.0)) ;

    println!("npv_n{}", tval::irr(array![-2.0, 0.5, 0.75, 1.35].view())?);

    let xytr:Vec<(&i32, i32)> = vec![1_i32,2,3,4].iter().zip(0_i32..).collect(); 

    println!("g_search => {:?}", util::optim::g_search(|x| (x-3.0).powf(2.0), 0.0, 5.0, 1.0e-9)?);

    Ok(())
}