pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.trim().split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();

  let mut v = vec![0; n];
  for i in 1..=n {
    v[i - 1] = i;
  }

  loop {
    let mut s = String::new();
    if std::io::stdin().read_line(&mut s).is_err() {
      break;
    }
    if s.trim().is_empty() {
      break;
    }
    let mut s_l = s.trim().split_whitespace();
    let cmd = s_l.next().unwrap().parse::<usize>().unwrap();
    if cmd == 1 {
      let p = s_l.next().unwrap().parse::<usize>().unwrap();
      let q = s_l.next().unwrap().parse::<usize>().unwrap();
      let before_group = v[p - 1];
      let after_group = v[q - 1];
      if before_group != after_group {
        for i in 0..n {
          if v[i] == before_group {
            v[i] = after_group
          }
        }
      }
    } else if cmd == 2 {
      let p = s_l.next().unwrap().parse::<usize>().unwrap();
      let q = s_l.next().unwrap().parse::<usize>().unwrap();
      let after_group = v[q - 1];
      v[p - 1] = after_group;
    } else if cmd == 3 {
      let p = s_l.next().unwrap().parse::<usize>().unwrap();
      let group = v[p - 1];
      let mut sum = 0;
      let mut len = 0;
      for i in 0..n {
        if v[i] == group {
          len += 1;
          sum += i + 1
        }
      }
      println!("{} {}", len, sum)
    }
  }
}
