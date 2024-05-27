use clap::Parser;

mod coast;
mod gold;
mod internet;
mod muddyhike;

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
  } else {
    println!("Unsupported");
  }
}
