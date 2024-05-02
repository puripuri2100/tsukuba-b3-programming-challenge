use clap::Parser;
mod baloni;
mod div;
mod forests;
mod sort;
mod stock;
mod union;
mod mali;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "baloni" {
    baloni::main()
  } else if args.probrem == "sort" {
    sort::main()
  } else if args.probrem == "div" {
    div::main()
  } else if args.probrem == "forests" {
    forests::main()
  } else if args.probrem == "union" {
    union::main()
  } else if args.probrem == "stock" {
    stock::main()
  } else if args.probrem == "mali" {
    mali::main()
  } else {
    println!("Unsupported");
  }
}
