pub fn main() {
  loop {
    let mut s = String::new();

    std::io::stdin().read_line(&mut s).unwrap();
    if s.trim().is_empty() {
      break;
    }
    let n = s.trim().parse::<usize>().unwrap();
    let mut v = 1_usize;
    for i in 2..=(n as f64).sqrt() as usize {
      if n % i == 0 {
        v += i;
        if n / i != i {
          v += n / i;
        }
      }
    }
    if n == v {
      println!("{n} perfect");
    } else if n.abs_diff(v) <= 2 {
      println!("{n} almost perfect")
    } else {
      println!("{n} not perfect")
    }
  }
}
