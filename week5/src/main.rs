use clap::Parser;

mod coast;
mod gold;
mod internet;

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
  } else {
    println!("Unsupported");
  }
}
