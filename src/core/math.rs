use std::f64::consts;

const HUGE_VAL: f64 = f64::MAX; // or f64::INFINITY?

const LOGDERIV_ORDER_MAX: usize = 4;
const POLYGAMMA_ORDER_MAX: usize = LOGDERIV_ORDER_MAX;

/// auxiliary values for computation of derivative of ln(gamma(x))
///
/// original: `_default_gamma_coef`
const _DEFAULT_GAMMA_COEF: [f64; 11] = [
    4.694580336184385e+04,
    -1.560605207784446e+05,
    2.065049568014106e+05,
    -1.388934775095388e+05,
    5.031796415085709e+04,
    -9.601592329182778e+03,
    8.785855930895250e+02,
    -3.155153906098611e+01,
    2.908143421162229e-01,
    -2.319827630494973e-04,
    1.251639670050933e-10,
];

#[rustfmt::skip]
const PRECOMPUTED_FACTORIAL: [f64; 35] = [
    1., 1., 2., 6., 24., 120., 720., 5040., 40320., 362880.,
    3628800., 39916800., 479001600., 6227020800., 87178291200.,
    1307674368000., 20922789888000., 355687428096000., 6402373705728000.,
    121645100408832000., 2432902008176640000., 51090942171709440000.,
    1124000727777607680000., 25852016738884976640000., 620448401733239439360000.,
    15511210043330985984000000., 403291461126605635584000000., 10888869450418352160768000000.,
    304888344611713860501504000000., 8841761993739701954543616000000.,
    265252859812191058636308480000000., 8222838654177922817725562880000000.,
    263130836933693530167218012160000000., 8683317618811886495518194401280000000.,
    295232799039604140847618609643520000000.];

#[rustfmt::skip]
// #[inline(always)]
pub fn expm1(x: f64) -> f64 {
    // double BLAST_Expm1(double	x)
    // ! use f64::exp_m1 instead.
    let absx = x.abs();
    if absx > 0.33 {
        x.exp() - 1.
    } else if absx < 1e-16 {
        x
    } else {
         x * (1.             + x *
             (1./2.          + x *
             (1./6.          + x *
             (1./24.         + x *
             (1./120.        + x *
             (1./720.        + x *
             (1./5040.       + x *
             (1./40320.      + x *
             (1./362880.     + x *
             (1./3628800.    + x *
             (1./39916800.   + x *
             (1./479001600.  + x *
              1./6227020800.)))))))))))) // TODO: avoid using divisions:
    }
}

/// original: `double BLAST_Log1p(double x)`
fn log_1p(x: f64) -> f64 {
    // ! use the native f64::ln_1p instead!
    // if x.abs() >= 0.2 {
    //     return (x + 1.0).ln();
    // }
    // let mut sum = 0f64;
    // let mut y = x;
    // // limit the loop to 500 terms.
    // let mut i = 0;
    // while i < 500 {
    //     i += 1;
    //     sum += y / i as f64;
    //     if y.abs() < f64::EPSILON {
    //         break;
    //     }
    //     y *= x;
    //     i += 1;
    //     sum -= y / i as f64;
    //     if y < f64::EPSILON {
    //         break;
    //     }
    //     y *= x;
    // }
    // sum
    x.ln_1p()
}

/// original: `s_LogDerivative`
fn log_derivative(order: i32, u: &[f64]) -> f64 {
    let mut y = [0f64; LOGDERIV_ORDER_MAX + 1];
    let value: f64;
    let tmp: f64;
    if order < 0 || order > POLYGAMMA_ORDER_MAX as i32 {
        // TODO: u8?
        return HUGE_VAL; // TODO: is this correct to represent HUGE_VAL?
    }
    if order > 0 && u[0] == 0.0 {
        return HUGE_VAL;
    }
    for i in 1..=order as usize {
        y[i] = u[i] / u[0];
    }
    match order {
        0 => {
            if u[0] > 0.0 {
                value = u[0].ln();
            } else {
                return HUGE_VAL;
            }
        }
        1 => {
            value = y[1];
        }
        2 => {
            value = y[2] - y[1] * y[1];
        }
        3 => {
            value = y[3] - 3.0 * y[2] * y[1] + 2.0 * y[1].powi(3);
        }
        _ => {
            return HUGE_VAL;
        }
    }
    value
}

/// original: `s_GeneralLnGamma`
fn general_ln_gamma(x: f64, order: i32) -> f64 {
    let mut y: [f64; POLYGAMMA_ORDER_MAX + 1];
    let xx = x - 1.0; // normalize from gamma(x + 1) to xx!
                      // let tx = xx +
    unimplemented!();
}
