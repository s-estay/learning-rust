
const TWO_HOURS_IN_SECONDS: u32 = 2*60*60;

fn main() {
    let mut x = 5; 
    println!("the value of x is: {x}");
    if x < TWO_HOURS_IN_SECONDS {
      x = 6;
      println!("the value of x is: {x}");
    }
}
