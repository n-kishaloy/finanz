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
    Cash                                = 1     ,
    CurrentReceivables                  = 2     ,
    CurrentLoans                        = 3     ,
    CurrentAdvances                     = 4     ,
    OtherCurrentAssets                  = 5     ,
    CurrentInvestmentsBv                = 6     ,
    CurrentInvestmentsMv                = 7     ,
    RawMaterials                        = 8     ,
    WorkInProgress                      = 9     ,
    FinishedGoods                       = 10    ,
    AccountReceivables                  = 11    ,
    LongTermLoans                       = 12    ,
    LongTermAdvances                    = 13    ,
    LongTermInvestmentsBv               = 14    ,
    LongTermInvestmentsMv               = 15    ,
    OtherLongTermAssets                 = 16    ,
    PlantPropertyEquipment              = 17    ,
    AccumulatedDepreciation             = 18    ,
    LeasingRentalAssset                 = 19    ,
    CapitalWip                          = 20    ,
    OtherTangibleAssets                 = 21    ,
    IntangibleAssets                    = 22    ,
    IntangibleAssetsDevelopment         = 23    ,
    AccumulatedAmortization             = 24    ,
    CurrentPayables                     = 25    ,
    CurrentBorrowings                   = 26    ,
    CurrentNotesPayable                 = 27    ,
    OtherCurrentLiabilities             = 94    ,
    InterestPayable                     = 95    ,
    CurrentProvisions                   = 28    ,
    CurrentTaxPayables                  = 29    ,
    LiabilitiesSaleAssets               = 30    ,
    CurrentLeasesLiability              = 31    ,
    AccountPayables                     = 32    ,
    LongTermBorrowings                  = 33    ,
    BondsPayable                        = 96    ,
    DeferredTaxLiabilities              = 34    ,
    LongTermLeasesLiability             = 35    ,
    DeferredCompensation                = 97    ,
    DeferredRevenues                    = 98    ,
    CustomerDeposits                    = 99    ,
    OtherLongTermLiabilities            = 36    ,
    PensionProvision                    = 37    ,
    LongTermProvisions                  = 38    ,
    CommonStock                         = 39    ,
    PreferredStock                      = 40    ,
    PdInCapAbovePar                     = 41    ,
    PdInCapTreasuryStock                = 42    ,
    RevaluationReserves                 = 43    ,
    Reserves                            = 100   ,
    RetainedEarnings                    = 101   ,
    AccumulatedOci                      = 102   ,
    MinorityInterests                   = 103   ,
    OperatingRevenue                    = 44    ,
    NonOperatingRevenue                 = 104   ,
    ExciseStaxLevy                      = 45    ,
    OtherIncome                         = 46    ,
    CostMaterial                        = 47    ,
    DirectExpenses                      = 48    ,
    Salaries                            = 49    ,
    AdministrativeExpenses              = 50    ,
    ResearchNDevelopment                = 51    ,
    OtherOverheads                      = 52    ,
    OtherOperativeExpenses              = 53    ,
    OtherExpenses                       = 54    ,
    ExceptionalItems                    = 107   ,
    Pbitdax                             = 109   ,
    Depreciation                        = 55    ,
    Amortization                        = 56    ,
    Pbitx                               = 57    ,
    Interest                            = 58    ,
    Pbtx                                = 59    ,
    ExtraordinaryItems                  = 60    ,
    PriorYears                          = 61    ,
    Pbt                                 = 62    ,
    TaxesCurrent                        = 63   ,
    TaxesDeferred                       = 64   ,
    Pat                                 = 65   ,
    GainsLossesForex                    = 66   ,
    GainsLossesActurial                 = 67   ,
    GainsLossesSales                    = 105   ,
    FvChgAvlSale                        = 68   ,
    OtherDeferredTaxes                  = 69   ,
    OtherComprehensiveIncome            = 70   ,
    TotalComprehensiveIncome            = 71   ,
    ChgInventories                      = 72   ,
    ChgReceivables                      = 73   ,
    ChgLiabilities                      = 74   ,
    ChgProvisions                       = 75   ,
    CashFlowOperations                  = 76   ,
    InvestmentsPpe                      = 77   ,
    InvestmentsCapDevp                  = 78   ,
    AcqEquityAssets                     = 79   ,
    DisEquityAssets                     = 80   ,
    DisPpe                              = 81   ,
    ChgInvestments                      = 82   ,
    OtherCfInvestments                  = 106   ,
    CashFlowInvestments                 = 83   ,
    StockSales                          = 84   ,
    StockRepurchase                     = 85   ,
    DebtIssue                           = 86   ,
    DebtRepay                           = 87   ,
    Dividends                           = 88   ,
    DonorContribution                   = 89   ,
    CashFlowFinancing                   = 90   ,
    Fcff                                = 91   ,
    Fcfe                                = 92   ,
    Fcfd                                = 93   ,
    SharesOutstanding                   = 108   

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