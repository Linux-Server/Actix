fn main(){
    let x = Some(String::from("sachin"));
    match &x{
        Some(val) => println!("the valu is {:?}", val),
        None => println!("Nothing found")
    }

    println!("The result is {:?}", x);
}