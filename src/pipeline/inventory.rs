use std::{time::{SystemTime, UNIX_EPOCH}, path::Path, io::Write, fs};
use serde::{Deserialize ,Serialize};

use crate::{fetch_logs, PathOption, fetch_daily_logs};

use super::{custom_date::Date, errors::PosError};



#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RawMaterial {
    pub name: String,
    pub quantity: f32,
    // materials in store will have this as None
    // when construting an instance for transaction purposes, then it has to be Some(thing)
    pub price_per: Option<f32>,
}

impl crate::LogPartial for RawMaterial {}

impl RawMaterial {
    /// Creates a new [`RawMaterial`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::sale::inventory::RawMaterial;
    /// 
    /// // to construct a new instance for a supplier.. first call..
    /// let material = RawMaterial::new("Soda".to_string(), 12.4 );
    /// 
    /// // then log the instance
    /// material.local_log();
    /// 
    /// // then add the price its being offered at per kg
    /// material.price(65)
    /// 
    /// ```

    pub fn new(name: String, quantity: f32)->Self {
        Self {name, quantity, price_per: None}
    }

    pub fn price(&mut self, amt: u32) {
        self.price_per = Some(amt as f32);
    }

                
    pub fn local_log(&self)->Result<(), PosError> {
        let mut rm_list = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
        let mut existence_validator = false;
        for rm in rm_list.iter_mut() {
            if rm.name == self.name {
                rm.quantity += self.quantity;
                existence_validator = true;
                
                let path = Path::new("records/rawmat");
                let mut file=fs::File::create(path)?;
                let item_log=serde_yaml::to_vec(&rm_list)?;
                file.write_all(&item_log)?;
                break;
            }
        }
        // if first appearance of raw material
        if !existence_validator {
            let slf = RawMaterial::new(self.name.to_owned(), self.quantity);
            rm_list.push(slf);
            
            let path = Path::new("records/rawmat");

            let mut file=fs::File::create(path)?;
            let item_log=serde_yaml::to_vec(&rm_list)?;
            file.write_all(&item_log)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product {
    pub name: String
}

impl crate::LogPartial for Product {}

impl Product {
    pub fn new(name: String)->Self {
        Self {name}
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Production {
    pub date: Date,
    pub product: Product,  // product being produced
    pub raw_mat: Vec<RawMaterial>,  // list of raw materials used
    // pub quantity_produced: f32
}

impl crate::LogPartial for Production {}

impl Production {
    pub fn new(product: Product)->Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let date = Date::at_utc(time as i64);
        let raw_mat: Vec<RawMaterial> = Vec::new();

        Self {date, product, raw_mat}
    }
    pub fn add_rawmat(&mut self, r: RawMaterial) {
        self.raw_mat.push(r.clone());

        // subtract from raw material in stock
        let mut rm_list = fetch_logs::<RawMaterial>(PathOption::RawMat).unwrap();
        
        for rm in rm_list.iter_mut() {
            if rm.name == r.name {
                rm.quantity -= r.quantity;
                break;
            }
        }
        
        // log
        let path = Path::new("records/rawmat");

        let mut file=std::fs::File::create(path).unwrap();
        let new_l = serde_yaml::to_vec(&rm_list).unwrap();
        file.write_all(&new_l).unwrap();

    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct DailyYield{
    pub date: Date,
    pub product: Product,
    pub quantity: f32
}
impl DailyYield {
    pub fn new(product: Product, quantity: f32)->Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let date = Date::at_utc(time as i64);
        // balances the books and logs the stock change

        //fetch the finished products list
        let mut fp_list = fetch_logs::<FinishedProd>(PathOption::FinProd).unwrap();

        // iterate over, find the unique product, add the quantity
        for fp in fp_list.iter_mut() {
            if fp.product == product {
                fp.quantity += quantity;
                break;
            }
        }
        // logging the finished products

        let path = Path::new("records/finprod");

        let mut file=std::fs::File::create(path).unwrap();
        let new_l = serde_yaml::to_vec(&fp_list).unwrap();
        file.write_all(&new_l).unwrap();

        // logging daily yield manenos
        let path_str = format!("records/{}dyield",product.name);
        let path = Path::new(&path_str);


        let mut dl = fetch_daily_logs(path).unwrap();
        let i =dl.len();
        if !dl.is_empty(){
            if date == dl[i-1].date {
                dl[i-1].quantity += quantity;
                let dlu8 = serde_yaml::to_vec(&dl).unwrap();
                let mut file=std::fs::File::create(path).unwrap();
                file.write_all(&dlu8).unwrap();
                return dl[i-1].clone();
            }else {
                let prod = product.clone();
                let slf = Self {date, product: prod, quantity};
                let dailyf = slf.clone();
                dl.push(slf);
                let dlu8 = serde_yaml::to_vec(&dl).unwrap();
                let mut file=std::fs::File::create(path).unwrap();
                file.write_all(&dlu8).unwrap();
                return dailyf;
            }
        }

        let prod = product.clone();
        let mut file=std::fs::File::create(path).unwrap();
        let new_l = serde_yaml::to_vec(&[Self {date, product, quantity}]).unwrap();
        file.write_all(&new_l).unwrap();

        Self {date, product: prod, quantity}
    }  
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq,  PartialOrd )]
pub struct FinishedProd {
    pub product: Product,
    pub quantity: f32
}

impl crate::LogPartial for FinishedProd {}

impl FinishedProd {
    pub fn new(product: Product)->Self {
        Self{product, quantity: 0.}
    }

    pub fn add(&mut self, amt: f32) {
        self.quantity+=amt;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct PackagedProd {
    pub product: Product,
    // i.e Pembe 2kg
    pub pkg_specify: String,
    // qty per pack in kg
    pub quantity: f32,
    pub cost: f32,
    // total no. of packs
    pub total: u32
}

impl crate::LogPartial for PackagedProd {}

impl PackagedProd {
    /// Creates a new [`PackagedProd`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::sale::inventory::PackagedProd;
    /// use edsa_pos::sale::inventory::Product;
    /// 
    /// // to construct a new instance for stock addition
    /// let pkg = PackagedProd::new(product);
    /// 
    /// // then add the price its being offered at per kg
    /// pkg.price(65)
    /// 
    /// // then log the instance.
    /// // on the frontend this should added to the local pkgprod list
    /// pkg.log();
    /// 
    /// 
    /// ```
    pub fn new(product: Product)->Self {
        Self {
            product,
            pkg_specify: String::default(),
            quantity: 0.,
            cost: 0.,
            total: 0,
        }
    }
    
    /// the big idea here is that the buyer gets an already cobstructed PackagedProd,
    /// with set price and quantity and name,
    /// only the total is set to zero so as the buyer can add whatever no. to her cart
    pub fn sell_pkg (name: String)->Option<Self>{
        let pkg_list = fetch_logs::<PackagedProd>(PathOption::PkgProd).unwrap();
        for (i, pkg) in pkg_list.iter().enumerate() {
            if pkg.pkg_specify == name {
                let mut pkg_cln = pkg_list[i].clone();
                // change the total back to zero
                pkg_cln.total = 0;
                return Some(pkg_cln);
            }
        }
        None
    }

    pub fn specify_qty(&mut self, qty: f32) {
        self.quantity = qty;
    }

    pub fn specify_cost(&mut self, amt: f32) {
        self.cost = amt;
    }

    pub fn specify_pkg(&mut self, name: String) {
        self.pkg_specify = name;
    }

    pub fn add_packs(&mut self, amt: u32) {
        self.total += amt;
    } 
}

