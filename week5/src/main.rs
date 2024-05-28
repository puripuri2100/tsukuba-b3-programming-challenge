use clap::Parser;

mod coast;
mod conservation;
mod gold;
mod internet;
mod muddyhike;
mod nature;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "internet" {
    internet::main()
  } else if args.probrem == "golds" {
    gold::main()
  } else if args.probrem == "coast" {
    coast::main()
  } else if args.probrem == "hike" {
    muddyhike::main()
  } else if args.probrem == "nature" {
    nature::main()
  } else if args.probrem == "conservation" {
    conservation::main()
  } else {
    println!("Unsupported");
  }
}
