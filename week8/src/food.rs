pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let mut s2_l = s_l.next().unwrap().split('.');
  let p_t = s2_l.next().unwrap().parse::<usize>().unwrap() * 100
    + s2_l.next().unwrap().parse::<usize>().unwrap();
  let mut s2_l = s_l.next().unwrap().split('.');
  let p_1 = s2_l.next().unwrap().parse::<usize>().unwrap() * 100
    + s2_l.next().unwrap().parse::<usize>().unwrap();
  let mut s2_l = s_l.next().unwrap().split('.');
  let p_2 = s2_l.next().unwrap().parse::<usize>().unwrap() * 100
    + s2_l.next().unwrap().parse::<usize>().unwrap();

  let mut is_ok = false;
  for i in 0..=p_t / p_1 {
    if (p_t - (p_1 * i)) % p_2 == 0 {
      let j = (p_t - (p_1 * i)) / p_2;
      println!("{i} {j}");
      is_ok = true;
    }
  }
  if !is_ok {
    println!("none")
  }
}
