pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.trim().split_whitespace();
  // 楽器数
  let n = s_l.next().unwrap().parse::<usize>().unwrap();

  // 楽器ごとの演奏できる音符のリスト
  let mut borads = vec![];
  for _ in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.trim().split_whitespace();
    s_l.next();
    let mut v2 = vec![];
    for c in s_l {
      v2.push(c.parse::<usize>().unwrap())
    }
    borads.push(v2)
  }
  // 演奏する音
  let mut lst = vec![];
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  for c in s.trim().split_whitespace() {
    lst.push(c.parse::<usize>().unwrap())
  }

  let mut a = 0_usize;
  let mut now_borads = borads.clone();
  for v in lst.iter() {
    let new_borads = now_borads
      .iter()
      .filter(|l| l.iter().any(|t| t == v))
      .cloned()
      .collect::<Vec<_>>();
    if new_borads.is_empty() {
      let new_borads = borads
        .clone()
        .iter()
        .filter(|l| l.iter().any(|t| t == v))
        .cloned()
        .collect::<Vec<_>>();
      now_borads = new_borads;
      a += 1;
    } else {
      now_borads = new_borads;
    }
  }

  println!("{a}");
}
