fn solve(alst: &Vec<usize>, blst: &Vec<usize>) -> usize {
  let mut alst = alst.clone();
  let mut blst = blst.clone();
  let mut ans = 0;
  let mut anum = 1;
  let mut bnum = 100;
  while anum <= 100 && bnum > 0 {
    if alst[anum] == 0 {
      anum += 1;
    } else if blst[bnum] == 0 {
      bnum -= 1;
    } else {
      // 上下ででかいのと小さいのを取り合ってるのでまんなかに近い値が出る
      ans = std::cmp::max(ans, anum + bnum);
      // 差分を取っていく
      let diff = alst[anum].abs_diff(blst[bnum]);
      if diff == 0 {
        alst[anum] = 0;
        blst[bnum] = 0;
        anum += 1;
        bnum -= 1;
      } else if alst[anum] > blst[bnum] {
        alst[anum] = diff;
        bnum -= 1;
      } else {
        blst[bnum] = diff;
        anum += 1;
      }
    }
  }
  ans
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let n = s.trim().parse::<usize>().unwrap();
  let mut alst = vec![0; 101];
  let mut blst = vec![0; 101];
  for _ in 0..n {
    let mut s = String::new();
    if std::io::stdin().read_line(&mut s).is_err() {
      break;
    }
    if s.trim().is_empty() {break;}
    let mut s_lst = s.trim().split_ascii_whitespace();
    let a = s_lst.next().unwrap().parse::<usize>().unwrap();
    let b = s_lst.next().unwrap().parse::<usize>().unwrap();
    alst[a] += 1;
    blst[b] += 1;
    let ans = solve(&alst, &blst);
    println!("{}", ans);
  }
}