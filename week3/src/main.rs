use clap::Parser;

mod acm;
mod closest_sums;
mod cudoviste;
mod lektira;
mod peg;
mod firefly;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "cudoviste" {
    cudoviste::main();
  } else if args.probrem == "closest_sums" {
    closest_sums::main()
  } else if args.probrem == "peg" {
    peg::main()
  } else if args.probrem == "lektira" {
    lektira::main()
  } else if args.probrem == "firefly" {
    firefly::main()
  } else if args.probrem == "acm" {
    acm::main()
  } else {
    println!("Unsupported");
  }
}
