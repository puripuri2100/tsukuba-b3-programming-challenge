use clap::Parser;

mod nikora;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "nikora" {
    nikora::main();
  } else {
    println!("Unsupported");
  }
}
