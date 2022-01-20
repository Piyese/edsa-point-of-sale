use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize ,Serialize};
use crate::fetch_pkg_log;

use super::{custom_date::Date, errors::PosError};



#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RawMaterial {
    pub name: String,
    pub quantity: f32,
    pub price_per: f32,
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
    /// let material = RawMaterial::new("Soda".to_string(), 12.4 );
    /// ```
    /// 

    pub fn new(name: String, quantity: f32)->Self {
        Self {name, quantity, price_per: 1.}
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
        self.raw_mat.push(r);
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
    pub pkg_specify: Option<String>,
    // qty per pack in kg
    pub quantity: f32,
    pub cost: f32,
    // total no. of packs
    pub total: u32
}

impl crate::LogPartial for PackagedProd {}

impl PackagedProd {
    pub fn new(product: Product)->Self {
        Self {
            product,
            pkg_specify: None,
            quantity: 0.,
            cost: 0.,
            total: 0,
        }
    }
    
    // pub fn sell_pkg (name: String)->Self{
    //     // the big idea here is that the buyer gets an already cobstructed PackagedProd,
    //     // with set price and quantity and name,
    //     // only the total is set to zero so as the buyer can add whatever no. to her cart
    //     let pkg_list = fetch_pkg_log()?;
    //     for (i, pkg) in pkg_list.iter().enumerate() {
    //         if let Some(spec) = pkg.pkg_specify{
    //             if spec == name {
    //                 let pkg_cln = pkg_list[i].clone();
    //                 // change the total back to zero
    //                 pkg_cln.total = 0;
    //                 Ok(pkg_cln)
    //             }
    //         }
    //     }
    // }

    pub fn specify_qty(&mut self, qty: f32) {
        self.quantity = qty;
    }

    pub fn specify_cost(&mut self, amt: f32) {
        self.cost = amt;
    }

    pub fn specify_pkg(&mut self, name: String) {
        self.pkg_specify=Some(name);
    }

    pub fn add_packs(&mut self, amt: u32) {
        self.total += amt;
    } 
}

