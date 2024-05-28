pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let case = s.trim().parse::<usize>().unwrap();
  for _ in 0..case {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    let m = s_l.next().unwrap().parse::<usize>().unwrap();
    let l = s_l.next().unwrap().parse::<usize>().unwrap();
    let _s = s_l.next().unwrap().parse::<usize>().unwrap();
    let mut start = Vec::new();
    let mut stations = vec![vec![]; n + 1];
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    for s in s.split_whitespace() {
      let n = s.parse::<usize>().unwrap();
      start.push(n)
    }
    for _ in 0..m {
      s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.split_whitespace();
      let i = s_l.next().unwrap().parse::<usize>().unwrap();
      let j = s_l.next().unwrap().parse::<usize>().unwrap();
      let e = s_l.next().unwrap().parse::<usize>().unwrap();
      stations[i].push((j, e));
      stations[j].push((i, e))
    }

    let mut coast = vec![None; n + 1];
    let mut queue = std::collections::VecDeque::new();
    for s in start {
      coast[s] = Some((s, 0_usize));
      for (n, e_ij) in &stations[s] {
        if let Some((_, e)) = coast[*n] {
          if e > e_ij + l {
            // 小さい値で更新
            coast[*n] = Some((s, e_ij + l));
            queue.push_back(n);
          }
        } else {
          coast[*n] = Some((s, e_ij + l));
          queue.push_back(n);
        }
      }
    }

    while let Some(n1) = queue.pop_front() {
      for (n2, e_ij) in &stations[*n1] {
        // 親側の値は更新してはいけない
        // n1：値が更新されたマス
        // s：n1の親のマス
        // s2：sの親のマス
        // n2：n1に依存するマス
        if let Some((s, e)) = coast[*n2] {
          // 循環参照にならないようにするチェック
          let is_ok = if let Some((s2, _)) = coast[*n1] {
            s2 != *n2
          } else {
            true
          };
          if is_ok && s != *n2 && e > e_ij + l {
            coast[*n2] = Some((*n1, e_ij + l));
            queue.push_back(n2);
          }
        } else {
          coast[*n2] = Some((*n1, e_ij + l));
          queue.push_back(n2);
        }
      }
    }

    let mut ans = 0;
    for (_, e) in coast.iter().flatten() {
      ans += e;
    }
    println!("{ans}");
  }
}
