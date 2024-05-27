pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();

  // マスの種類と、トラップに隣接しているかどうか
  let mut water_map = vec![vec![false; m]; n];
  let mut queue = std::collections::VecDeque::new();
  let mut sea_map = vec![vec![false; m]; n];
  for i in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for (j, c) in s.trim().chars().enumerate() {
      if c == '0' {
        if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
          sea_map[i][j] = true;
          queue.push_back((j, i));
        }
        water_map[i][j] = true;
      }
    }
  }

  while let Some((x, y)) = queue.pop_front() {
    if x != 0 && !sea_map[y][x - 1] && water_map[y][x - 1] {
      sea_map[y][x - 1] = true;
      queue.push_back((x - 1, y));
    }
    if x != m - 1 && !sea_map[y][x + 1] && water_map[y][x + 1] {
      sea_map[y][x + 1] = true;
      queue.push_back((x + 1, y));
    }

    if y != 0 && !sea_map[y - 1][x] && water_map[y - 1][x] {
      sea_map[y - 1][x] = true;
      queue.push_back((x, y - 1));
    }
    if y != n - 1 && !sea_map[y + 1][x] && water_map[y + 1][x] {
      sea_map[y + 1][x] = true;
      queue.push_back((x, y + 1));
    }
  }

  // 海と陸の境界カウント
  let mut count = 0_usize;
  for y in 0..n {
    for x in 0..m {
      if !sea_map[y][x] {
        // 陸から見て、隣接する箇所が海若しくは無のときに境界をカウントする
        if x != 0 && sea_map[y][x - 1] {
          count += 1;
        }
        if x == 0 {
          count += 1;
        }
        if x != m - 1 && sea_map[y][x + 1] {
          count += 1;
        }
        if x == m - 1 {
          count += 1;
        }
        if y != 0 && sea_map[y - 1][x] {
          count += 1;
        }
        if y == 0 {
          count += 1;
        }
        if y != n - 1 && sea_map[y + 1][x] {
          count += 1;
        }
        if y == n - 1 {
          count += 1;
        }
      }
    }
  }
  println!("{count}");
}
