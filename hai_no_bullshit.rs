use std::thread::park_timeout;
use std::time::Duration;
use std::io;
use rodio::{MixerDeviceSink, Player};
use rodio::source::{Amplify, SineWave, Source, TakeDuration};



fn input(unit: &mut String) {
    io::stdin()
        .read_line( {unit})
        .expect("what?");
}

fn play_audio(frequency: f32, duration: f64) {
    let mut handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream!!!");
    let player = Player::connect_new(&handle.mixer());
    let source = SineWave::new({frequency} as f32).take_duration(Duration::from_secs_f64(duration)).amplify(0.25);
    player.append(source);
    player.sleep_until_end();
    handle.log_on_drop(false)
}

fn main() {

    println!("input some numbers!");
    let mut ax = String::new();
    let mut  ay = String::new();
    let mut az= String::new();


    println!("input for x?");

    input(&mut ax);

    println!("input for y?");

    input(&mut ay);

    println!("input for z?");

    input(&mut az);

    let ax:f64 = ax.trim().parse().expect("X IS NOT A NUMBER!");
    let ay:f64 = ay.trim().parse().expect("Y IS NOT A NUMBER!");
    let az:f64 = az.trim().parse().expect("Z IS NOT A NUMBER!");

    println!("now that that's done, let's add your second set of numbers!");

    let mut bx= String::new();
    let mut by = String::new();
    let mut bz = String::new();

    println!("input for 2nd x?");

    input(&mut bx);

    println!("input for 2nd y?");

    input(&mut by);

    println!("input for 2nd z?");

    input(&mut bz);

    let bx:f64 = bx.trim().parse().expect("2ND X IS NOT A NUMBER!");
    let by:f64 = by.trim().parse().expect("2ND Y IS NOT A NUMBER!");
    let bz:f64 = bz.trim().parse().expect("2ND Z IS NOT A NUMBER!");

    println!("now we multiply!");

    let a:[f64; 3] = [ax * bx, ax * by, ax * bz];

    let b:[f64; 3] = [ay * bx, ay * by, ay * bz];

    let c:[f64; 3] = [az * bx, az * by, az * bz];

    println!("alright! here are the results!:3 :");
    play_audio(440.0, 0.10);
    println!("x= {}, {}, {}" ,a[0], a[1], a[2]);
    play_audio(480.0, 0.10);
    println!("y= {}, {}, {}" ,b[0], b[1], b[2]);
    play_audio(500.0, 0.10);
    println!("z= {}, {}, {}" ,c[0], c[1], c[2]);
    park_timeout(Duration::new(1, 0));
    println!("it looks like this! :3 :
     \x1b[91mx\x1b[0m:{}                 \x1b[92my\x1b[0m:{}                  \x1b[93mz\x1b[0m:{}
     {}                   {}                   {}
     {}                   {}                   {}
    " ,a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]);

    if ax == 0.0 {
        println!("1st x was zero, so values multiplied by it is zero :c");
        play_audio(300.0, 0.10);
    }
    if ay == 0.0 {
        println!("1st y was zero, so values multiplied by it is zero :c");
        play_audio(300.0, 0.10);
    }
    if az == 0.0 {
        println!("1st z was zero, so values multiplied by it is zero :c");
        play_audio(300.0, 0.10);
    }
    if bx == 0.0 {
        println!("the second x was zero, so values multiplied by it is zero :c");
        play_audio(350.0, 0.10);
    }
    if by == 0.0 {
        println!("the second y was zero, so values multiplied by it is zero :c");
        play_audio(350.0, 0.10);
    }
    if bz == 0.0 {
        println!("the second z was zero, so values multiplied by it is zero :c");
        play_audio(350.0, 0.10);
    }
}