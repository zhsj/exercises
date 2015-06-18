use std::default::Default;
use std::f64::NAN;

trait RungeKutta {
    fn rk_f(&self, x:f64, y:f64)->f64;
    fn rk_next(&mut self) -> (f64,f64);
}

impl RungeKutta for Problem {
    fn rk_f(&self, x:f64, y:f64) -> f64 {
        -x*x*y*y
    }

    fn rk_next(& mut self) -> (f64,f64) {
        if self.xn + self.h <= self.end_x {
            let k1 = self.rk_f(self.xn, self.yn);
            let k2 = self.rk_f(self.xn + 0.5*self.h, self.yn + 0.5*self.h*k1);
            let k3 = self.rk_f(self.xn + 0.5*self.h, self.yn + 0.5*self.h*k2);
            let k4 = self.rk_f(self.xn + self.h, self.yn + self.h*k3);
            self.yn = self.yn + self.h/6.0 * (k1+2.0*k2+2.0*k3+k4);
            self.xn = self.xn + self.h;
        }
        return (self.xn, self.yn);
    }
}

trait Adams {
    fn adams_next(& mut self) -> (f64,f64);
}

impl Adams for Problem {
    fn adams_next(& mut self) -> (f64,f64) {
        if self.xn_prev.is_nan() {
            self.xn_prev = self.xn;
            self.yn_prev = self.yn;
            let (xn,yn) = self.rk_next();
            self.xn = xn;
            self.yn = yn;
        } else if self.xn + self.h <= self.end_x{
            let c = -12.0/self.h*self.yn + 8.0*self.xn*self.xn*self.yn*self.yn -
                self.xn_prev*self.xn_prev*self.yn_prev*self.yn_prev;
            let b = 12.0/self.h;
            self.xn_prev = self.xn;
            self.yn_prev = self.yn;
            self.xn = self.xn + self.h;
            let a = 5.0*self.xn*self.xn;
            let y1 = (-b+(b*b-4.0*a*c).sqrt())/(2.0*a);
            let y2 = (-b-(b*b-4.0*a*c).sqrt())/(2.0*a);
            //println!("D: {}, {}",y1,y2);
            if (self.yn - y1).abs() < (self.yn - y2).abs() {
                self.yn = y1;
            } else {
                self.yn = y2;
            }
        }
        return (self.xn, self.yn);
    }

}

trait Init {
    fn init(& mut self);
}

impl Init for Problem {
    fn init(& mut self){
        self.xn = self.start_x;
        self.yn = self.start_y;
        self.xn_prev = NAN;
        self.yn_prev = NAN;
    }
}

struct Problem {
    xn:f64,
    yn:f64,
    xn_prev:f64,
    yn_prev:f64,
    start_x:f64,
    start_y:f64,
    end_x:f64,
    h:f64,
}

impl Default for Problem {
    fn default() -> Problem {
        Problem {xn:NAN, yn:NAN, xn_prev:NAN, yn_prev:NAN, start_x:NAN, start_y:NAN, end_x:NAN, h:NAN}
    }
}

fn error(x:f64, y:f64) -> f64 {
    (3.0/(1.0+x*x*x) - y).abs()
}

fn main() {
    println!("Runge-Kutta 法的误差和误差阶:");
    let test = vec![1,2,4,8];
    let mut prev_err = NAN;
    for i in test {
        let mut p = Problem {start_x:0.0, start_y:3.0, end_x:1.5, h:0.1/(i as f64), ..Default::default()};
        p.init();
        let mut x= 0.0;
        loop {
            let (xn,yn) = p.rk_next();
            if (xn-x).abs() < 1e-6 {
                let err = error(xn,yn);
                if prev_err.is_nan() {
                    println!("步长：{:3.4}, {:16.12e}", 0.1/(i as f64), err);
                } else {
                    let err = error(xn,yn);
                    println!("步长：{:3.4}, {:16.12e}, {:16.12e}", 0.1/(i as f64), error(xn,yn), (prev_err/err).log2());
                }
                prev_err = err;
                break;
            }
            x = xn;
        }
    }
    println!("Adams 法:");
    let test = vec![1,2,4,8];
    let mut prev_err = NAN;
    for i in test {
        let mut p = Problem {start_x:0.0, start_y:3.0, end_x:1.5, h:0.1/(i as f64), ..Default::default()};
        p.init();
        let mut x= 0.0;
        loop {
            let (xn,yn) = p.adams_next();
            if (xn-x).abs() < 1e-6 {
                let err = error(xn,yn);
                if prev_err.is_nan() {
                    println!("步长：{:3.4}, {:16.12e}", 0.1/(i as f64), err);
                } else {
                    let err = error(xn,yn);
                    println!("步长：{:3.4}, {:16.12e}, {:16.12e}", 0.1/(i as f64), error(xn,yn), (prev_err/err).log2());
                }
                prev_err = err;
                break;
            }
            x = xn;
        }
    }


}
