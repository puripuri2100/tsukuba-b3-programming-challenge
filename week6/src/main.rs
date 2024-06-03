use clap::Parser;

mod arbitrage;
mod crusher;
mod grid;
mod trails;

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
  } else if args.probrem == "trails" {
    trails::main()
  } else {
    println!("Unsupported");
  }
}
