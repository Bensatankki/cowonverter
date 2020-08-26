use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
    .expect("file could not be opened");

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