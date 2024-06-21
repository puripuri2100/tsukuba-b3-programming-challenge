use std::f64::consts::PI;

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let r = s_l.next().unwrap().parse::<f64>().unwrap();
    let h = s_l.next().unwrap().parse::<f64>().unwrap();
    let s = s_l.next().unwrap().parse::<f64>().unwrap();
    if r == 0.0 && h == 0.0 && s == 0.0 {
      break;
    }

    // 点(0, h)から円x^2 + y^2 = r^2に引いた接点(p, q)における接線の公式は
    // px + qy = r^2
    // 接線は点(0,h)を通るので、式に代入するとqh = r^2
    // また、点(p, q)は円周上なので p^2 + q^2 = r^2
    // よって連立して
    // qh = p^2 + q^2 = r^2
    let q = r.powf(2.0) / h;
    let p = (r.powf(2.0) - q.powf(2.0)).sqrt();
    let mut len = (p.powf(2.0) + (h - q).powf(2.0)).sqrt() * 2.0;
    let rad = (p / r).acos();
    // 2r * π * ((rad * 2 + π) / 2π) = (r * rad) + r * π
    let len2 = (2.0 * r * rad) + r * PI;
    len += len2;
    len *= 1.0 + s / 100.0;
    println!("{:.2}", len);
  }
}
