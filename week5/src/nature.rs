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
    let mut root_info = vec![None; n + 1];
    let mut childrens = vec![Vec::new(); n + 1];
    let mut links = Vec::new();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut root = 0;
    for (i, s) in s.split_whitespace().enumerate() {
      let n = s.parse::<usize>().unwrap();
      if i == 0 {
        root_info[n] = Some(n);
        root = n;
        childrens[n].push(n);
      } else {
        root_info[n] = Some(root);
        childrens[root].push(n);
      }
    }
    for _ in 0..m {
      s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.split_whitespace();
      let i = s_l.next().unwrap().parse::<usize>().unwrap();
      let j = s_l.next().unwrap().parse::<usize>().unwrap();
      let e = s_l.next().unwrap().parse::<usize>().unwrap();
      links.push((i, j, e + l));
    }
    links.sort_by_key(|(_, _, e)| *e);

    let mut ans = 0;
    for (i, j, e) in links.iter() {
      match (root_info[*i], root_info[*j]) {
        (None, None) => {
          // 新しくリンクを登録する
          root_info[*i] = Some(*i);
          root_info[*j] = Some(*i);
          childrens[*i].push(*i);
          childrens[*i].push(*j);
          ans += e;
        }
        (Some(root1), None) => {
          // すでに片方がグラフに繋がってるのでそこに接続する
          root_info[*j] = Some(root1);
          childrens[root1].push(*j);
          ans += e;
        }
        (None, Some(root1)) => {
          // すでに片方がグラフに繋がってるのでそこに接続する
          root_info[*i] = Some(root1);
          childrens[root1].push(*i);
          ans += e;
        }
        (Some(root1), Some(root2)) => {
          // rootが一致していたらすでにより小さいコストで繋がっているので何もしない
          if root1 != root2 {
            // 片方にもう片方を結合させる
            let children2 = childrens[root2].clone();
            for n in children2.iter() {
              root_info[*n] = Some(root1);
              childrens[root1].push(*n);
            }
            childrens[root2] = Vec::new();
            ans += e;
          }
        }
      }
    }
    println!("{ans}");
  }
}
