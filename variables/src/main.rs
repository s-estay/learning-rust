
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5; // declare new variable
    println!("the value of x is: {x}");
    {
        let x = x * 2; // shadow variable x inside inner scope
        println!("the value of x is: {x}");
    }
    let x = x + 1; // shadow variable x
    println!("the value of x is: {x}");

    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type
}
