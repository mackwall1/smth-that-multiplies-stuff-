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

    let ax:f64 = ax.trim().parse().expect("X IS NOT A NUMBER!");
    let ay:f64 = ay.trim().parse().expect("Y IS NOT A NUMBER!");
    let az:f64 = az.trim().parse().expect("Z IS NOT A NUMBER!");

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

    let bx:f64 = bx.trim().parse().expect("2ND X IS NOT A NUMBER!");
    let by:f64 = by.trim().parse().expect("2ND Y IS NOT A NUMBER!");
    let bz:f64 = bz.trim().parse().expect("2ND Z IS NOT A NUMBER!");

    println!("now we multiply!");

    let xa:f64 = ax * bx;
    let xb:f64 = ax * by;
    let xc:f64 = ax * bz;

    let ya:f64 = ay * bx;
    let yb:f64 = ay * by;
    let yc:f64 = ay * bz;

    let za:f64 = az * bx;
    let zb:f64 = az * by;
    let zc:f64 = az * bz;

    println!("alright! here are the results!:3 :");
    park_timeout(Duration::new(1, 0));
    println!("Calculating.");
    park_timeout(Duration::new(1, 0));
    println!("Calculating..");
    park_timeout(Duration::new(1, 0));
    println!("Calculating...");
    park_timeout(Duration::new(1, 0));
    println!("x= {xa}, {xb}, {xc}");
    park_timeout(Duration::new(1, 0));
    println!("Calculating.");
    park_timeout(Duration::new(1, 0));
    println!("Calculating..");
    park_timeout(Duration::new(1, 0));
    println!("Calculating...");
    park_timeout(Duration::new(1, 0));
    println!("y= {ya}, {yb}, {yc}");
    park_timeout(Duration::new(1, 0));
    println!("Calculating.");
    park_timeout(Duration::new(1, 0));
    println!("Calculating..");
    park_timeout(Duration::new(1, 0));
    println!("Calculating...");
    park_timeout(Duration::new(1, 0));
    println!("z= {za}, {zb}, {zc}");
    park_timeout(Duration::new(1, 0));
    println!("it looks like this! :3 :
     x:{xa}                  y:{ya}                   z:{za}
       {xb}                    {yb}                     {zb}
       {xc}                   {yc}                     {zc}
    ");





}