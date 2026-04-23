use std::{io, thread};
use std::time::Duration;
use std::thread::sleep;

fn eep(time: u64) {
    thread::sleep(Duration::from_secs(time))
}


fn inp(number: &mut String) {
    io::stdin().read_line(number).expect("failed to read line!");
}

fn main() {
    println!("welcome to HAI v2.0! it still sucks!!!");
    println!("now, if you've been 'using' or inspecting HAI before v2.0, you know what this does.");
    println!("if not, welp, in versions before 2.0, this was only multiplication, now it's multiplication, addition blah blah blah...");
    println!("all these happen between 6 numbers, 1st 3 numbers do math stuff with the other 3, like this:");
    println!("x * x1, x * y1, x * z1... blah blah blah, but this time it's with the other math stuff, so have fun! :3");

    eep(1);

    println!("alright, enter a NUMBER for x!!!");

    let mut x = String::new();                                                  //this is x!
    inp(&mut x);                                                                //this is where you *hopefully* enter a number
    let rx:f64 = x.trim().parse().expect("it's supposed to be a number 3:");    //this where your number gets converted into a... number
                                                                                //... or where you get yelled at for not entering a number
    eep(1);

    println!("now do it for y...");

    let mut y = String::new();
    inp(&mut y);
    let ry:f64 = y.trim().parse().expect("dude, a NUMBER, please!");

    println!("now for z...");

    let mut z = String::new();
    inp(&mut z);
    let rz:f64 = z.trim().parse().expect("you're never gonna learn how to enter a number, will you?");

    if (rx, ry, rz) == (0.0, 0.0, 0.0) {
        println!("just so you know, x, y and z are all zero's!");
    }

    eep(1);

    println!("if you're here, that means you entered numbers for x, y and z! yay!!!!! :3");

    eep(1);

    println!("you know how this goes, enter a number for the second x...");

    let mut x1 = String::new();
    inp(&mut x1);
    let rx1:f64 = x1.trim().parse().expect("you have to be trolling .w.");

    println!("alright, now the 2nd y...");

    let mut y1 = String::new();
    inp(&mut y1);
    let ry1:f64 = y1.trim().parse().expect("yup, you're trolling. x_x");

    println!("now do it for z, the 2nd one of course");

    let mut z1 = String::new();
    inp(&mut z1);
    let rz1:f64 = z1.trim().parse().expect("do you know what numbers are?");

    eep(1);

    println!("ok, now you will enter a mathematical operator, such as '*, +, -, /' and not anything else, please. .w.");

    let mut opr = String::new();
    inp(&mut opr);
    let op = opr.trim();

    match op {
        "*" => { let xtxyz:[f64;3] = [rx * rx1, rx * ry1, rx * rz1];
            let ytxyz:[f64;3] = [ry * rx1, ry * ry1, ry * rz1];
            let ztxyz:[f64;3] = [rz * rx1, rz * ry1, rz * rz1];
            println!("x: {} y: {} z:{}
   {}    {}   {}
   {}    {}   {}" ,xtxyz[0], xtxyz[1], xtxyz[2], ytxyz[0], ytxyz[1], ytxyz[2], ztxyz[0], ztxyz[1], ztxyz[2]); eep(1);}
        "/" => { let xtxyz:[f64;3] = [rx / rx1, rx / ry1, rx / rz1];
            let ytxyz:[f64;3] = [ry / rx1, ry / ry1, ry / rz1];
            let ztxyz:[f64;3] = [rz / rx1, rz / ry1, rz / rz1];
            println!("x: {} y: {} z:{}
   {}    {}   {}
   {}    {}   {}" ,xtxyz[0], xtxyz[1], xtxyz[2], ytxyz[0], ytxyz[1], ytxyz[2], ztxyz[0], ztxyz[1], ztxyz[2]); eep(1);}
        "+" => { let xtxyz:[f64;3] = [rx + rx1, rx + ry1, rx + rz1];
            let ytxyz:[f64;3] = [ry + rx1, ry + ry1, ry + rz1];
            let ztxyz:[f64;3] = [rz + rx1, rz + ry1, rz + rz1];
            println!("x: {} y: {} z:{}
   {}    {}   {}
   {}    {}   {}" ,xtxyz[0], xtxyz[1], xtxyz[2], ytxyz[0], ytxyz[1], ytxyz[2], ztxyz[0], ztxyz[1], ztxyz[2]); eep(1);}
        "-" => { let xtxyz:[f64;3] = [rx - rx1, rx - ry1, rx - rz1];
            let ytxyz:[f64;3] = [ry - rx1, ry - ry1, ry - rz1];
            let ztxyz:[f64;3] = [rz - rx1, rz - ry1, rz - rz1];
            println!("x: {} y: {} z:{}
   {}    {}   {}
   {}    {}   {}" ,xtxyz[0], xtxyz[1], xtxyz[2], ytxyz[0], ytxyz[1], ytxyz[2], ztxyz[0], ztxyz[1], ztxyz[2]); eep(1);}
        _ => {println!("I TOLD YOU TO ENTER A MATHEMATICAL OPERATOR, WHY ARE YOU DOING THIS???"); eep(3); panic!("you deserve it.");}
    }

}
