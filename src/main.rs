/*
 * TODO use BufReader??
 * TODO nice error handling
 * TODO get a nice pair of programming socks to boost programming skill up to 4x
 */

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("OOPSIE WOOPSIE!! UwU We made a fucky wucky!! A wittle fucko boingo! The code monkeys at our headquarters are working VEWY HAWD to fix this! \n error: {}", error); }
    };

    let result: String = content.chars()
    .map(|x| match x {
        'l' => 'w',
        'L' => 'W',
        'r' => 'w',
        'R' => 'W',
        _  => x
    }).collect();

println!("{}", result)
}
