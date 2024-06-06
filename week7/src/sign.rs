pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  for _ in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let len = s_l.next().unwrap().parse::<usize>().unwrap();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    let mut s_lst = Vec::new();
    for _ in 0..n {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      s_lst.push(s.trim().to_string());
    }
    let mut ans = len;
    let mut s_iter = s_lst.iter().peekable();
    while let Some(s1) = s_iter.next() {
      if let Some(s2) = s_iter.peek() {
        for i in (0..=len).rev() {
          let sub1 = &s1[len - i..len];
          let sub2 = &s2[0..i];
          if sub1 == sub2 {
            ans += len - i;
            break;
          }
        }
      } else {
        break;
      }
    }
    println!("{ans}");
  }
}
