#[derive(Debug)]
struct Temperature {
    pub fahr: f64,
    pub celsius: f64,
}

impl Temperature {
    pub fn from_fahrenheit(fahr: f64) -> Self {
        Temperature { fahr: fahr, celsius: ((fahr - 32.0) * (5.0/9.0)) }
    }

    pub fn from_celsius(celsius: f64) -> Self {
        Temperature { fahr: (celsius * (9.0/5.0) + 32.0), celsius: celsius }
    }
}