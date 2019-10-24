extern crate ndarray;
extern crate util;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use util::{approx};
use ndarray::{Array1, ArrayView1,array};
use chrono::prelude::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::fmt;
use serde_json::{Value, Map};


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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JEntry {
    Debit(f64), Credit(f64)
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

#[repr(u8)] #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Classez {
    ClCurrentAssets                     =   1   ,
    ClInventories                       =   2   ,
    ClNonCurrentAssets                  =   3   ,
    ClTangibleAssets                    =   4   ,
    ClIntangibleAssets                  =   5   ,
    ClCurrentLiabilities                =   6   ,
    ClNonCurrentLiabilities             =   7   ,
    ClEquity                            =   8   ,
    ClRevenue                           =   9   ,
    ClDirectCosts                       =   10  ,
    ClIndirectCosts                     =   11  ,
    ClOtherExpenses                     =   12  ,
    ClDepreciationAmortization          =   13  ,
    ClInterest                          =   14  ,
    ClExtraordinaryItems                =   15  ,
    ClTaxes                             =   16  ,
    ClOtherComprehensiveIncome          =   17  ,
    ClCashFlowOperations                =   18  ,
    ClCashFlowInvestments               =   19  ,
    ClCashFlowFinancing                 =   20  ,
    ClDcfCashFlows                      =   21  ,
    ClOthers                            =   22   
}

#[repr(u8)] #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GrClass {
    BalanceSheet                        =   1   ,
    ProfitLoss                          =   2   ,
    CashFlow                            =   3   
}

impl fmt::Display for Typez {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result { fmt::Debug::fmt(self,f) }
}

impl fmt::Display for Classez {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result { fmt::Debug::fmt(self,f) }
}

impl fmt::Display for GrClass {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result { fmt::Debug::fmt(self,f) }
}




pub struct Mapz{
    
    pub type_map: HashMap<String, Typez>, 
    pub class_map: HashMap<String, Classez>,
    pub gr_class_map: HashMap<String, GrClass>,

    pub type_int:   [Option<Typez>;     128],  
    pub class_int:  [Option<Classez>;    32],  
    pub gr_class_int: [Option<GrClass>;   4],  

    pub type_class:  HashMap<Classez, HashSet<Typez>>,
    pub class_gr_cl: HashMap<GrClass, HashSet<Classez>>

}

impl Default for Mapz {
    
    fn default() -> Mapz {

        use crate::Typez::*;
        use crate::Classez::*;
        use crate::GrClass::*;

        let mut type_map        = HashMap::<String, Typez>::new();
        let mut class_map       = HashMap::<String, Classez>::new();
        let mut gr_class_map    = HashMap::<String, GrClass>::new();

        let mut type_int = [None; 128];
        let mut class_int = [None; 32];
        let mut gr_class_int = [None; 4];

        let mut type_class = HashMap::<Classez, HashSet<Typez>>::new() ;
        let mut class_gr_cl = HashMap::<GrClass, HashSet<Classez>>::new();    

        {
            let mut addgrclass = | gc:GrClass | {
                gr_class_map.insert(gc.to_string(), gc);
                gr_class_int[gc as usize] = Some(gc);
                class_gr_cl.insert(gc, HashSet::<Classez>::new());
            };

            addgrclass(BalanceSheet); 
            addgrclass(ProfitLoss); 
            addgrclass(CashFlow);
        }

        {
            class_map.insert(ClOthers.to_string(), ClOthers);
            class_int[ClOthers as usize] = Some(ClOthers);
            type_class.insert(ClOthers, HashSet::<Typez>::new());

            let mut addclass = |cl: Classez, gc: GrClass| {
                class_map.insert(cl.to_string(), cl);
                class_int[cl as usize] = Some(cl);
                type_class.insert(cl, HashSet::<Typez>::new());
                class_gr_cl.get_mut(&gc).unwrap().insert(cl);
            };

            addclass(ClCurrentAssets, BalanceSheet);
            addclass(ClInventories, BalanceSheet);
            addclass(ClNonCurrentAssets, BalanceSheet);
            addclass(ClTangibleAssets, BalanceSheet);
            addclass(ClIntangibleAssets, BalanceSheet);
            addclass(ClCurrentLiabilities, BalanceSheet);
            addclass(ClNonCurrentLiabilities, BalanceSheet);
            addclass(ClEquity, BalanceSheet);
            addclass(ClRevenue, ProfitLoss);
            addclass(ClDirectCosts, ProfitLoss);
            addclass(ClIndirectCosts, ProfitLoss);
            addclass(ClOtherExpenses, ProfitLoss);
            addclass(ClDepreciationAmortization, ProfitLoss);
            addclass(ClInterest, ProfitLoss);
            addclass(ClExtraordinaryItems, ProfitLoss);
            addclass(ClTaxes, ProfitLoss);
            addclass(ClOtherComprehensiveIncome, ProfitLoss);
            addclass(ClCashFlowOperations, CashFlow);
            addclass(ClCashFlowInvestments, CashFlow);
            addclass(ClCashFlowFinancing, CashFlow);
            addclass(ClDcfCashFlows, CashFlow);
        }

        {
            let mut addtype = | ty:Typez, cl:Classez | {
                type_map.insert(ty.to_string(), ty);
                type_int[ty as usize] = Some(ty);
                type_class.get_mut(&cl).unwrap().insert(ty); 
            };
            addtype(Cash, ClCurrentAssets);
            addtype(CurrentReceivables, ClCurrentAssets);
            addtype(CurrentLoans, ClCurrentAssets);
            addtype(CurrentAdvances, ClCurrentAssets);
            addtype(OtherCurrentAssets, ClCurrentAssets);
            addtype(CurrentInvestmentsBv, ClCurrentAssets);
            addtype(CurrentInvestmentsMv, ClCurrentAssets);
            addtype(RawMaterials, ClInventories);
            addtype(WorkInProgress, ClInventories);
            addtype(FinishedGoods, ClInventories);
            addtype(AccountReceivables, ClNonCurrentAssets);
            addtype(LongTermLoans, ClNonCurrentAssets);
            addtype(LongTermAdvances, ClNonCurrentAssets);
            addtype(LongTermInvestmentsBv, ClNonCurrentAssets);
            addtype(LongTermInvestmentsMv, ClNonCurrentAssets);
            addtype(OtherLongTermAssets, ClNonCurrentAssets);
            addtype(PlantPropertyEquipment, ClTangibleAssets);
            addtype(AccumulatedDepreciation, ClTangibleAssets);
            addtype(LeasingRentalAssset, ClTangibleAssets);
            addtype(CapitalWip, ClTangibleAssets);
            addtype(OtherTangibleAssets, ClTangibleAssets);
            addtype(IntangibleAssets, ClIntangibleAssets);
            addtype(IntangibleAssetsDevelopment, ClIntangibleAssets);
            addtype(AccumulatedAmortization, ClIntangibleAssets);
            addtype(CurrentPayables, ClCurrentLiabilities);
            addtype(CurrentBorrowings, ClCurrentLiabilities);
            addtype(CurrentNotesPayable, ClCurrentLiabilities);
            addtype(OtherCurrentLiabilities, ClCurrentLiabilities);
            addtype(InterestPayable, ClCurrentLiabilities);
            addtype(CurrentProvisions, ClCurrentLiabilities);
            addtype(CurrentTaxPayables, ClCurrentLiabilities);
            addtype(LiabilitiesSaleAssets, ClCurrentLiabilities);
            addtype(CurrentLeasesLiability, ClCurrentLiabilities);
            addtype(AccountPayables, ClNonCurrentLiabilities);
            addtype(LongTermBorrowings, ClNonCurrentLiabilities);
            addtype(BondsPayable, ClNonCurrentLiabilities);
            addtype(DeferredTaxLiabilities, ClNonCurrentLiabilities);
            addtype(LongTermLeasesLiability, ClNonCurrentLiabilities);
            addtype(DeferredCompensation, ClNonCurrentLiabilities);
            addtype(DeferredRevenues, ClNonCurrentLiabilities);
            addtype(CustomerDeposits, ClNonCurrentLiabilities);
            addtype(OtherLongTermLiabilities, ClNonCurrentLiabilities);
            addtype(PensionProvision, ClNonCurrentLiabilities);
            addtype(LongTermProvisions, ClNonCurrentLiabilities);
            addtype(CommonStock, ClEquity);
            addtype(PreferredStock, ClEquity);
            addtype(PdInCapAbovePar, ClEquity);
            addtype(PdInCapTreasuryStock, ClEquity);
            addtype(RevaluationReserves, ClEquity);
            addtype(Reserves, ClEquity);
            addtype(RetainedEarnings, ClEquity);
            addtype(AccumulatedOci, ClEquity);
            addtype(MinorityInterests, ClEquity);
            addtype(OperatingRevenue, ClRevenue);
            addtype(NonOperatingRevenue, ClRevenue);
            addtype(ExciseStaxLevy, ClRevenue);
            addtype(OtherIncome, ClRevenue);
            addtype(CostMaterial, ClDirectCosts);
            addtype(DirectExpenses, ClDirectCosts);
            addtype(Salaries, ClIndirectCosts);
            addtype(AdministrativeExpenses, ClIndirectCosts);
            addtype(ResearchNDevelopment, ClIndirectCosts);
            addtype(OtherOverheads, ClIndirectCosts);
            addtype(OtherOperativeExpenses, ClIndirectCosts);
            addtype(OtherExpenses, ClOtherExpenses);
            addtype(ExceptionalItems, ClOtherExpenses);
            addtype(Pbitdax, ClDepreciationAmortization);
            addtype(Depreciation, ClDepreciationAmortization);
            addtype(Amortization, ClDepreciationAmortization);
            addtype(Pbitx, ClInterest);
            addtype(Interest, ClInterest);
            addtype(Pbtx, ClExtraordinaryItems);
            addtype(ExtraordinaryItems, ClExtraordinaryItems);
            addtype(PriorYears, ClExtraordinaryItems);
            addtype(Pbt, ClTaxes);
            addtype(TaxesCurrent, ClTaxes);
            addtype(TaxesDeferred, ClTaxes);
            addtype(Pat, ClTaxes);
            addtype(GainsLossesForex, ClOtherComprehensiveIncome);
            addtype(GainsLossesActurial, ClOtherComprehensiveIncome);
            addtype(GainsLossesSales, ClOtherComprehensiveIncome);
            addtype(FvChgAvlSale, ClOtherComprehensiveIncome);
            addtype(OtherDeferredTaxes, ClOtherComprehensiveIncome);
            addtype(OtherComprehensiveIncome, ClOtherComprehensiveIncome);
            addtype(TotalComprehensiveIncome, ClOtherComprehensiveIncome);
            addtype(ChgInventories, ClCashFlowOperations);
            addtype(ChgReceivables, ClCashFlowOperations);
            addtype(ChgLiabilities, ClCashFlowOperations);
            addtype(ChgProvisions, ClCashFlowOperations);
            addtype(CashFlowOperations, ClCashFlowOperations);
            addtype(InvestmentsPpe, ClCashFlowInvestments);
            addtype(InvestmentsCapDevp, ClCashFlowInvestments);
            addtype(AcqEquityAssets, ClCashFlowInvestments);
            addtype(DisEquityAssets, ClCashFlowInvestments);
            addtype(DisPpe, ClCashFlowInvestments);
            addtype(ChgInvestments, ClCashFlowInvestments);
            addtype(OtherCfInvestments, ClCashFlowInvestments);
            addtype(CashFlowInvestments, ClCashFlowInvestments);
            addtype(StockSales, ClCashFlowFinancing);
            addtype(StockRepurchase, ClCashFlowFinancing);
            addtype(DebtIssue, ClCashFlowFinancing);
            addtype(DebtRepay, ClCashFlowFinancing);
            addtype(Dividends, ClCashFlowFinancing);
            addtype(DonorContribution, ClCashFlowFinancing);
            addtype(CashFlowFinancing, ClCashFlowFinancing);
            addtype(Fcff, ClDcfCashFlows);
            addtype(Fcfe, ClDcfCashFlows);
            addtype(Fcfd, ClDcfCashFlows);
            addtype(SharesOutstanding, ClOthers);   

        }

        Mapz {
            type_map, class_map, gr_class_map, 
            type_int, class_int, gr_class_int,
            type_class, class_gr_cl,    
        }
    }
}

impl Mapz {

    pub fn print_json(&self) -> () {}

    pub fn find_group(&self, cl:Classez) -> Result<GrClass, &'static str> {
        for gr in &self.gr_class_int {
            if let Some(xg) = gr { 
                if Some(&cl) == self.class_gr_cl[xg].iter()
                    .find(|&&clx| clx == cl){ return Ok(*xg) }
            }
        }
        Err("Group class not found")
    }

    pub fn find_class(&self, ty:Typez) -> Result<Classez, &'static str> {
        for cl in &self.class_int {
            if let Some(xg) = cl { 
                if Some(&ty) == self.type_class[xg].iter()
                    .find(|&&tx| tx == ty){ return Ok(*xg) }
            }
        }
        Err("Classez not found")
    }


}

pub struct Document{

    time_beg: DateTime<Utc>,
    time_end: DateTime<Utc>,

    bE:f64,
    bD:f64,
    rf:f64,
    rm:f64,

    records : HashMap<Typez, Option<f64>>  



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




#[cfg(test)] mod mapz_test {
    use super::*;
    use crate::Typez::*;
    use crate::Classez::*;
    use crate::GrClass::*;

    #[test] fn online_test(){
        let a = Mapz { ..Default::default() } ;

        let mapr: Map<String, Value> = 
            serde_json::from_str(
                &reqwest::get("https://raw.githubusercontent.com/n-kishaloy/finanz/master/fintypes.json").unwrap().text().unwrap()
            ).unwrap();

        let typz = &mapr["types"];
        let clsz = &mapr["classes"];

        let strtpt = &mapr["connection"];

        fn jar2vc(gr_cl:&Value, typ:&str) -> Vec<String> {            
            let cl_ca = &gr_cl[typ].as_array().unwrap();
            (0..cl_ca.len()).map(|i| (match &cl_ca[i].clone() { 
                Value::String(x) => x, _ => "Non" }).to_owned()).collect()
        }

        let check_class = |cl:Classez|  {
            let grp = a.find_group(cl).unwrap();
            let mp=jar2vc(&strtpt[grp.to_string()],&cl.to_string());

            if cl as u8 != clsz[cl.to_string()] { return false }
            for ty in mp { 
                if cl != a.find_class(a.type_map[&ty]).unwrap() { return false }
                if typz[&ty] != (a.type_map[&ty] as u8) { return false }
            }
            true
        };

        assert!(
            a.class_int.iter().fold(true,|x, y| x && match *y { 
                None | Some(ClOthers) => true, _ => check_class(y.unwrap()) 
            }) &&

            a.type_int.iter().fold(true, |x,y| x && 
                if let Some(ty) = y { typz[ty.to_string()] == (*ty as u8) } 
                else { true }
            )
        );
    }

    #[test] fn find_class_test() {
        let a = Mapz { ..Default::default() } ;
        assert!(a.find_class(Cash)                  == Ok(ClCurrentAssets));
        assert!(a.find_class(CurrentInvestmentsBv)  == Ok(ClCurrentAssets));
        assert!(a.find_class(WorkInProgress)        == Ok(ClInventories));
        assert!(a.find_class(LongTermLoans)         == Ok(ClNonCurrentAssets));
        assert!(a.find_class(SharesOutstanding)     == Ok(ClOthers));
    }

    #[test] fn find_group_test() {
        let a = Mapz { ..Default::default() } ;
        assert!(a.find_group(ClCurrentAssets)       == Ok(BalanceSheet));
        assert!(a.find_group(ClCurrentLiabilities)  == Ok(BalanceSheet));
        assert!(a.find_group(ClRevenue)             == Ok(ProfitLoss));
        assert!(a.find_group(ClCashFlowOperations)  == Ok(CashFlow));
        assert!(a.find_group(ClIndirectCosts)       == Ok(ProfitLoss));
        assert!(a.find_group(ClDcfCashFlows)        == Ok(CashFlow));
        assert!(a.find_group(ClOthers)        == Err("Group class not found"));
    }

}