pub mod pipeline;
use std::{path::Path, fs::{OpenOptions, self}, io::Write};
use serde::{Serialize, Deserialize};
use pipeline::{accounts::{TransactionIn, OutTransaction, DebtExt, DebtInt}, errors::PosError, inventory::{ FinishedProd, RawMaterial, PackagedProd, Product}, people::{Person,Employee}};


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

pub fn fetch_int_debt_holders()->Result<Vec<DebtInt>, PosError> {
    let path = Path::new("records/int_deni");

    if path.exists(){
        let data = std::fs::read(path)?;
        let people_log: Vec<DebtInt> = serde_yaml::from_slice(&data)?;
        Ok(people_log)
    }else{
        let people_log:Vec<DebtInt>=Vec::default();
        Ok(people_log)
    }
}

pub fn fetch_pkg_log()->Result<Vec<PackagedProd>, PosError> {
    let path = Path::new("records/pkgprod");

    if path.exists(){
        let data = std::fs::read(path)?;
        let pkg_log: Vec<PackagedProd> = serde_yaml::from_slice(&data)?;
        Ok(pkg_log)
    }else{
        let pkg_log:Vec<PackagedProd>=Vec::default(); 
        Ok(pkg_log)
    }
}

pub fn fetch_rawmat_log()->Result<Vec<RawMaterial>, PosError> {
    let path = Path::new("records/rawmat");

    if path.exists(){
        let data = std::fs::read(path)?;
        let rawmat: Vec<RawMaterial> = serde_yaml::from_slice(&data)?;
        Ok(rawmat)
    }else{
        let rawmat:Vec<RawMaterial>=Vec::default(); 
        Ok(rawmat)
    }
}

pub fn fetch_finished_prod_log()->Result<Vec<FinishedProd>, PosError> {
    let path = Path::new("records/finprod");

    if path.exists(){
        let data = std::fs::read(path)?;
        let fd: Vec<FinishedProd> = serde_yaml::from_slice(&data)?;
        Ok(fd)
    }else{
        let fd:Vec<FinishedProd>=Vec::default(); 
        Ok(fd)
    }
}

pub fn fetch_product_log()->Result<Vec<Product>, PosError> {
    let path = Path::new("records/products");

    if path.exists(){
        let data = std::fs::read(path)?;
        let prod: Vec<Product> = serde_yaml::from_slice(&data)?;
        Ok(prod)
    }else{
        let prod:Vec<Product>=Vec::default(); 
        Ok(prod)
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
