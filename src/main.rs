mod pricing;

fn main() {
    let price = 100.0;
    let strike = 110.0;
    let rfr = 0.05;
    let tte = 1.0;
    let sigma = 0.2;

    for i in [1i8, -1i8].iter() {
        let option_price = pricing::black_scholes(price, strike, rfr, tte, sigma, *i);

        let option_type = match *i {
            -1 => "put",
            1 => "call",
            _ => "unkown",
        };

        println!("{} price: {:.2}", option_type, option_price)
    }
}
