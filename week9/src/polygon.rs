#[derive(Clone, Debug, Copy)]
struct Point {
  x: isize,
  y: isize,
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  for _ in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let m = s_l.next().unwrap().parse::<usize>().unwrap();
    let mut v = Vec::new();
    for _ in 0..m {
      let x = s_l.next().unwrap().parse().unwrap();
      let y = s_l.next().unwrap().parse().unwrap();
      v.push(Point { x, y });
    }
    let mut v = v.iter();
    let mut s = 0.0;
    let base = v.next().unwrap();
    let mut point1 = v.next().unwrap();
    let mut point2 = v.next().unwrap();
    s += f(*base, *point1, *point2);
    for p in v {
      point1 = point2;
      point2 = p;
      s += f(*base, *point1, *point2);
    }
    println!("{s}");
  }
}

fn f(base: Point, point1: Point, point2: Point) -> f64 {
  let x1 = point1.x - base.x;
  let y1 = point1.y - base.y;
  let x2 = point2.x - base.x;
  let y2 = point2.y - base.y;
  let v = (x1 * y2).abs_diff(x2 * y1);
  v as f64 * 0.5
}
