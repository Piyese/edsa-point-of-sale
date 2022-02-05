use std::{path::Path, io::Write, fs};
use crate::{fetch_logs, PathOption};
use super::{people::Person, inventory::{PackagedProd, RawMaterial}};
use chrono::Local;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct TransactionIn {
    pub time: String,
    pub person: Person,
    pub items: Vec<PackagedProd>,
    pub total_cost: f32,
    pub bill_settled: bool,
    pub balance: Option<f32>
}

impl crate::LogPartial for TransactionIn{}

impl TransactionIn {
    pub fn new(person: Person)-> Self {
        let time = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        let items: Vec<PackagedProd> = Vec::new();
        let total_cost=0.;
        let bill_settled=false;
        let balance=None;
        Self{
            time,
            person,
            items,
            total_cost,
            bill_settled,
            balance,
        }
    }
    pub fn add(&mut self, p: PackagedProd) {
        let tc = p.cost * p.total as f32;
        self.total_cost += tc;
        self.items.push(p);
        self.balance = Some(self.total_cost);
    }

    /// subtract the bought packaged products from total packaged products
    /// and log the new list 
    pub fn balance_books(&self) {
        let mut pkg_list = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
        for item in &self.items {
            for pkg in pkg_list.iter_mut() {
                if pkg.pkg_specify == item.pkg_specify {
                    pkg.total -= item.total;
                    break;
                }
            }
        }
        // log
        let path = Path::new("records/pkgprod");

        let mut file=std::fs::File::create(path).unwrap();
        let new_l = serde_yaml::to_vec(&pkg_list).unwrap();
        file.write_all(&new_l).unwrap();

    }

    pub fn settle_bill(&mut self, amount:f32){
        if self.bill_settled{
            println!("nothing more to do.. the bill is fully settled")
        }else{
            if let Some(x)=self.balance{
                if amount >= x{
                    self.bill_settled=true;
                    self.balance=None;
                }else{
                    self.balance=Some(x-amount);

                    // debt tracking
                    let mut list = fetch_logs::<Debtor>(PathOption::Debtors).unwrap();
                    let mut existence_validator = false;

                    for de in list.iter_mut() {
                        if de.person == self.person {
                            println!(" in pattern1");
                            de.total_amount += self.balance.unwrap() as u32; // f32 to u32 ***************
                            existence_validator = true;
                            break;
                        }
                    }
                    if !existence_validator {
                        println!(" in pattern2");
                        let ext = Debtor {
                            person: self.person.to_owned(),
                            total_amount: self.balance.unwrap() as u32, // f32 to u32 ***************
                        };
                        list.push(ext);
                    }

                    // logging the debt record
                    let path = Path::new("records/debtors");
            
                    let mut file=std::fs::File::create(path).unwrap();
                    let new_l = serde_yaml::to_vec(&list).unwrap();
                    file.write_all(&new_l).unwrap();
                }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct OutTransaction {
    pub time: String,
    pub person: Person,
    pub items: Vec<RawMaterial>,
    pub total_cost: f32,
    pub bill_settled: bool,
    pub balance: Option<f32>
}

impl crate::LogPartial for OutTransaction{}

impl OutTransaction {
    /// Creates a new [`OutTransaction`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::pipeline::accounts::OutTransaction;
    /// use edsa_pos::pipeline::people::Person;
    /// 
    /// let p = Person::new("Holy".to_string(), "0789564132".to_string());
    /// let ot = OutTransaction::new(p);
    /// 
    /// ```
    pub fn new(person: Person)-> Self {
        let time = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        let items: Vec<RawMaterial> = Vec::new();
        let total_cost=0.;
        let bill_settled=false;
        let balance=None;
        Self{
            time,
            person,
            items,
            total_cost,
            bill_settled,
            balance,
        }
    }

    pub fn add(&mut self, r: RawMaterial) {
        let tc = r.quantity * r.price_per.unwrap(); // should use an 'if let'
        self.total_cost += tc;
        self.balance = Some(self.total_cost);
        self.items.push(r);
    }

    /// add the quantity of bought materials to overall quantity
    pub fn balance_books(&self) {
        let mut rm_list = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
        for item in &self.items {
            for rawmat in rm_list.iter_mut() {
                if rawmat.name == item.name {
                    rawmat.quantity += item.quantity;
                    break;
                }
            }
        }
        // log the raw material
        let path = Path::new("records/rawmat");
        let item_log=serde_yaml::to_vec(&rm_list).unwrap();
        let mut file=fs::File::create(path).expect("cant open file");
        file.write_all(&item_log).expect("cant write into..");
    }

    pub fn settle_bill(&mut self, amount:f32){
        if self.bill_settled{
            println!("nothing more to do.. the bill is fully settled")
        }else{
            if let Some(x)=self.balance{
                if amount >= x{
                    self.bill_settled=true;
                    self.balance=None;
                }else{
                    self.balance=Some(x-amount);

                    // debt tracking
                    let mut list = fetch_logs::<Creditor>(PathOption::Creditor).unwrap();
                    let mut existence_validator = false;

                    for di in list.iter_mut() {
                        if di.person == self.person {
                            println!(" in pattern1");
                            di.total_amount += self.balance.unwrap() as u32; // f32 to u32 ***************
                            existence_validator = true;
                            break;
                        } 
                    }
                    if !existence_validator {
                        println!(" in pattern2");
                        let int = Creditor {
                            person: self.person.to_owned(),
                            total_amount: self.balance.unwrap() as u32, // f32 to u32 ***************
                        };
                        list.push(int);
                    }

                    // logging the debt record
                    let path = Path::new("records/creditors");
            
                    let mut file=std::fs::File::create(path).unwrap();
                    let new_l = serde_yaml::to_vec(&list).unwrap();
                    file.write_all(&new_l).unwrap();
                }
            }
        }
    }
}


#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Debtor {
    pub person: Person,
    pub total_amount: u32,
}

impl crate::LogPartial for Debtor{}

impl Debtor {
    pub fn settle_debt(&mut self, mut amount: u32, trans_list: &mut Vec<TransactionIn>) {
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
                        if amount as f32 > balance {  // f32 to u32 ***************
                            // balancing
                            amount -= balance as u32; // u32 to f32 ***************
                            trans.bill_settled = true;
                            trans.balance = None;
                        }else {
                            trans.balance = Some(balance - amount as f32);//********
                            break;
                        }
                    }
                }
            }
        }
        // log
        let path_a = Path::new("records/in_acc");
        let item_log=trans_list.to_owned();
        let item_log=serde_yaml::to_vec(&item_log).unwrap();
        let mut file=fs::File::create(path_a).expect("cant open file");
        file.write_all(&item_log).expect("cant write into..");
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Creditor {
    pub person: Person,
    pub total_amount: u32,
}

impl crate::LogPartial for Creditor{}

impl Creditor {
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
                        if amount as f32 > balance { // *********
                            amount -= balance as u32; // balancing / ********
                            trans.bill_settled = true;
                            trans.balance = None;
                        }else {
                            trans.balance = Some(balance - amount as f32); // *********
                            break;
                        }
                    }
                }
            }
        }
        let path_a = Path::new("records/out_acc");
        let item_log=trans_list.to_owned();
        let item_log=serde_yaml::to_vec(&item_log).unwrap();
        let mut file=fs::File::create(path_a).expect("cant open file");
        file.write_all(&item_log).expect("cant write into..");
    }
}