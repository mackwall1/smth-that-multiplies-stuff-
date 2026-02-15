use std::thread::park_timeout;
use std::time::Duration;
use std::io;

fn main() {
    let mut x = 5;
    let x = 5;
    println!("the value of x is {x}! :3");
    {
        let x = 77;
        println!("wha- ughh.. i just told you! the value of x is {x}- wha- what? IT CHANGED?");
    }
    println!("\n\x1b[91mwhat... it can't be!\x1b[0m");
    park_timeout(Duration::new(2, 0));
    println!("i'm going to try again...");
    park_timeout(Duration::new(1, 0));
    println!("the value of \x1b[91mx\x1b[0m is hopefully... {x}... \x1b[92mIT WORKS!\x1b[0m");
    println!("let me try something...");

    println!("input some numbers!");
    let mut ax:String = String::new();
    let mut  ay:String = String::new();
    let mut az:String = String::new();

    println!("DECIMAL NUMBERS AREN'T SUPPORTED YET! i'll add support for them later! :3");

    println!("input for x?");

    io::stdin()
            .read_line(&mut ax)
            .expect("what?");

    println!("input for y?");

    io::stdin()
        .read_line(&mut ay)
        .expect("what?");

    println!("input for z?");

    io::stdin()
         .read_line(&mut az)
         .expect("what?");

    let ax:i32 = ax.trim().parse().expect("X IS NOT A NUMBER!");
    let ay:i32 = ay.trim().parse().expect("Y IS NOT A NUMBER!");
    let az:i32 = az.trim().parse().expect("Z IS NOT A NUMBER!");

    println!("now that that's done, let's add your second set of numbers!");

    let mut bx:String = String::new();
    let mut by:String = String::new();
    let mut bz:String = String::new();

    println!("input for 2nd x?");

    io::stdin()
       .read_line(&mut bx)
       .expect("what?");

    println!("input for 2nd y?");

    io::stdin()
       .read_line(&mut by)
       .expect("what?");

    println!("input for 2nd z?");

    io::stdin()
        .read_line(&mut bz)
        .expect("what?");

    let bx:i32 = bx.trim().parse().expect("2ND X IS NOT A NUMBER!");
    let by:i32 = by.trim().parse().expect("2ND Y IS NOT A NUMBER!");
    let bz:i32 = bz.trim().parse().expect("2ND Z IS NOT A NUMBER!");

    println!("now we multiply!");

    let xa = ax * bx;
    let xb = ax * by;
    let xc = ax * bz;

    let ya:i32 = ay * bx;
    let yb:i32 = ay * by;
    let yc:i32 = ay * bz;

    let za:i32 = az * bx;
    let zb:i32 = az * by;
    let zc:i32 = az * bz;

    println!("alright! here are the results!:3 :");
    println!("x= {xa}, {xb}, {xc}");
    println!("y= {ya}, {yb}, {yc}");
    println!("z= {za}, {zb}, {zc}");





}