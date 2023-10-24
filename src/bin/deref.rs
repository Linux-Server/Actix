use std::collections::HashMap;

fn main(){
   let mut  x = HashMap::new();
   let mut  y = HashMap::new();

//    x.insert(1, 10);

   let b: &mut Team = x.entry(2).or_insert(Team{yes:10,no:Nos{not:88}});
   let c= y.entry(2).or_insert(22);

    (*b).yes += 100;
//    (*b).no.not += 12;
   
   *c +=99;
   println!("The data are {:?}", x);
   println!("The data are {:?}", y);

//    match y{
//     Some(val) => println!("The val is {:?}", val),
//     None => println!("Nothing")
//    }

}


#[derive(Debug)]
struct Team{
    yes:i32,
    no:Nos
}

#[derive(Debug)]

struct Nos{
   not: i32
}