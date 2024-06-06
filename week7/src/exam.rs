pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let correct_n = s.trim().parse::<usize>().unwrap();
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut clst1 = s.trim().chars();
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut clst2 = s.trim().chars();
  let mut len = 0_usize;
  let mut diff_n = 0_usize;
  while let (Some(c1), Some(c2)) = (clst1.next(), clst2.next()) {
    len += 1;
    if c1 != c2 {
      diff_n += 1
    }
  }
  // 7 : 3, 6 -> 9
  // 7 : 3, 7 -> 10
  // 7 : 3, 8 -> 9
  // 7 : 3, 9 -> 8
  // 7 : 3, 10 -> 7
  if correct_n < len - diff_n {
    println!("{}", correct_n + diff_n)
  } else {
    // 0 : 5, 3 -> 2
    println!("{}", diff_n - (correct_n - (len - diff_n)) + (len - diff_n))
  }
}
