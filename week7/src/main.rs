use clap::Parser;

mod exam;
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
  } else {
    println!("Unsupported");
  }
}
