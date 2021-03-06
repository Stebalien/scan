#![feature(phase)]

extern crate scan;
#[phase(plugin)] extern crate scan_mac;

fn print(s: &str) {
    let mut stdout = std::io::stdio::stdout();
    stdout.write_str(s).ok();
}

fn main() {
    print("Please enter your name: ");

    let name = readln!();
    let year;
    let place;
    loop {
        print("Please enter your year and place of birth: ");
        let (maybe_year, maybe_place) = scanln!(" {u32} {s} ");
        match (maybe_year, maybe_place) {
            (Some(y), Some(s)) => {
                year = y;
                place = s;
                break;
            },
            _ => { }
        }
    }

    println!("{} {} {}", name, year + 1, place);
}
