/*
 * TODO use BufReader??
 * TODO nice error handling
 */

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
    .expect("OOPSIE WOOPSIE!! Uwu We made a fucky wucky!! A wittle fucko boingo! The code monkeys at our headquarters are working VEWY HAWD to fix this!");

    let result:String = content.chars()
    .map(|x| match x {
        'l' => 'w',
        'L' => 'W',
        'r' => 'w',
        'R' => 'W',
        _  => x
    }).collect();

println!("{}", result)
}
