mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = &args[1];
    match day.as_str() {
        "01" => day01::main("input/day_01.txt"),
        "02" => day02::main("input/day_02.txt"),
        _ => println!("Unknown day"),
    }
}
