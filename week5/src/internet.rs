// (c) tanakah
// <https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8>
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
      let mut iter = $s.split_whitespace();
      let mut next = || { iter.next().unwrap() };
      input_inner!{next, $($r)*}
  };
  ($($r:tt)*) => {
      let stdin = std::io::stdin();
      let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
      let mut next = move || -> String{
          bytes
              .by_ref()
              .map(|r|r.unwrap() as char)
              .skip_while(|c|c.is_whitespace())
              .take_while(|c|!c.is_whitespace())
              .collect()
      };
      input_inner!{next, $($r)*}
  };
}

macro_rules! input_inner {
  ($next:expr) => {};
  ($next:expr, ) => {};

  ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
      let $var = read_value!($next, $t);
      input_inner!{$next $($r)*}
  };
}

macro_rules! read_value {
  ($next:expr, ( $($t:tt),* )) => {
      ( $(read_value!($next, $t)),* )
  };

  ($next:expr, [ $t:tt ; $len:expr ]) => {
      (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
  };

  ($next:expr, chars) => {
      read_value!($next, String).chars().collect::<Vec<char>>()
  };

  ($next:expr, usize1) => {
      read_value!($next, usize) - 1
  };

  ($next:expr, $t:ty) => {
      $next().parse::<$t>().expect("Parse error")
  };
}

pub fn main() {
  input! {
    n: usize,
    m: usize,
    lst: [[usize; 2]; m],
  }

  // 無向グラフ
  // graph[i]: 繋がっている頂点
  let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
  for l in lst.iter() {
    graph[l[0]].push(l[1]);
    graph[l[0]].sort();
    graph[l[0]].dedup();
    graph[l[1]].push(l[0]);
    graph[l[1]].sort();
    graph[l[1]].dedup();
  }

  //println!("{graph:?}");

  let mut checked = vec![false; n + 1];
  checked[0] = true;
  checked[1] = true;
  for (i, l) in graph.iter().enumerate() {
    if i != 0 {
      let is_connect = l.iter().any(|i| checked[*i]) || checked[i];
      if is_connect {
        checked[i] = true;
        for i in l.iter() {
          checked[*i] = true;
        }
      }
    }
  }

  //println!("{checked:?}");

  let mut count = 0_usize;

  for (i, _) in checked.iter().enumerate() {
    if !checked[i] {
      println!("{i}");
      count += 1;
    }
  }
  if count == 0 {
    println!("Connected");
  }
}
