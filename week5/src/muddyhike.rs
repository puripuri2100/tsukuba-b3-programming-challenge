pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let w = s_l.next().unwrap().parse::<usize>().unwrap();
  let h = s_l.next().unwrap().parse::<usize>().unwrap();

  // 記録されている深さ
  let mut map = vec![vec![0; w]; h];
  // そこに到達できる最小の最大値を記録する
  let mut depth_map = vec![vec![usize::MAX; w]; h];
  // 最大値の変更があった座標を記録して周囲のアップデートを行う
  let mut queue = std::collections::VecDeque::new();
  for i in 0..w {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for (j, s) in s.split_whitespace().enumerate() {
      let v = s.parse::<usize>().unwrap();
      map[j][i] = v;
      if i == 0 {
        depth_map[i][j] = v;
        queue.push_back((i, j));
      }
    }
  }


  while let Some((i, j)) = queue.pop_front() {
    if i != 0 {
      let value =
    }
  }

  let mut ans_l = &depth_map[h - 1];
  let ans = ans_l.iter().max().unwrap();

  println!("{}", ans);
}

