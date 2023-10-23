use std::collections::HashMap;

fn main(){
   let mut  x = HashMap::new();
   x.insert(1, 10);

   *x.entry(2).or_insert(20) =55;

   
   println!("The data are {:?}", x);

//    match y{
//     Some(val) => println!("The val is {:?}", val),
//     None => println!("Nothing")
//    }

}