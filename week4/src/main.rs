use clap::Parser;

mod bag;
mod dolls;
mod keyborads;
mod nikora;
mod orders;
mod sort;
mod spatulas;
mod tight_words;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "nikora" {
    nikora::main();
  } else if args.probrem == "tight_words" {
    tight_words::main();
  } else if args.probrem == "keyboards" {
    keyborads::main();
  } else if args.probrem == "orders" {
    orders::main();
  } else if args.probrem == "sort" {
    sort::main()
  } else if args.probrem == "dolls" {
    dolls::main();
  } else if args.probrem == "bag" {
    bag::main()
  } else if args.probrem == "spatulas" {
    spatulas::main()
  } else {
    println!("Unsupported");
  }
}
