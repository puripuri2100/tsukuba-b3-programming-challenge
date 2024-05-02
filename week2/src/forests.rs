pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.trim().split_whitespace();
  let p_size = s_l.next().unwrap().parse::<usize>().unwrap();
  let t_size = s_l.next().unwrap().parse::<usize>().unwrap();

  let mut v = vec![vec![false; t_size]; p_size];
  loop {
    let mut s = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut s) {
      if s.trim().is_empty() {
        break;
      }
      let mut s_l = s.trim().split_whitespace();
      let p = s_l.next().unwrap().parse::<usize>().unwrap();
      let t = s_l.next().unwrap().parse::<usize>().unwrap();
      v[p - 1][t - 1] = true;
    } else {
      break;
    }
  }
  v.sort_by(|a, b| a.cmp(&b));
  v.dedup();
  println!("{}", v.len());
}
