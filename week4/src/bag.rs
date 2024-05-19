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
    let tiles = s
      .trim()
      .split_whitespace()
      .map(|s| s.parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.trim().split_whitespace();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    let t = s_l.next().unwrap().parse::<usize>().unwrap();

    if n == 0 && t == 0 {
      println!("Game {g} -- 1 : 0");
    } else if n == 0 {
      println!("Game {g} -- 0 : 1");
    } else {
      let l = vec![vec![0; t + 1]; n + 1];
      let r = vec![vec![0; t + 1]; n + 1];
    }
  }
}
