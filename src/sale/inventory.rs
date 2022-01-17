
use serde::{Deserialize, Serialize};
use std::{time::{SystemTime, UNIX_EPOCH}, path::Path, io::Write};

use crate::fetch_daily_logs;

use super::{custom_date::Date, errors::PosError};

//record available quantity of specified raw material
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawMaterial {
    pub material: String,
    pub available_quantity: u32,
}

impl RawMaterial {
    /// Creates a new [`RawMaterial`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::sale::inventory::RawMaterial;
    ///
    /// let material = RawMaterial::new("Soda".to_string(), 12 );
    /// ```
    /// 

    pub fn new(material: String, available_quantity: u32 )->Self {
        Self {material, available_quantity,}
    }

    pub fn log(&self)->Result<(),PosError> {
        let path = Path::new("records/rawmat");
        // std::fs::
        let mut file=std::fs::File::create(path)?;
        let fp = serde_yaml::to_vec(&self)?;
        file.write_all(&fp)?;
        Ok(())
    }
}

impl Default for RawMaterial{
    fn default() -> Self {
        Self { material: String::from("Maize"), available_quantity: 0 }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DailyYield{
    pub date: Date,
    pub product: String,
    pub quantity: u32
}
impl DailyYield {
    pub fn new(product: String, quantity: u32, stock: &mut FinishedProduct)->Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let date = Date::at_utc(time as i64);
        // balances the books and logs the stock change
        stock.available_quantity += quantity;
        stock.log().unwrap();
        
        let path = Path::new("records/dailyyield");
        let mut dl = fetch_daily_logs().unwrap();
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


//record the available quantity of the final product as it changes
#[derive(Debug, Deserialize, Serialize)]
pub struct FinishedProduct {
    pub product: String,
    pub available_quantity: u32,
}

impl FinishedProduct {
    pub fn new(product: String, available_quantity: u32)->Self {
        Self {product, available_quantity}
    }
    pub fn log(&self)->Result<(),PosError> {
        let path = Path::new("records/finishedproduct");
        // std::fs::
        let mut file=std::fs::File::create(path)?;
        let fp = serde_yaml::to_vec(&self)?;
        file.write_all(&fp)?;
        Ok(())
    }
}
impl Default for FinishedProduct {
    fn default() -> Self {
        Self::new("Flour".to_string(), 0)
    }
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq)]
pub struct Product{
    pub name: String,
    pub package_specifier:Option<String>,
    pub price: u32,
    pub quantity: u32,
}
impl Product{
    pub fn new(
        name: String,
        package_specifier:Option<String>,
        price:u32,
        quantity:u32
    )->Self{
        Self{
            name,
            package_specifier,
            price,
            quantity
        }
    }   
    pub fn update_price(&mut self,new_price:u32){
        self.price=new_price;
    }   
    pub fn add_quantity(&mut self,to_add:u32){
        self.quantity+=to_add;
    }
    pub fn subtract_quantity(&mut self,to_sub:u32){
        self.quantity-=to_sub;
    }
}
