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
  #[allow(clippy::needless_range_loop)]
  for i in 0..w {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for (j, s) in s.split_whitespace().enumerate() {
      let v = s.parse::<usize>().unwrap();
      map[j][i] = v;
      if j == 0 {
        depth_map[j][i] = v;
        queue.push_back((i, j));
      }
    }
  }

  while let Some((i, j)) = queue.pop_front() {
    if i != 0 {
      let value = map[j][i - 1];
      let depth1 = depth_map[j][i - 1];
      let depth2 = depth_map[j][i];
      let new_depth = value.max(depth2);
      if depth1 > new_depth {
        depth_map[j][i - 1] = new_depth;
        queue.push_back((i - 1, j))
      }
    }
    if i != w - 1 {
      let value = map[j][i + 1];
      let depth1 = depth_map[j][i + 1];
      let depth2 = depth_map[j][i];
      let new_depth = value.max(depth2);
      if depth1 > new_depth {
        depth_map[j][i + 1] = new_depth;
        queue.push_back((i + 1, j))
      }
    }
    if j != 0 {
      let value = map[j - 1][i];
      let depth1 = depth_map[j - 1][i];
      let depth2 = depth_map[j][i];
      let new_depth = value.max(depth2);
      if depth1 > new_depth {
        depth_map[j - 1][i] = new_depth;
        queue.push_back((i, j - 1))
      }
    }
    if j != h - 1 {
      let value = map[j + 1][i];
      let depth1 = depth_map[j + 1][i];
      let depth2 = depth_map[j][i];
      let new_depth = value.max(depth2);
      if depth1 > new_depth {
        depth_map[j + 1][i] = new_depth;
        queue.push_back((i, j + 1))
      }
    }
  }

  let ans_l = &depth_map[h - 1];
  let ans = ans_l.iter().min().unwrap();

  println!("{}", ans);
}
