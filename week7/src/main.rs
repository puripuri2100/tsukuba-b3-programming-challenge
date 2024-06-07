use clap::Parser;

mod buzzwords;
mod exam;
mod knight;
mod power;
mod sign;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "sign" {
    sign::main()
  } else if args.probrem == "exam" {
    exam::main()
  } else if args.probrem == "knight" {
    knight::main()
  } else if args.probrem == "power" {
    power::main()
  } else if args.probrem == "buzzwords" {
    buzzwords::main()
  } else {
    println!("Unsupported");
  }
}
