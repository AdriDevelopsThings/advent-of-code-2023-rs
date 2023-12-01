use std::{env, fs::create_dir, path::PathBuf, time::Instant};

use clap::Parser;
use dotenv::dotenv;

mod loader;

mod one;

type AdventOfCodeSolveFunctions =
    &'static [(&'static str, fn(String) -> String, fn(String) -> String)];

static ADVENTOFCODE_SOLVE_FUNCTIONS: AdventOfCodeSolveFunctions =
    &[("1", one::solve_first, one::solve_second)];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    session_token: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    puzzle_part: u8,
    #[arg(value_name = "puzzle")]
    puzzle: String,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let cache_path = PathBuf::from(".cache");
    if !cache_path.exists() {
        create_dir(cache_path.clone()).unwrap();
    }

    let session_token = args.session_token.unwrap_or_else(|| {
        env::var("SESSION_TOKEN").expect("No session token argument or environment variable given")
    });
    let session = loader::Session { session_token };
    let cache = loader::Cache::new(cache_path, session);
    for function in ADVENTOFCODE_SOLVE_FUNCTIONS.iter() {
        if function.0 == args.puzzle {
            println!("Challenge {} ({}/2)", function.0, args.puzzle_part);
            let input = cache.get_puzzle_input(function.0);
            let now = Instant::now();
            let output = match args.puzzle_part {
                1 => function.1,
                2 => function.2,
                _ => function.1,
            }(input);
            println!("Took {:.2?}ms", now.elapsed().as_millis());
            println!("Answer: {output}");
        }
    }
}
