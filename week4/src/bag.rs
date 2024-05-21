pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let game = s.trim().parse::<usize>().unwrap();
  for g in 1..=game {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let m = s.trim().parse::<usize>().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut tiles = s
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    tiles.sort();
    let mut tile_types = Vec::new();
    let mut tmp_n = tiles[0];
    let mut tmp_len = 0_usize;
    for v in tiles.iter() {
      if tmp_n == *v {
        tmp_len += 1;
      } else {
        tile_types.push((tmp_n, tmp_len));
        tmp_n = *v;
        tmp_len = 1;
      }
    }
    tile_types.push((tmp_n, tmp_len));
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.trim().split_whitespace();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    let t = s_l.next().unwrap().parse::<usize>().unwrap();

    println!("{tile_types:?}");
    if n == 0 && t == 0 {
      println!("Game {g} -- 1 : 0");
    } else if n == 0 {
      println!("Game {g} -- 0 : 1");
    } else {
      // 個数制限有ナップザックDPを解き、それぞれの個数の組み合わせ数を計算する
      // タイルの値ごとの枚数の値をリストで保持する
      let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![]; t + 1];
      dp[0] = vec![vec![0; tile_types.len()]];
      for i in 1..=t {
        let mut new_lst = Vec::new();
        for (j, (v, _)) in tile_types.iter().enumerate() {
          if i >= *v {
            // タイルの数の方が小さいので探索できる
            let lst = &dp[i - v];
            for l in lst.iter() {
              if l[j] < tile_types[j].1 {
                // まだ余裕があるから追加できる
                let mut l2 = l.clone();
                l2[j] += 1;
                new_lst.push(l2);
              }
            }
          }
        }
        new_lst.sort();
        new_lst.dedup();
        dp[i] = new_lst;
        //println!("{i}: {:?}", dp[i]);
      }
      let all = ncr(m, n);
      let mut win = 0;
      for lst in dp[t].iter() {
        let n2 = lst.iter().sum::<usize>();
        if n == n2 {
          // 合計が条件と一致しているときのみ
          let mut local = 1;
          for (i, n) in lst.iter().enumerate() {
            if *n != 0 {
              let (_, m) = tile_types[i];
              local *= ncr(m, *n);
            }
          }
          win += local;
        }
      }
      println!("Game {g} -- {win} {}", all - win);
    }
  }
}

fn ncr(n: usize, r: usize) -> usize {
  let mut a = 1;
  for v in (n - r + 1)..=n {
    a *= v;
  }
  for v in 2..=r {
    a /= v;
  }
  a
}
