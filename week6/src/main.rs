use clap::Parser;

mod grid;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "grid" {
    grid::main()
  } else {
    println!("Unsupported");
  }
}
