#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let mut h = s_l.next().unwrap().parse::<usize>().unwrap();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let m = s_l.next().unwrap().parse::<usize>().unwrap();

  // 空かどうかのマップ
  let mut group_map: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; m]; n];
  let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

  for i in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s_l = s.split_whitespace();
    for (j, s) in s_l.enumerate() {
      if s == "." {
        if i % 2 != 0 {
          if let Some(group) = group_map[i - 1][j] {
            group_map[i][j] = Some(group);
          }
          if j != m - 1 {
            if let Some(group) = group_map[i - 1][j + 1] {
              group_map[i][j] = Some(group);
            }
          }
          if j != 0 {
            if let Some(group) = group_map[i][j - 1] {
              group_map[i][j] = Some(group);
            }
          }
        } else {
          if i != 0 {
            if let Some(group) = group_map[i - 1][j] {
              group_map[i][j] = Some(group);
            }
            if j != 0 {
              if let Some(group) = group_map[i - 1][j - 1] {
                group_map[i][j] = Some(group);
              }
            }
          }
          if j != 0 {
            if let Some(group) = group_map[i][j - 1] {
              group_map[i][j] = Some(group);
            }
          }
        }
        group_map[i][j] = Some((i, j));
        queue.push_back((i, j));
      }
    }
  }

  while let Some((i, j)) = queue.pop_front() {
    let now_group = group_map[i][j].unwrap();
    //println!("({i}, {j}): {now_group:?}");
    // 隣接する6つの座標を検査
    if i % 2 != 0 {
      // 奇数段目
      // 上左は必ずある
      let i2 = i - 1;
      let j2 = j;
      if let Some(now_group2) = group_map[i2][j2] {
        if now_group != now_group2 {
          group_map[i2][j2] = Some(now_group);
          queue.push_back((i2, j2));
        }
      }
      if j != 0 {
        // 左ある
        let i2 = i;
        let j2 = j - 1;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
      }
      if i != n - 1 {
        // 下左ある
        let i2 = i + 1;
        let j2 = j;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
      }
      if j != m - 1 {
        // 右ある
        // 上右ある
        let i2 = i - 1;
        let j2 = j + 1;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
        // 右ある
        let i2 = i;
        let j2 = j + 1;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
        // 下右ある
        if i != n - 1 {
          let i2 = i + 1;
          let j2 = j + 1;
          if let Some(now_group2) = group_map[i2][j2] {
            if now_group != now_group2 {
              group_map[i2][j2] = Some(now_group);
              queue.push_back((i2, j2));
            }
          }
        }
      }
    } else {
      // 偶数段目
      if j != 0 {
        // 左がある
        if i != 0 {
          // 上左がある
          let i2 = i - 1;
          let j2 = j - 1;
          if let Some(now_group2) = group_map[i2][j2] {
            if now_group != now_group2 {
              group_map[i2][j2] = Some(now_group);
              queue.push_back((i2, j2));
            }
          }
        }
        // 左がある
        let i2 = i;
        let j2 = j - 1;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
        if i != n - 1 {
          // 上左がある
          let i2 = i + 1;
          let j2 = j - 1;
          if let Some(now_group2) = group_map[i2][j2] {
            if now_group != now_group2 {
              group_map[i2][j2] = Some(now_group);
              queue.push_back((i2, j2));
            }
          }
        }
      }
      if i != 0 {
        // 上右がある
        let i2 = i - 1;
        let j2 = j;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
      }
      if j != m - 1 {
        // 右がある
        let i2 = i;
        let j2 = j + 1;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
      }
      if i != n - 1 {
        // 下右がある
        let i2 = i + 1;
        let j2 = j;
        if let Some(now_group2) = group_map[i2][j2] {
          if now_group != now_group2 {
            group_map[i2][j2] = Some(now_group);
            queue.push_back((i2, j2));
          }
        }
      }
    }
  }

  let mut count_map: HashMap<(usize, usize), usize> = HashMap::new();
  for i in 0..n {
    for j in 0..m {
      if let Some(group) = group_map[i][j] {
        if let Some(count) = count_map.get(&group) {
          count_map.insert(group, *count + 1);
        } else {
          count_map.insert(group, 1);
        }
      }
    }
  }

  let mut lst = Vec::new();
  for (_, v) in count_map.iter() {
    lst.push(*v);
  }
  lst.sort_by(|a, b| b.cmp(a));

  let mut ans = 0_usize;
  for i in lst.iter() {
    if h <= *i {
      ans += 1;
      break;
    } else {
      h -= *i;
      ans += 1;
    }
  }

  println!("{ans}");
}
