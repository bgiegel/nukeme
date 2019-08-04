use std::env;
use std::process;
// light speed in m/s
const C:f64 = 299_792_458_f64;

// Number of joules to get the equivalent of 1 kt of TNT
const KILOTON_OF_TNT:f64 = 4_184_000_000_000_f64;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mass:f64 = args.get(1).unwrap_or_else(|| {
        eprintln!("Please provide the mass in kg you want to explode.");
        process::exit(1);
    }).parse().unwrap_or_else(|_err|{
        eprintln!("The mass you provided is not a number.");
        process::exit(1);
    });

    // energy in joules
    let energy:f64 = mass * C * C;

    let tnt_equivalent = energy / KILOTON_OF_TNT;
    let little_boy_equivalent = tnt_equivalent / 15_f64;
    let tsar_bomba_equivalent = tnt_equivalent / 1000_f64 / 50_f64;

    println!("energy in joules of {} kg of any element {:}", mass ,energy);
    println!("{:.2} kiloton of TNT",tnt_equivalent);
    println!("{:.2} megaton of TNT", tnt_equivalent / 1000_f64);
    println!("{:.2} times the nuke dropped on hiroshima (little boy)", little_boy_equivalent);
    println!("{:.2} times the TSAR Bomba (most powerful nuke ever tested)", tsar_bomba_equivalent);

}
