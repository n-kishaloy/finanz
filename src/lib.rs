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
    Cash                              = 1     ,
    Current_receivables               = 2     ,
    Current_loans                     = 3     ,
    Current_advances                  = 4     ,
    Other_current_assets              = 5     ,
    Current_investments_bv            = 6     ,
    Current_investments_mv            = 7     ,
    Raw_materials                     = 8     ,
    Work_in_progress                  = 9     ,
    Finished_goods                    = 10    ,
    Account_receivables               = 11    ,
    Long_term_loans                   = 12    ,
    Long_term_advances                = 13    ,
    Long_term_investments_bv          = 14    ,
    Long_term_investments_mv          = 15    ,
    Other_long_term_assets            = 16    ,
    Plant_property_equipment          = 17    ,
    Accumulated_depreciation          = 18    ,
    Leasing_rental_assset             = 19    ,
    Capital_wip                       = 20    ,
    Other_tangible_assets             = 21    ,
    Intangible_assets                 = 22    ,
    Intangible_assets_development     = 23    ,
    Accumulated_amortization          = 24    ,
    Current_payables                  = 25    ,
    Current_borrowings                = 26    ,
    Current_notes_payable             = 27    ,
    Other_current_liabilities         = 94    ,
    Interest_payable                  = 95    ,
    Current_provisions                = 28    ,
    Current_tax_payables              = 29    ,
    Liabilities_sale_assets           = 30    ,
    Current_leases_liability          = 31    ,
    Account_payables                  = 32    ,
    Long_term_borrowings              = 33    ,
    Bonds_payable                     = 96    ,
    Deferred_tax_liabilities          = 34    ,
    Long_term_leases_liability        = 35    ,
    Deferred_compensation             = 97    ,
    Deferred_revenues                 = 98    ,
    Customer_deposits                 = 99    ,
    Other_long_term_liabilities       = 36    ,
    Pension_provision                 = 37    ,
    Long_term_provisions              = 38    ,
    Common_stock                      = 39    ,
    Preferred_stock                   = 40    ,
    Pd_in_cap_above_par               = 41    ,
    Pd_in_cap_treasury_stock          = 42    ,
    Revaluation_reserves              = 43    ,
    Reserves                          = 100   ,
    Retained_earnings                 = 101   ,
    Accumulated_oci                   = 102   ,
    Minority_interests                = 103   ,
    Operating_revenue                 = 44    ,
    Non_operating_revenue             = 104   ,
    Excise_stax_levy                  = 45    ,
    Other_income                      = 46    ,
    Cost_material                     = 47    ,
    Direct_expenses                   = 48    ,
    Salaries                          = 49    ,
    Administrative_expenses           = 50    ,
    Research_n_development            = 51    ,
    Other_overheads                   = 52    ,
    Other_operative_expenses          = 53    ,
    Other_expenses                    = 54    ,
    Exceptional_items                 = 107   ,
    Pbitdax                           = 109   ,
    Depreciation                      = 55    ,
    Amortization                      = 56    ,
    Pbitx                             = 57    ,
    Interest                          = 58    ,
    Pbtx                              = 59    ,
    Extraordinary_items               = 60    ,
    Prior_years                       = 61    ,
    Pbt                               = 62    ,
    Taxes_current                     = 63    ,
    Taxes_deferred                    = 64    ,
    Pat                               = 65    ,
    Gains_losses_forex                = 66    ,
    Gains_losses_acturial             = 67    ,
    Gains_losses_sales                = 105   ,
    Fv_chg_avl_4_sale                 = 68    ,
    Other_deferred_taxes              = 69    ,
    Other_comprehensive_income        = 70    ,
    Total_comprehensive_income        = 71    ,
    Chg_inventories                   = 72    ,
    Chg_receivables                   = 73    ,
    Chg_liabilities                   = 74    ,
    Chg_provisions                    = 75    ,
    Cash_flow_operations              = 76    ,
    Investments_ppe                   = 77    ,
    Investments_cap_devp              = 78    ,
    Acq_equity_assets                 = 79    ,
    Dis_equity_assets                 = 80    ,
    Dis_ppe                           = 81    ,
    Chg_investments                   = 82    ,
    Other_cf_investments              = 106   ,
    Cash_flow_investments             = 83    ,
    Stock_sales                       = 84    ,
    Stock_repurchase                  = 85    ,
    Debt_issue                        = 86    ,
    Debt_repay                        = 87    ,
    Dividends                         = 88    ,
    Donor_contribution                = 89    ,
    Cash_flow_financing               = 90    ,
    Fcff                              = 91    ,
    Fcfe                              = 92    ,
    Fcfd                              = 93    ,
    Shares_outstanding                = 108   , 
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