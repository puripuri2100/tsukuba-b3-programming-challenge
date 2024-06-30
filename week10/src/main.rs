use clap::Parser;

mod bee;
mod bus;
mod gears;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "bus" {
    bus::main()
  } else if args.probrem == "bee" {
    bee::main()
  } else if args.probrem == "gears" {
    gears::main()
  } else {
    println!("Unsupported");
  }
}
