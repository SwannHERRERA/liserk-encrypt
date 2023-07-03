use std::f64;
use std::iter::Iterator;

pub struct PRNG {
    pub coins: Box<dyn Iterator<Item = bool>>,
}

impl PRNG {
    pub fn new(coins: Box<dyn Iterator<Item = bool>>) -> PRNG {
        PRNG { coins }
    }

    pub fn draw(&mut self) -> f64 {
        let mut bits = Vec::new();
        for _ in 0..32 {
            bits.push(self.coins.next().unwrap());
        }
        assert_eq!(bits.len(), 32);

        let mut out = 0;
        for &b in &bits {
            out = (out << 1) | (b as u32);
        }

        let res = out as f64 / ((1u64 << 32) - 1) as f64;
        assert!(res >= 0.0 && res <= 1.0);
        res
    }
}

pub fn afc(i: i32) -> f64 {
    if i < 0 {
        panic!("i should not be < 0");
    } else if i == 0 {
        return 0.0;
    }

    let frac_12 = 1.0 / 12.0;
    let frac_360 = 1.0 / 360.0;
    let frac_pi = 0.5 * f64::consts::PI.ln();

    let i = i as f64;
    (i + 0.5) * i.ln() - i + frac_12 / i - frac_360 / (i * i * i) + frac_pi
}

fn loggam(x: f64) -> f64 {
    let a = [
        8.333333333333333e-02,
        -2.777777777777778e-03,
        7.936507936507937e-04,
        -5.952380952380952e-04,
        8.417508417508418e-04,
        -1.917526917526918e-03,
        6.410256410256410e-03,
        -2.955065359477124e-02,
        1.796443723688307e-01,
        -1.39243221690590,
    ];

    let mut x0 = x;
    let mut n = 0;

    if x0 == 1.0 || x0 == 2.0 {
        return 0.0;
    } else if x0 <= 7.0 {
        n = (7.0 - x0) as i32;
        x0 += n as f64;
    }

    let x2 = 1.0 / (x0 * x0);
    let xp = 2.0 * f64::consts::PI;

    let mut gl0 = a[9];
    for k in (0..=8).rev() {
        gl0 = gl0 * x2 + a[k];
    }

    let mut gl = gl0 / x0 + 0.5 * xp.ln() + (x0 - 0.5) * x0.ln() - x0;
    if x <= 7.0 {
        for _k in 1..=n {
            gl -= (x0 - 1.0).ln();
            x0 -= 1.0;
        }
    }
    gl
}

pub fn hypergeometric_hyp(prng: &mut PRNG, good: f64, bad: f64, sample: f64) -> i32 {
    let d1 = bad + good - sample;
    let d2 = f64::min(bad, good);

    let mut y = d2;
    let mut k = sample;
    while y > 0.0 {
        let u = prng.draw();
        y -= (u + y / (d1 + k)).floor();
        k -= 1.0;
        if k == 0.0 {
            break;
        }
    }

    let mut z = (d2 - y) as i32;
    if good > bad {
        z = sample as i32 - z;
    }
    z
}

pub fn hypergeometric_hrua(prng: &mut PRNG, good: f64, bad: f64, sample: f64) -> i32 {
    const D1: f64 = 1.7155277699214135;
    const D2: f64 = 0.8989161620588988;

    let mingoodbad = good.min(bad);
    let popsize = good + bad;
    let maxgoodbad = good.max(bad);
    let m = sample.min(popsize - sample);
    let d4 = mingoodbad / popsize;
    let d5 = 1.0 - d4;
    let d6 = m * d4 + 0.5;
    let d7 = ((popsize - m) * sample * d4 * d5 / (popsize - 1.0) + 0.5).sqrt();
    let d8 = D1 * d7 + D2;
    let d9 = ((m + 1.0) * (mingoodbad + 1.0) / (popsize + 2.0)).floor() as i32;
    let d10 = loggam(d9 as f64 + 1.0)
        + loggam(mingoodbad - d9 as f64 + 1.0)
        + loggam(m - d9 as f64 + 1.0)
        + loggam(maxgoodbad - m + d9 as f64 + 1.0);
    let d11 = m.min(mingoodbad) + 1.0_f64.min((d6 + 16.0 * d7).floor());

    loop {
        let x = prng.draw();
        let y = prng.draw();
        let w = d6 + d8 * (y - 0.5) / x;

        if w < 0.0 || w >= d11 {
            continue;
        }

        let z = w.floor() as i32;
        let t = d10
            - (loggam(z as f64 + 1.0)
                + loggam(mingoodbad - z as f64 + 1.0)
                + loggam(m - z as f64 + 1.0)
                + loggam(maxgoodbad - m + z as f64 + 1.0));

        if (x * (4.0 - x) - 3.0) <= t {
            let mut result = z;
            if good > bad {
                result = m as i32 - result;
            }
            if m < sample {
                result = good as i32 - result;
            }
            return result;
        }

        if x * (x - t) >= 1.0 {
            continue;
        }

        if 2.0 * x.ln() <= t {
            let mut result = z;
            if good > bad {
                result = m as i32 - result;
            }
            if m < sample {
                result = good as i32 - result;
            }
            return result;
        }
    }
}

pub type Coins = Box<dyn Iterator<Item = bool> + 'static>;

pub fn rhyper(kk: i32, nn1: f64, nn2: f64, coins: Coins) -> i32 {
    let mut prng = PRNG::new(coins);

    if kk > 10 {
        hypergeometric_hrua(&mut prng, nn1, nn2, kk as f64)
    } else {
        hypergeometric_hyp(&mut prng, nn1, nn2, kk as f64)
    }
}
