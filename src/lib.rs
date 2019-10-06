extern crate ndarray;
extern crate util;
extern crate chrono;

use util::{approx};
use ndarray::{Array1, ArrayView1,array};
use chrono::prelude::{DateTime, Utc};
use std::collections::HashMap;


pub mod base;
pub mod tval;

pub trait Dbz {
    fn write_db(&self);
    fn read_db(&mut self);
    fn set_mapz(&mut self) -> ();
}

pub trait Internetz {
    fn set_price(&mut self);
    fn set_docz(&mut self);
}

#[repr(u8)] #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Typez {
    cash                              =   1     ,
    current_receivables               =   4     ,
    current_loans                     =   3     ,
    others                            =   2     ,


}

pub struct Mapz{
    
    stat_list: [String; 6], stat_back: HashMap<String, u8>, 
    class_list: [String; 30], class_back: HashMap<String, u8>,
    types_list: [String; 120], types_back: HashMap<String, u8>,

    class_stat: [u8; 36], stat_class: [Vec<u8>; 6],
    type_class: [u8; 120], class_type: [Vec<u8>; 30],

}

impl Mapz {

    // pub fn new(&self)-> Self {
    //     Self {
    //         // stat_list : ["".to_string();   6],
    //         // class_list: ["".to_string();  30],
    //         // types_list: ["".to_string(); 120],

    //         stat_back : HashMap::new(),
    //         class_back: HashMap::new(),
    //         types_back: HashMap::new(),

    //         class_stat: [0;  36], stat_class: [vec![];  6],
    //         type_class:  [0; 120], class_type: [vec![]; 30],

    //     }
    // }

    pub fn read_json(&mut self, fil: String)-> () {}

}

pub struct Document{

    time_beg: DateTime<Utc>,
    time_end: DateTime<Utc>,

    bE:f64,
    bD:f64,
    rf:f64,
    rm:f64,

    records : HashMap<String, f64>  



}

impl Document {

    pub fn export_json(&self) -> Result<String, &'static str>{
        Err("Not implemented yet")
    }

    pub fn get(&self, typ:&String) -> Result<f64, &'static str>{
        Err("Not implemented yet")
    }

    pub fn set(&self, mz:&Mapz, typ:&String) -> Result<f64, &'static str>{
        Err("Not implemented yet")
    }

    pub fn check_balance_sheet(&self) -> bool { false }

    pub fn check_cf(&self) -> bool { false }

    pub fn get_debt(&self) -> f64 { 0.0 }

    pub fn get_equity(&self) -> f64 { 0.0 }

    
} 


pub struct Company {

    name: String,
    code: String,

    docz: Vec<Document>,

    share_price: f64,
    prev_clos_price: f64,
    shares_volume: i64,
    shares_outstanding: i64,

    timez: DateTime<Utc>,

    consolidated: bool,

    yahoo_code: String,
    mc_code: String,

}

impl Company {

    pub fn check_cash_flow(&self) -> bool { false }

    pub fn generate_cash_flow(&self) -> () { }

    pub fn dcf(&self) -> ()  {  }

    pub fn check_timez(&self) -> bool { false }




}