pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let c = s.trim().parse::<usize>().unwrap();
    if c == 0 {
      break;
    }
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut ratio_map = vec![vec![None; c]; c];
    let mut currencies = std::collections::HashMap::new();
    for (i, s) in s.split_whitespace().enumerate() {
      currencies.insert(s.to_string(), i);
      ratio_map[i][i] = Some(1.0);
    }

    let mut is_ans = false;

    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let r = s.trim().parse::<usize>().unwrap();
    for _ in 0..r {
      s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.split_whitespace();
      let c1 = s_l.next().unwrap();
      let n1 = currencies.get(c1).unwrap();
      let c2 = s_l.next().unwrap();
      let n2 = currencies.get(c2).unwrap();
      let mut r_s = s_l.next().unwrap().split(':');
      let r1 = r_s.next().unwrap().parse::<usize>().unwrap();
      let r2 = r_s.next().unwrap().parse::<usize>().unwrap();
      let ratio1 = r2 as f64 / r1 as f64;
      let ratio2 = r1 as f64 / r2 as f64;
      if let Some(ratio) = ratio_map[*n1][*n2] {
        if ratio != ratio1 {
          println!("Arbitrage");
          is_ans = true;
          break;
        }
      } else {
        ratio_map[*n1][*n2] = Some(ratio1)
      }

      if let Some(ratio) = ratio_map[*n2][*n1] {
        if ratio != ratio2 {
          println!("Arbitrage");
          is_ans = true;
          break;
        }
      } else {
        ratio_map[*n2][*n1] = Some(ratio2)
      }
    }

    for k in 0..c {
      for i in 0..c {
        for j in 0..c {
          let v1 = ratio_map[i][k];
          let v2 = ratio_map[k][j];
          let v3 = ratio_map[i][j];
          if let Some(v3) = v3 {
            if let (Some(v1), Some(v2)) = (v1, v2) {
              if v3 < v1 * v2 {
                ratio_map[i][j] = Some(v1 * v2)
              }
            }
          } else if let (Some(v1), Some(v2)) = (v1, v2) {
            ratio_map[i][j] = Some(v1 * v2)
          }
        }
      }
    }
    #[allow(clippy::needless_range_loop)]
    for i in 0..c {
      if let Some(v) = ratio_map[i][i] {
        if v > 1.0 {
          println!("Arbitrage");
          is_ans = true;
          break;
        }
      }
    }

    if !is_ans {
      println!("Ok")
    }
  }
}
