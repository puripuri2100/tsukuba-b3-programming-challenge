use std::f64::consts::PI;

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let a = s_l.next().unwrap().parse::<usize>().unwrap();
    let b = s_l.next().unwrap().parse::<usize>().unwrap();
    let s = s_l.next().unwrap().parse::<usize>().unwrap();
    let m = s_l.next().unwrap().parse::<usize>().unwrap();
    let n = s_l.next().unwrap().parse::<usize>().unwrap();
    if a == 0 && b == 0 && s == 0 && m == 0 && n == 0 {
      break;
    }

    // 長方形を公倍数分並べて直線を引けば、反射が全て展開される
    let x = a * m;
    let y = b * n;
    // Θ / 2π = x / 360 ⇔ x = Θ (360 / 2π)
    let rand = (y as f64 / x as f64).atan() * (180.0 / PI);
    let len = ((x * x + y * y) as f64).sqrt() / s as f64;
    println!("{:.2} {:.2}", rand, len);
  }
}
