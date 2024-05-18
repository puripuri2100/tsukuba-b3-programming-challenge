pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();

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

  // dp[i] i円のメニューを構成する方法がいくつあるのか
  let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![]; *max_query + 1];
  for i in 1..=*max_query {
    let mut new_dp_i = Vec::new();
    for (index, value) in menu.iter().enumerate() {
      if i == *value {
        new_dp_i.push(vec![index + 1]);
      }
      if i > *value {
        let mut lst = dp[i - value].clone();
        for l in lst.iter_mut() {
          l.push(index + 1);
          l.sort();
          new_dp_i.push(l.to_vec());
        }
        //println!("{i}, {value}: {lst:?} -> {new:?}");
      }
    }
    new_dp_i.sort();
    new_dp_i.dedup();
    dp[i] = new_dp_i;
    //println!("{i}: {:?}", dp.iter().enumerate().collect::<Vec<_>>());
  }

  for v in query.iter() {
    let a = &dp[*v];
    let l = a.len();
    if l == 0 {
      println!("Impossible")
    } else if l > 1 {
      println!("Ambiguous")
    } else {
      let a = a[0]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<_>>()
        .join(" ");
      println!("{a}")
    }
  }
}
