
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn can_hold(&self, other:&Rectangle)-> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect_one = Rectangle{width:10,height:20};
    let other_rect = Rectangle{width:33, height:3};

    let result = rect_one.can_hold(&other_rect);

    println!("The result is {:?}", result);

}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn large_can_hold_small(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));

    }
    #[test]
    fn smaller_can(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
        assert!(false,"Im failed test : {:?}", "sachin")


    }
}