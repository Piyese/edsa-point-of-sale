use std::path::Path;

use chrono::Local;
use edsa_pos::{pipeline::{errors::PosError, people::{Employee, Person, Sex}, inventory::{Product, FinishedProd, PackagedProd, RawMaterial, Production, DailyYield}, accounts::{TransactionIn, OutTransaction}}};
use edsa_pos::LogPartial;


fn main()->Result <(), PosError>{
    // let path = Path::new("records/employees");
    // let path2 = Path::new("records/people");
    // let path3 = Path::new("records/products");
    // let path4 = Path::new("records/finprod");
    // let path5 = Path::new("records/pkgprod");
    // let path6 = Path::new("records/in_acc");
    // let path7 = Path::new("records/out_acc");
    // let path8 = Path::new("records/rawmat");

    // create the people
    // let e = Employee::new("Femi two".to_string(), Sex::Female, true, "010654181".to_string());
    // e.log(path);
    // let s = Person::new( "Sawk".to_string(), "038270028".to_string());
    // let c = Person::new( "ivy".to_string(), "3353344242".to_string());
    // s.log(path2);
    // c.log(path2);
    // // product to be produce
    // let pr = Product::new(String::from("Flour") );
    // let pr2 = Product::new(String::from("Kuku Feeds") );
    // pr.log(path3);
    // pr2.log(path3);

    // // Finished Products(unpacked)
    // let mut fd = FinishedProd::new(pr.clone()) ;
    // let mut fd2 = FinishedProd::new(pr2.clone()) ;
    // fd.add(500.);
    // fd2.add(740.);
    // fd.log(path4);
    // fd2.log(path4);

    // // packaged products 
    // let mut pembe = PackagedProd::new(pr.clone());
    // pembe.specify_pkg("Pembe small".to_string() );
    // pembe.specify_qty(2.);
    // pembe.add_packs(25);

    // let mut chick_mash = PackagedProd::new(pr2.clone());
    // chick_mash.specify_pkg("Chick Mash".to_string());
    // chick_mash.specify_qty(0.5);
    // chick_mash.add_packs(87);

    // pembe.log(path5);
    // chick_mash.log(path5);

    // // buyer c walks in
    // let mut trans = TransactionIn::new(c);
    // // items to buy
    // let mut buy_item = PackagedProd::sell_pkg("Chick Mash".to_string()).unwrap();
    // buy_item.add_packs(10);
    // dbg!(&buy_item);
    // let mut buy_item2 = PackagedProd::sell_pkg("Pembe small".to_string()).unwrap();
    // buy_item2.add_packs(10);
    // dbg!(&buy_item2);
      
    // trans.add(buy_item);  
    // trans.add(buy_item2);

    // trans.balance_books();
    // trans.settle_bill(3000.);

    // dbg!(&trans);
    // trans.log(path6);

    // // to buy
    // let mut fresh = RawMaterial::new("beans".to_string(), 50.5);//new
    // fresh.local_log();
    // fresh.price(45);

    // let mut brm = RawMaterial::new("maize germ".to_string(), 10.);
    // brm.local_log();
    // brm.price(80);

    // let mut borm = RawMaterial::new("soya".to_string(), 150.);
    // borm.local_log();
    // borm.price(70);

    // let mut transo = OutTransaction::new(c);

    // transo.add(brm);
    // transo.add(borm);
    // transo.add(fresh);
    // // transo.balance_books();
    // transo.settle_bill(1000.);
    // transo.log(path7);

    // // production
    // let mut prod = Production::new(pr2);
    // prod.add_rawmat(brm);
    // prod.add_rawmat(borm);
    // prod.add_rawmat(fresh);

    // dbg!(&prod);

    // // Dailyyield
    // let dy = DailyYield::new(pr2, 1000.);

    // let now = Local::now(); //2022-01-15T12:19:03.123970801+03:00
    // let s = now.format("%d-%m-%Y %H:%M:%S").to_string();
    // println!("{:?}",&s);
    // let mut dl = fetch_ext_debt_holders()?;
    // let mut tl = fetch_transaction_in_log()?;
    // dl[0].settle_debt(300, &mut tl);
    // dbg!(&tl);

    Ok(())
}