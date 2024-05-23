use clap::Parser;

mod internet;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "internet" {
    internet::main()
  } else {
    println!("Unsupported");
  }
}
