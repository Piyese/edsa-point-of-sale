// use std::time::SystemTime;
use std::{path::Path, io::Write};
use std::fs::{OpenOptions, self};
use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::{fetch_ext_debt_holders, fetch_int_debt_holders};

use super::inventory::{RawMaterial, FinishedProduct};
use super::{inventory::Product, people::Person};



#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq)]
pub struct TransactionIn {
    pub time: String,
    pub buyer: Person,
    pub items: Vec<Product>,
    pub total_cost: u32,
    pub bill_settled: bool,
    pub balance: Option<u32>
}

impl TransactionIn {
    pub fn new(buyer:Person)-> Self {
        let item_list:Vec<Product> = Vec::new();

        let time = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        let items=item_list;
        let total_cost=0;
        let bill_settled=false;
        let balance=None;
        Self{
            time,
            buyer,
            items,
            total_cost,
            bill_settled,
            balance,
        }
    }
    pub fn add_item(&mut self,item: Product){
        self.total_cost+=item.price*item.quantity;
        self.balance=Some(self.total_cost);
        self.items.push(item);
    }

    pub fn remove_last_added_item(&mut self){
        let item = self.items.pop().unwrap();
        self.total_cost-=item.price*item.quantity;
    }

    pub fn balance_books(&self, fp:  &mut FinishedProduct ) {
        // subtract the quantity bought from overall available quantity
        if let Some(p) = self.items.get(0){
            fp.available_quantity -= p.quantity;
            fp.log().unwrap();
        }
    }

    pub fn settle_bill(&mut self, amount:u32){
        if self.bill_settled{
            println!("nothing more to do.. the bill is fully settled")
        }else{
            if let Some(x)=self.balance{
                if amount>=x{
                    self.bill_settled=true;
                    self.balance=None;
                }else{
                    self.balance=Some(x-amount);
                    // for the debt tracker
                    let mut list = fetch_ext_debt_holders().unwrap();
                    dbg!(&list);
                    let mut existence_validator = false;

                    for de in list.iter_mut() {
                        if de.person == self.buyer {
                            println!(" in pattern1");
                            de.total_amount += self.balance.unwrap();
                            existence_validator = true;
                            break;
                        }
                    }
                    if !existence_validator {
                        println!(" in pattern2");
                        let ext = DebtExt {
                            person: self.buyer.to_owned(),
                            total_amount: self.balance.unwrap(),
                        };
                        list.push(ext);
                    }
                    // logging
                    let path = Path::new("records/ext_deni");
            
                    let mut file=std::fs::File::create(path).unwrap();
                    let new_l = serde_yaml::to_vec(&list).unwrap();
                    file.write_all(&new_l).unwrap();

                }
            }
        }
    }
    pub fn log(self){
        let path=Path::new("records/in_acc");//change to absolute path
        let trans_log=[self];
        let trans_log=serde_yaml::to_vec(&trans_log).unwrap();

        if path.exists(){
            let mut file=OpenOptions::new().append(true).open(path).expect("cant open file");
            let trans_log=&trans_log[4..];
            file.write_all(&trans_log).expect("cant write into..");
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&trans_log).expect("cant write into..");
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutTransaction {
    pub time: String,
    pub person: Person,
    pub raw_mat: RawMaterial,
    pub price_per_kg: u32,
    pub total_cost: u32,
    pub bill_settled: bool,
    pub balance: Option<u32>,
}

impl OutTransaction {
    pub fn new(person: Person, raw_mat: RawMaterial)-> Self {
        let time = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        let total_cost=0;
        let price_per_kg =0;
        let bill_settled=false;
        let balance=None;
        Self{
            time,
            person,
            raw_mat,
            price_per_kg,
            total_cost,
            bill_settled,
            balance,
        }
    }

    pub fn update(&mut self, price_per_kg: u32) {
        self.price_per_kg= price_per_kg;
        self.total_cost=price_per_kg*self.raw_mat.available_quantity;
        self.balance = Some(self.total_cost);
    }

    pub fn settle_bill(&mut self, amount:u32){
        if self.bill_settled{
            println!("nothing more to do.. the bill is fully settled")
        }else{
            if let Some(x)=self.balance{
                if amount>=x{
                    self.bill_settled=true;
                    self.balance=None;
                }else{
                    self.balance=Some(x-amount);
                    // debt tracking
                    let mut list = fetch_int_debt_holders().unwrap();
                    dbg!(&list);
                    let mut existence_validator = false;

                    for de in list.iter_mut() {
                        if de.person == self.person {
                            println!(" in pattern1");
                            de.total_amount += self.balance.unwrap();
                            existence_validator = true;
                            break;
                        }
                    }
                    if !existence_validator {
                        println!(" in pattern2");
                        let ext = DebtInt {
                            person: self.person.to_owned(),
                            total_amount: self.balance.unwrap(),
                        };
                        list.push(ext);
                    }
                    // logging
                    let path = Path::new("records/ext_deni");
            
                    let mut file=std::fs::File::create(path).unwrap();
                    let new_l = serde_yaml::to_vec(&list).unwrap();
                    file.write_all(&new_l).unwrap();

                }
            }
        }
    }

    pub fn balance_books(&self, rm: &mut RawMaterial){
        rm.available_quantity += self.raw_mat.available_quantity;
        rm.log().unwrap();
    }

    pub fn log(self){
        let path=Path::new("records/out_acc");//change to absolute path
        let trans_log=[self];
        let trans_log=serde_yaml::to_vec(&trans_log).unwrap();

        if path.exists(){
            let mut file=OpenOptions::new().append(true).open(path).expect("cant open file");
            let trans_log=&trans_log[4..];
            file.write_all(&trans_log).expect("cant write into..");
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&trans_log).expect("cant write into..");
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebtExt {
    pub person: Person,
    pub total_amount: u32,
}

impl DebtExt {
    pub fn settle_debt(&mut self, mut amount: u32, trans_list: &mut Vec<TransactionIn>) {
        if amount > self.total_amount {
            for trans in trans_list.iter_mut() {
                if !trans.bill_settled && self.person == trans.buyer {
                    trans.bill_settled = true;
                    trans.balance = None;
                }
            }
            self.total_amount = 0
        }else {
            self.total_amount -= amount;

            for trans in trans_list.iter_mut() {
                if !trans.bill_settled && self.person == trans.buyer {
                    if let Some(balance)=trans.balance{
                        if amount > balance {
                            amount -= balance; // balancing
                            trans.bill_settled = true;
                            trans.balance = None;
                        }else {
                            trans.balance = Some(balance - amount);
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DebtInt {
    pub person: Person,
    pub total_amount: u32,
}

impl DebtInt {
    pub fn settle_debt(&mut self, mut amount: u32, trans_list: &mut Vec<OutTransaction>) {
        if amount > self.total_amount {
            for trans in trans_list.iter_mut() {
                if !trans.bill_settled && self.person == trans.person {
                    trans.bill_settled = true;
                    trans.balance = None;
                }
            }
            self.total_amount = 0
        }else {
            self.total_amount -= amount;

            for trans in trans_list.iter_mut() {
                if !trans.bill_settled && self.person == trans.person {
                    if let Some(balance)=trans.balance{
                        if amount > balance {
                            amount -= balance; // balancing
                            trans.bill_settled = true;
                            trans.balance = None;
                        }else {
                            trans.balance = Some(balance - amount);
                            break;
                        }
                    }
                }
            }
        }
    }
}