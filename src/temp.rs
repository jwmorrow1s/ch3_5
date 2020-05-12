#[derive(Debug, PartialEq)]
struct Temperature {
    pub fahr: f64,
    pub celsius: f64,
}

impl Temperature {
    #[allow(dead_code)]
    pub fn from_fahrenheit(fahr: f64) -> Self {
        Temperature {
            fahr: fahr,
            celsius: ((fahr - 32.0) * (5.0 / 9.0)),
        }
    }

    #[allow(dead_code)]
    pub fn from_celsius(celsius: f64) -> Self {
        Temperature {
            fahr: (celsius * (9.0 / 5.0) + 32.0),
            celsius: celsius,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Temperature::from_fahrenheit(32.0),
            Temperature {
                fahr: 32.0,
                celsius: 0.0
            }
        );
        assert_eq!(
            Temperature::from_celsius(0.0),
            Temperature {
                fahr: 32.0,
                celsius: 0.0
            }
        );
        assert_eq!(
            Temperature::from_fahrenheit(104.0),
            Temperature {
                fahr: 104.0,
                celsius: 40.0
            }
        );
        assert_eq!(
            Temperature::from_celsius(40.0),
            Temperature {
                fahr: 104.0,
                celsius: 40.0
            }
        );
    }
}
