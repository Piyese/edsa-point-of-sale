use chrono::Local;
use edsa_pos::pipeline::{errors::PosError, people::{Employee, Person, Sex}, inventory::{Product, FinishedProd, PackagedProd}, accounts::TransactionIn};

fn main()->Result <(), PosError>{
    // // create supplier,customer,employer
    let _e = Employee::new("Rapper".to_string(), Sex::Male, true, "0101019181".to_string());
    let s = Person::new( "Thread".to_string(), "038271928".to_string());
    let c = Person::new( "Rayon".to_string(), "3823344242".to_string());

    // product to be produced
    let pr = Product::new(String::from("Flour") );
    let pr2 = Product::new(String::from("Kuku Feeds") );

    //Finished Products(unpacked)
    let mut fd = FinishedProd::new(pr.clone()) ;
    let mut fd2 = FinishedProd::new(pr2.clone()) ;

    fd.add(500.);
    dbg!(&fd);
    fd2.add(740.);
    dbg!(&fd2);

    // packaged products 
    let mut pembe = PackagedProd::new(pr.clone());
    pembe.specify_pkg("Pembe small".to_string() );
    pembe.specify_qty(2.);
    pembe.add_packs(25);


    let mut chick_mash = PackagedProd::new(pr2.clone());
    chick_mash.specify_pkg("Chick Mash".to_string());
    chick_mash.specify_qty(0.5);
    chick_mash.add_packs(87);


    // buyer c walks in
    let mut trans = TransactionIn::new(c);
    // items to buy
    let mut buy_item = PackagedProd::new(pr.clone());
    buy_item.specify_pkg("Pembe small".to_string());
    buy_item.specify_qty(2.);
    buy_item.specify_cost(65.);
    buy_item.add_packs(13);
  
    let mut buy_item2 = PackagedProd::new(pr2.clone());
    buy_item2.specify_pkg("Chick Mash".to_string());
    buy_item2.specify_cost(65.);
    buy_item2.specify_qty(0.5);
    buy_item2.add_packs(16);

    trans.add(buy_item);  
    dbg!(&trans);
    trans.add(buy_item2);

    dbg!(&trans);


    // let now = Local::now(); //2022-01-15T12:19:03.123970801+03:00
    // let s = now.format("%d-%m-%Y %H:%M:%S").to_string();
    // println!("{:?}",&s);

    // let mut dl = fetch_ext_debt_holders()?;
    // let mut tl = fetch_transaction_in_log()?;
    
    // dl[0].settle_debt(300, &mut tl);
    // dbg!(&tl);



    Ok(())
}