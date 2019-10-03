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


pub struct Mapz{
    
    stat_list: [String; 6], stat_back: HashMap<String, u8>, 
    class_list: [String; 30], class_back: HashMap<String, u8>,
    types_list: [String; 120], types_back: HashMap<String, u8>,

    class_stat: [u8; 36], stat_class: [Vec<u8>; 6],
    type_class: [u8; 120], class_type: [Vec<u8>; 30],

}

impl Mapz {

    pub fn read_json(&mut self, fil: String)-> () {}

}

pub struct Document{

    time_beg: DateTime<Utc>,
    time_end: DateTime<Utc>,

    bE:f64,
    bD:f64,
    rf:f64,
    rm:f64,


}

impl Document {

    pub fn export_json(&self) -> Result<String, &'static str>{
        Err("No implemented yet")
    }

    pub fn build_cf(&self) -> () {  }


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