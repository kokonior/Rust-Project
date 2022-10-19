use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'u', long = "url")]
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let resp = reqwest::blocking::get(args.url)?
        .text()?;
    println!("{}", resp);
    Ok(())
}
