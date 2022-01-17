pub mod sale;

use std::path::Path;

use sale::{accounts::{TransactionIn, OutTransaction, DebtExt}, errors::PosError, inventory::{DailyYield,FinishedProduct, RawMaterial}, people::{Person,Employee}};


pub fn fetch_transaction_in_log()->Result<Vec<TransactionIn>, PosError>{
    let path=Path::new("records/in_acc");

    if path.exists(){
        let data = std::fs::read(path)?;
        let trans_log: Vec<TransactionIn> = serde_yaml::from_slice(&data)?;
        // println!("{:?}",trans_log);
        Ok(trans_log)
    }else{
        let trans_log:Vec<TransactionIn>=Vec::default();
        Ok(trans_log)
    }
}

pub fn fetch_transaction_out_log()->Result<Vec<OutTransaction>, PosError>{
    let path=Path::new("records/out_acc");

    if path.exists(){
        let data = std::fs::read(path)?;
        let trans_log: Vec<OutTransaction> = serde_yaml::from_slice(&data)?;
        // println!("{:?}",trans_log);
        Ok(trans_log)
    }else{
        let trans_log:Vec<OutTransaction>=Vec::default();
        Ok(trans_log)
    }
}

pub fn fetch_daily_logs()->Result< Vec<DailyYield>, PosError> {
    let path = Path::new("records/dailyyield");
    
    if path.exists(){
        let data = std::fs::read(path)?;
        let daily_log: Vec<DailyYield> = serde_yaml::from_slice(&data)?;
        Ok(daily_log)
    }else{
        let daily_log:Vec<DailyYield>=Vec::default();
        Ok(daily_log)
    }
}
pub fn fetch_employee_logs()->Result< Vec<Employee>, PosError> {
    let path = Path::new("records/employees");
    
    if path.exists(){
        let data = std::fs::read(path)?;
        let emp_log: Vec<Employee> = serde_yaml::from_slice(&data)?;
        Ok(emp_log)
    }else{
        let emp_log:Vec<Employee>=Vec::default();
        Ok(emp_log)
    }
}
pub fn fetch_people_logs()->Result< Vec<Person>, PosError> {
    let path = Path::new("records/people");
    
    if path.exists(){
        let data = std::fs::read(path)?;
        let people_log: Vec<Person> = serde_yaml::from_slice(&data)?;
        Ok(people_log)
    }else{
        let people_log:Vec<Person>=Vec::default();
        Ok(people_log)
    }
} 
pub fn fetch_finished_product_log()-> Result<FinishedProduct, PosError> {
    let path = Path::new("records/finishedproduct");
    
    if path.exists(){
        let data = std::fs::read(path)?;
        let fd: FinishedProduct = serde_yaml::from_slice(&data)?;
        Ok(fd)
    }else{
        let fd = FinishedProduct::default();
        Ok(fd)
    }
}

pub fn fetch_raw_material_log()->Result<RawMaterial, PosError> {
    let path = Path::new("records/rawmat"); 

    if path.exists() {
        let data = std::fs::read(path)?;
        let rm: RawMaterial = serde_yaml::from_slice(&data)?;
        Ok(rm)
    }else {
        let rm  = RawMaterial::default();
        Ok(rm)
    }
}

// pub fn all_debt_holders() -> Result<Vec<Person>, PosError> {
//     let dlist = fetch_transaction_in_log()?;

//     if dlist.is_empty() {
//         println!("no transactions");
//         let names: Vec<Person> = Vec::new();
//         Ok(names)
//     }else {
//         let p: Vec<_> = dlist.into_iter().filter(|t| t.bill_settled == false ).collect();
//         let mut names: Vec<Person> = Vec::new();
//         while let Some(tr) = p.to_owned().into_iter().next() {
//             if !names.contains(&tr.buyer) { names.push(tr.buyer) }
//         }
//         Ok(names)
//     }
// }

pub fn fetch_ext_debt_holders()->Result<Vec<DebtExt>, PosError> {
    let path = Path::new("records/ext_deni");

    if path.exists(){
        let data = std::fs::read(path)?;
        let people_log: Vec<DebtExt> = serde_yaml::from_slice(&data)?;
        Ok(people_log)
    }else{
        let people_log:Vec<DebtExt>=Vec::default();
        Ok(people_log)
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
