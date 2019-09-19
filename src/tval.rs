
extern crate util;
extern crate ndarray;

use util::{approx};
use ndarray::{Array1, ArrayView1,array};

pub fn t_mul(r:f64, n:f64)->f64 { (1.0 + r).powf(n) }
pub fn t_mul_m(r:f64, n:f64, m:f64)->f64 { (1.0 + r/m).powf(n*m) }
pub fn rate(f:f64, p:f64, n:f64)->f64 { (f/p).powf(1.0/n) - 1.0 }
pub fn period(f:f64, p:f64, r:f64)-> f64 { (f/p).ln()/(1.0+r).ln() }  

pub fn pv(fv:f64,r:f64,n:f64)->f64 { fv/(1.0+r).powf(n) }
pub fn pv_m(fv:f64,r:f64,n:f64,m:f64)->f64 { fv/(1.0+r/m).powf(n*m) }
pub fn pv_c(fv:f64,r:f64,n:f64)->f64 { fv/f64::exp(r*n) }

pub fn fv(fv:f64,r:f64,n:f64)->f64 { fv*(1.0+r).powf(n) }
pub fn fv_m(fv:f64,r:f64,n:f64,m:f64)->f64 { fv*(1.0+r/m).powf(n*m) }
pub fn fv_c(fv:f64,r:f64,n:f64)->f64 { fv*f64::exp(r*n) }

pub fn pv_annuity(pmt:f64,r:f64,n:f64)->f64 { 
    pmt/r*(1.0 - 1.0/(1.0+r).powf(n))
}

pub fn pv_annuity_m(pmt:f64,r:f64,n:f64, m:f64)->f64 {
    pmt/(r/m)*(1.0 - 1.0/(1.0 + r/m).powf(n*m))
}

pub fn pv_annuity_inf(pmt:f64, r:f64)->f64 { pmt/r }

pub fn fv_annuity(pmt:f64,r:f64,n:f64)->f64 { 
    pmt/r * ((1. + r).powf(n) - 1.) 
}
pub fn fv_annuity_m(pmt:f64,r:f64,n:f64,m:f64)->f64 { 
    pmt/(r/m) * ((1. + r/m).powf(n*m) - 1.) 
}

pub fn pmt(pv:f64,r:f64,n:f64)->f64 { pv*r/(1.-1./(1.+r).powf(n)) }  
pub fn pmt_m(pv:f64,r:f64,n:f64,m:f64)->f64 { pv*r/m/(1.-1./(1.+r/m).powf(n*m)) }  

pub fn fmt(fv:f64,r:f64,n:f64)->f64 { fv*r/((1.+r).powf(n) - 1.) }
pub fn fmt_m(fv:f64,r:f64,n:f64,m:f64)->f64 { fv*r/m/((1.+r/m).powf(n*m) - 1.) }

pub fn eff_r(r:f64,m:f64)->f64 { (1. + r/m).powf(m) - 1. }
pub fn eff_r_cont(r:f64)->f64 { f64::exp(r) - 1. }

pub fn npv(mut r:f64,tim:ArrayView1<f64>,cf:ArrayView1<f64>,t0:f64)->f64 {
    let mut sm = 0.0; r += 1.0;
    for i in 0..cf.len(){ sm += cf[i]/r.powf(tim[i]) }
    sm/r.powf(t0)
}

pub fn npv_n(mut r:f64,cf:ArrayView1<f64>,t0:f64)->f64 {
    let mut sm = 0.0; r += 1.0;
    for i in 0..cf.len(){ sm += cf[i]/r.powf(i as f64) }
    sm/r.powf(t0)
} 

pub fn npv_t0(mut r:f64,tim:ArrayView1<f64>,cf:ArrayView1<f64>)->f64 {
    let mut sm = 0.0; r += 1.0;
    for i in 0..cf.len() { sm += cf[i]/r.powf(tim[i]) }
    sm
}

pub fn npv_n0(mut r:f64,cf:ArrayView1<f64>)->f64 {
    let mut sm = 0.0; r += 1.0;
    for i in 0..cf.len(){ sm += cf[i]/r.powf(i as f64) }
    sm
} 

pub fn npv_r(mut r:f64)->impl FnMut(ArrayView1<f64>,ArrayView1<f64>,f64)->f64 {
    move |tim:ArrayView1<f64>,cf:ArrayView1<f64>,t0:f64| { 
        let mut sm = 0.0; r += 1.0;
        for i in 0..cf.len(){ sm += cf[i]/r.powf(tim[i]) }
        sm/r.powf(t0)
    }
}

pub fn npv_r0(mut r:f64)->impl FnMut(ArrayView1<f64>,ArrayView1<f64>)->f64 {
    move |tim:ArrayView1<f64>,cf:ArrayView1<f64>| { 
        let mut sm = 0.0; r += 1.0;
        for i in 0..cf.len(){ sm += cf[i]/r.powf(tim[i]) }
        sm
    }
}

pub fn npv_rt(mut r:f64,t0:f64)->impl FnMut(ArrayView1<f64>,ArrayView1<f64>)->f64 {
    move |tim:ArrayView1<f64>,cf:ArrayView1<f64>| { 
        let mut sm = 0.0; r += 1.0;
        for i in 0..cf.len(){ sm += cf[i]/r.powf(tim[i]) }
        sm/r.powf(t0)
    }
}

pub fn irr(cf:ArrayView1<f64>)->Result<f64, &'static str> { 
    util::roots::root_nwt(move |x| npv_n0(x,cf), 0.0, 1e-6) 
}

pub fn twrr(bv:ArrayView1<f64>, b_inf:ArrayView1<f64>)->f64 {
    let n = bv.len()-1; let mut r = 1.0;
    for i in 0..n { r *= bv[i+1]/(bv[i]+b_inf[i]); }
    r.powf(1.0/(n as f64)) - 1.0
}

pub fn twrr_n(n:f64, bv:ArrayView1<f64>, b_inf:ArrayView1<f64>)->f64 {
    let mut r = 1.0;
    for i in 0..bv.len()-1 { r *= bv[i+1]/(bv[i]+b_inf[i]); }
    r.powf(1.0/(n as f64)) - 1.0
}

// T-bill
pub fn t_bill_r(t:f64, p0:f64, f:f64)->f64 { (1.0 - p0/f) * 360.0/t }
pub fn t_bill_d(r:f64,t:f64,f:f64)->f64 { r*t*f/360.0 }

pub fn hpy(p0:f64,p1:f64,d1:f64)->f64 { (p1+d1)/p0-1.0 }
pub fn eay(t:f64,p0:f64,p1:f64,d1:f64)->f64 { 
    ((p1+d1)/p0).powf(365.0/t) - 1.0
}
pub fn mmy(t:f64,p0:f64,p1:f64,d1:f64)->f64{ ((p1+d1)/p0-1.0)*360./t }

pub fn sharpe(rf:f64,rp:f64,sp:f64)->f64 { (rp - rf)/sp }
pub fn sharpe_rf(rf:f64)->impl Fn(f64,f64)->f64 { 
    move |rp:f64,sp:f64| { (rp - rf)/sp }
}




#[cfg(test)] mod tv {
    use super::*; 

    #[test] fn t_mul_test() { assert!(t_mul(0.06/12., -120.) == 0.5496327333641637); }

    #[test] fn t_mul_m1_test() { assert!(t_mul_m(0.06, -10., 12.) == 0.5496327333641637); }

    #[test] fn rate_test() { assert!(rate(7.35, 8.52, 5.0) == -0.029111071029244595) }

    #[test] fn period_test() { assert!(period(100.0,50.0,0.07) == 10.244768351058712) }

}


#[cfg(test)] mod pv {
    use super::*; 

    #[test] fn pv_test() { assert!(pv(10_000_000., 0.09, 5.)==6_499_313.862983453088) }

    #[test] fn pv_m_test() { assert!(approx(pv_m(12_704_891.6109538, 0.06, 4., 12.), 10_000_000., 1e-6)) }
    
    #[test] fn pv_c_test() { assert!(pv_c(10_000., 0.08, -2.) == 11_735.1087099181022495) }

}

#[cfg(test)] mod fv {
    use super::*; 

    #[test] fn fv_test(){assert!(fv(6_499_313.86298345309,0.09,5.)==1e+7)}
    #[test] fn fv_m_test(){assert!(fv_m(1e+7, 0.06, 4., 12.) == 12_704_891.6109538227319717)}
    #[test] fn fv_c_test(){assert!(fv_c(1e+4,0.08,2.) == 11_735.108709918102)}
}

#[cfg(test)] mod annuity {
    use super::*; 

    #[test] fn pv_annuity_test(){assert!(pv_annuity(1000., 0.12, 5.) == 3_604.776202345007)}
    #[test] fn pv_annuity_m_test(){assert!(pv_annuity_m(7.33764573879378, 0.08, 30., 12.) == 1000.)}

    #[test] fn pv_annuity_inf_test(){assert!(pv_annuity_inf(100., 0.05)==2000.)}

    #[test] fn fv_annuity_test(){assert!(fv_annuity(1000., 0.05, 5.) == 5_525.631250000007)}
    #[test] fn fv_annuity_m_test(){assert!(fv_annuity_m(2000., 0.24, 5., 3.) == 54_304.2278549568)}

    #[test] fn pmt_test(){assert!(pmt(3_604.776202345007,0.12,5.)==1000.)}
    #[test] fn pmt_m_test(){assert!(pmt_m(1000.,0.08,30.,12.) == 7.33764573879378)}

    #[test] fn fmt_test(){assert!(fmt(5_525.631250000007,0.05,5.)==1000.)}
    #[test] fn fmt_m_test(){assert!(fmt_m(54_304.2278549568, 0.24, 5., 3.)==2000.)}

    #[test] fn eff_r_test(){assert!(approx(eff_r(0.08, 2.),0.0816,1e-8))}
}

#[cfg(test)] mod npv {
    use super::*; 

    #[test] fn npv_test() {
        let tim = array![0.25,6.25,3.5,4.5,1.25]; 
        let cf = array![-6.25,1.2,1.25,3.6,2.5];
        assert!(npv(0.08,tim.view(),cf.view(),0.45) == 0.36962283798505946);
        assert!(npv_r0(0.08)(tim.view(),cf.view())==0.3826480347907877 );
    }

    #[test] fn npv_n_test() {assert!(npv_n(0.05,array![1000.,2000.,4000.,5000.,6000.].view(),1.45) == 14709.9233383357313869)}

    #[test] fn npv_t0_test() {assert!(npv_t0(0.08,array![0.25,6.25,3.5,4.5,1.25].view(),array![-6.25,1.2,1.25,3.6,2.5].view()) == 0.3826480347907877)}

    #[test] fn npv_n0_test() {assert!(approx(npv_n0(0.1,array![-2.,0.5,0.75,1.35].view()),0.08865514650638584726040,1e-9) )}

    #[test] fn npv_r_test() {assert!(npv_r(0.08)(array![0.25,6.25,3.5,4.5,1.25].view(),array![-6.25,1.2,1.25,3.6,2.5].view(), 0.45)==0.36962283798505946 )}

    #[test] fn npv_r0_test() {assert!(npv_r0(0.08)(array![0.25,6.25,3.5,4.5,1.25].view(),array![-6.25,1.2,1.25,3.6,2.5].view())==0.3826480347907877 )}

    #[test] fn npv_rt_test() {assert!(npv_rt(0.08, 0.45)(array![0.25,6.25,3.5,4.5,1.25].view(),array![-6.25,1.2,1.25,3.6,2.5].view())==0.36962283798505946 )}

    #[test] fn irr_test() {
        assert!(irr(array![-2.0, 0.5, 0.75, 1.35].view()) == 
            Ok(0.12129650313094050840) );
        
        assert!(irr(array![2.0, 0.5, 0.75, 1.35].view()) == 
            Err("No soln") );
    }

    #[test] fn twrr_test() {
        assert!(twrr(array![4.,6.,5.775,6.72,5.508].view(),array![1.,-0.5,0.225,-0.6].view()) == 0.06159232319186159)
    }

    #[test] fn twrr_n_test() {
        assert!(twrr_n(1.0, array![100., 112., 142.64].view(),array![0.,20.].view()) == 0.21027878787878795)
    }
}

#[cfg(test)] mod t_bill {
    use super::*; 
    
    #[test] fn t_bill_r_test() {
        assert!(approx(t_bill_r(150.,98_000.,100_000.),0.048,1e-7));
    }
    #[test] fn t_bill_d_test() {
        assert!(t_bill_d(0.048,150.,100_000.) == 2_000.)
    }

    #[test] fn hpy_test() { assert!( hpy(98.,95.,5.) == 0.020408163265306145) }
    #[test] fn eay_test() { assert!( 
        eay(150.,98.,95.,5.) == 0.05038831660532006)
    }
    #[test] fn mmy_test() { assert!(
        mmy(150.,98.,95.,5.) == 0.04897959183673475)
    }

    #[test] fn sharpe_test() {
        assert!(sharpe(1.58, 9.26, 22.36) == 0.3434704830053667)
    }
    #[test] fn sharpe_rf_test() {
        assert!(sharpe_rf(1.58)(9.26, 22.36) == 0.3434704830053667)
    }

}