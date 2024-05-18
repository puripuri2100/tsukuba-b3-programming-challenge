pub fn main() {
  loop {
    let mut s = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut s) {
      if s.trim().is_empty() {
        break;
      }
      let mut s_l = s.trim().split_whitespace();
      let k = s_l.next().unwrap().parse::<usize>().unwrap();
      let n = s_l.next().unwrap().parse::<usize>().unwrap();
      println!("{}", f(k, n));
    } else {
      break;
    }
  }
}

fn f(k: usize, n: usize) -> f64 {
  if k == 0 || k == 1 || n == 1 {
    100.0
  } else {
    let t = 1.0 / (k + 1) as f64;
    // dp[i][j]：i番目の数字がjのときのtight wordsの確率
    let mut dp = vec![vec![t; k + 1]; n];
    for i in 1..n {
      for j in 0..=k {
        if j == 0 {
          dp[i][j] = dp[i - 1][j] + dp[i - 1][j + 1];
        } else if j == k {
          dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        } else {
          dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j] + dp[i - 1][j + 1];
        }
        // 足してるので割る
        dp[i][j] = dp[i][j] / (k + 1) as f64
      }
    }
    let mut a = 0.0;
    for j in 0..=k {
      a += dp[n - 1][j];
    }
    a * 100.0
  }
}
