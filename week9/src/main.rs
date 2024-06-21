use clap::Parser;

mod ornaments;
mod platforme;
mod polygon;
mod gps;

#[derive(Parser, Debug)]
struct Arg {
  probrem: String,
}

fn main() {
  let args = Arg::parse();
  if args.probrem == "platforme" {
    platforme::main()
  } else if args.probrem == "polygon" {
    polygon::main()
  } else if args.probrem == "ornaments" {
    ornaments::main()
  } else if args.probrem == "gps" {
    gps::main()
  } else {
    println!("Unsupported");
  }
}
