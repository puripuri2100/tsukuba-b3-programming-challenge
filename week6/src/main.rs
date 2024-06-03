use clap::Parser;

mod arbitrage;
mod crusher;
mod grid;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "grid" {
    grid::main()
  } else if args.probrem == "crusher" {
    crusher::main()
  } else if args.probrem == "arbitrage" {
    arbitrage::main()
  } else {
    println!("Unsupported");
  }
}
