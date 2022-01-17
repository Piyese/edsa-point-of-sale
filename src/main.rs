use chrono::Local;
use edsa_pos::{sale::{people::{Employee, Sex, Person, Role}, inventory::{FinishedProduct, Product, RawMaterial, DailyYield}, accounts::{TransactionIn, OutTransaction}, errors::PosError}, fetch_finished_product_log};


fn main()->Result <(), PosError>{
    // // create supplier,customer,employer
    // let _e = Employee::new("Slav".to_string(), Sex::Male, true, "0101019181".to_string());
    // let s = Person::new(Role::Supplier, "SAiyona".to_string(), "038271928".to_string());
    // let c = Person::new(Role::Customer, "Asap".to_string(), "3823344242".to_string());

    // // // create a "finishedproduct" struct
    // let mut fc = FinishedProduct::new("Flour".to_string(), 87);
    // let p = Product::new("Flour".to_string(), None, 60, 5);
    // fc.log()?;

    // // an in_transaction
    // let mut int = TransactionIn::new(c);
    // int.add_item(p);
    // int.settle_bill(200);
    // int.balance_books(&mut fc);
    // int.balance_books(&mut fc);
    // int.log();

    // //raw material for out_transaction
    // let mut main_raw_mat= RawMaterial::new("Maize".to_string(), 123);
    // main_raw_mat.log().unwrap();
    // // an out_transaction
    // let raw_mat= RawMaterial::new("Maize".to_string(), 12);

    // let mut outt =OutTransaction::new(s, raw_mat);
    // outt.update(90);
    // outt.balance_books(&mut main_raw_mat);
    // outt.settle_bill(300);
    // outt.log();
    
    // // record a daily-yield, try twice
    // let _dy = DailyYield::new("Flour".to_string(), 984, &mut fc);
    // let _dx = DailyYield::new("Flour".to_string(), 984, &mut fc);
    // let x = fetch_finished_product_log().unwrap();
    // println!("{:?}",x);
    // let x = edsa_pos::fetch_raw_material_log()?;
    // println!("{:?}",x);

    // let now = Local::now(); //2022-01-15T12:19:03.123970801+03:00
    // let s = now.format("%d-%m-%Y %H:%M:%S").to_string();
    // println!("{:?}",&s);


    Ok(())
}