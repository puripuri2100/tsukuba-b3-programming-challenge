pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let menu_n = s.trim().parse::<usize>().unwrap();

  // 値段のリスト
  let mut menu = vec![];
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  for c in s.trim().split_whitespace() {
    menu.push(c.parse::<usize>().unwrap())
  }

  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();

  // 注文
  let mut query = vec![];
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  for c in s.trim().split_whitespace() {
    query.push(c.parse::<usize>().unwrap())
  }

  let max_query = query.iter().max().unwrap();

  // dp[i] i円のメニューの構成組み合わせ
  let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![]; *max_query + 1];
  dp[0] = vec![vec![0; menu_n]];
  for i in 1..=*max_query {
    let mut new_dp_i = Vec::new();
    for (index, value) in menu.iter().enumerate() {
      if i >= *value {
        let mut lst = dp[i - value].clone();
        for l in lst.iter_mut() {
          l[index] += 1;
          new_dp_i.push(l.to_vec());
        }
      }
    }
    new_dp_i.sort();
    new_dp_i.dedup();
    if new_dp_i.len() > 2 {
      // 枝刈り
      dp[i] = vec![new_dp_i[0].clone(), new_dp_i[1].clone()];
    } else {
      dp[i] = new_dp_i;
    }
  }

  for v in query.iter() {
    let a = &dp[*v];
    let l = a.len();
    if l == 0 {
      println!("Impossible")
    } else if l > 1 {
      println!("Ambiguous")
    } else {
      let mut s = String::new();
      for (i, n) in a[0].iter().enumerate() {
        for _ in 0..*n {
          s.push_str(&format!(" {}", i + 1));
        }
      }
      println!("{}", s.trim());
    }
  }
}
