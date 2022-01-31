pub mod pipeline;
use std::{path::Path, fs::{OpenOptions, self}, io::Write};
use serde::{Serialize, de};
use pipeline::{inventory::DailyYield, errors::PosError};


pub trait LogPartial {

    fn log(&self, path: &Path) where Self: Serialize {
        let item_log=[self];
        let item_log=serde_yaml::to_vec(&item_log).unwrap();

        if path.exists(){
            let mut file=OpenOptions::new().append(true).open(path).expect("cant open file");
            let item_log=&item_log[4..];
            file.write_all(&item_log).expect("cant write into..");
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&item_log).expect("cant write into..");
        }
    }

}

pub enum PathOption {
    TransIn,
    TransOut,
    FinProd,
    Product,
    Production,
    PkgProd,
    Staff,
    People,
    RawMat,
    Creditor,
    Debtors,
}

/// takes a [`PathOption`] which determines which path to use.
/// # example
/// 
/// ```
/// use edsa_pos::fetch_logs;
/// use edsa_pos::PathOption;
/// use edsa_pos::pipeline::accounts::{TransactionIn, OutTransaction};
/// 
/// // method A
/// let example_vec = fetch_logs::<TransactionIn>(PathOption::TransIn).unwrap(); 
/// 
/// // method B
/// let example2: Vec<OutTransaction> = fetch_logs(PathOption::TransOut).unwrap(); 
/// 
pub fn fetch_logs<T: LogPartial + de::DeserializeOwned> (opt: PathOption) -> Result<Vec<T>, PosError> {
    let path = match opt {
        PathOption::TransIn => Path::new("records/in_acc"),
        PathOption::TransOut => Path::new("records/out_acc"),
        PathOption::FinProd => Path::new("records/finprod"),
        PathOption::Product => Path::new("records/products"),
        PathOption::Production => Path::new("records/production"),
        PathOption::PkgProd => Path::new("records/pkgprod"),
        PathOption::Staff => Path::new("records/employees"),
        PathOption::People => Path::new("records/people"),
        PathOption::RawMat => Path::new("records/rawmat"),
        PathOption::Creditor => Path::new("records/creditors"),
        PathOption::Debtors => Path::new("records/debtors"),
    };

    if path.exists(){
        let data = std::fs::read(path)?;
        let item_log: Vec<T> = serde_yaml::from_slice(&data)?;
        Ok(item_log)
    }else{
        let item_log:Vec<T>=Vec::new();
        Ok(item_log)
    }
}

pub fn fetch_daily_logs(path: &Path)-> Result<Vec<DailyYield>, PosError>{
    if path.exists(){
        let data = std::fs::read(path)?;
        let item_log: Vec<DailyYield> = serde_yaml::from_slice(&data)?;
        Ok(item_log)
    }else{
        let item_log:Vec<DailyYield>=Vec::new();
        Ok(item_log)
    }
}
#[cfg(test)]
mod tests {
    // use super::*;

    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
