fn normal_cdf(x: f64) -> f64 {
    // approximation using Abramowitz and Stegun approximation

    const A1: f64 = 0.319381530;
    const A2: f64 = -0.356563782;
    const A3: f64 = 1.781477937;
    const A4: f64 = -1.821255978;
    const A5: f64 = 1.330274429;
    const GAMMA: f64 = 0.2316419;
    const PI: f64 = 3.14159265358979323846;

    let t = 1.0 / (1.0 + (GAMMA * x.abs()));

    let cdf_approx = 1.0
        - (1.0 / (2.0 * PI).sqrt())
            * (-x.powi(2) / 2.0).exp()
            * ((A1 * t)
                + (A2 * t.powi(2))
                + (A3 * t.powi(3))
                + (A4 * t.powi(4))
                + (A5 * t.powi(5)));

    if x >= 0.0 {
        return cdf_approx;
    } else {
        return 1.0 - cdf_approx;
    }
}

pub fn black_scholes(
    price: f64,
    strike: f64,
    rfr: f64,
    tte: f64,
    sigma: f64,
    option_type: i8,
) -> Option<f64> {
    let d1 = ((price / strike).ln() + (rfr + 0.5 * sigma.powi(2)) * tte) / (sigma * tte.sqrt());
    let d2 = d1 - sigma * tte.sqrt();

    match option_type {
        -1 => Some(strike * (-rfr * tte).exp() * normal_cdf(-d2) - price * normal_cdf(-d1)), // put
        1 => Some(price * normal_cdf(d1) - strike * (-rfr * tte).exp() * normal_cdf(d2)),    // call
        _ => None, // invalid
    }
}
