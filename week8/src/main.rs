use clap::Parser;

mod food;
mod perfect;
mod prsteni;
mod reseto;
mod zero;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "prsteni" {
    prsteni::main()
  } else if args.probrem == "reseto" {
    reseto::main()
  } else if args.probrem == "perfect" {
    perfect::main()
  } else if args.probrem == "food" {
    food::main()
  } else if args.probrem == "zero" {
    zero::main()
  } else {
    println!("Unsupported");
  }
}
