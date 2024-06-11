pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut n_lst = s.split_whitespace().map(|s| s.parse::<usize>().unwrap());
  let m = n_lst.next().unwrap();
  for n in n_lst {
    let gcd_i = gcd(m, n);
    println!("{}/{}", m / gcd_i, n / gcd_i);
  }
}

fn gcd(mut m: usize, mut n: usize) -> usize {
  if m < n {
    gcd(n, m)
  } else {
    let mut r: usize;
    loop {
      r = m % n;
      if r == 0 {
        return n;
      }
      m = n;
      n = r;
    }
  }
}
