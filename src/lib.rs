#![no_std]
#[cfg(feature = "std")]
extern crate std;

#[macro_export]
macro_rules! unit {
    ($unit:ident, $symbol:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        struct $unit(f32);

        impl core::ops::Div<f32> for $unit {
            type Output = $unit;

            fn div(self, rhs: f32) -> Self::Output {
                $unit(self.0 / rhs)
            }
        }

        impl core::ops::Mul<f32> for $unit {
            type Output = $unit;

            fn mul(self, rhs: f32) -> Self::Output {
                $unit(self.0 * rhs)
            }
        }

        impl core::ops::Div for $unit {
            type Output = f32;

            fn div(self, rhs: $unit) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl core::ops::Sub for $unit {
            type Output = $unit;

            fn sub(self, rhs: $unit) -> Self::Output {
                $unit(self.0 - rhs.0)
            }
        }

        impl core::ops::SubAssign for $unit {
            fn sub_assign(&mut self, rhs: $unit) {
                self.0 -= rhs.0;
            }
        }

        impl core::ops::Add for $unit {
            type Output = $unit;

            fn add(self, rhs: $unit) -> Self::Output {
                $unit(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign for $unit {
            fn add_assign(&mut self, rhs: $unit) {
                self.0 += rhs.0;
            }
        }

        #[cfg(feature = "std")]
        impl std::fmt::Display for $unit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", self.0, $symbol)
            }
        }

        #[cfg(feature = "defmt")]
        impl defmt::Format for $unit {
            fn format(&self, fmt: defmt::Formatter) {
                defmt::write!(fmt, "{}{}", self.0, $symbol)
            }
        }
    };
}

#[macro_export]
macro_rules! divide_units {
    ($numerator_unit:ident, $denominator_unit:ident, $quotient_unit:ident) => {
        impl core::ops::Div<$denominator_unit> for $numerator_unit {
            type Output = $quotient_unit;

            fn div(self, rhs: $denominator_unit) -> Self::Output {
                $quotient_unit(self.0 / rhs.0)
            }
        }

        impl core::ops::Mul<$denominator_unit> for $quotient_unit {
            type Output = $numerator_unit;

            fn mul(self, rhs: $denominator_unit) -> Self::Output {
                $numerator_unit(self.0 * rhs.0)
            }
        }

        impl core::ops::Mul<$quotient_unit> for $denominator_unit {
            type Output = $numerator_unit;

            fn mul(self, rhs: $quotient_unit) -> Self::Output {
                $numerator_unit(self.0 * rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! multiply_units {
    ($unit_1:ident, $unit_2:ident, $product_unit:ident) => {
        impl core::ops::Mul<$unit_2> for $unit_1 {
            type Output = $product_unit;

            fn mul(self, rhs: $unit_2) -> Self::Output {
                $product_unit(self.0 * rhs.0)
            }
        }

        impl core::ops::Mul<$unit_1> for $unit_2 {
            type Output = $product_unit;

            fn mul(self, rhs: $unit_1) -> Self::Output {
                $product_unit(self.0 * rhs.0)
            }
        }

        impl core::ops::Div<$unit_1> for $product_unit {
            type Output = $unit_2;

            fn div(self, rhs: $unit_1) -> Self::Output {
                $unit_2(self.0 / rhs.0)
            }
        }

        impl core::ops::Div<$unit_2> for $product_unit {
            type Output = $unit_1;

            fn div(self, rhs: $unit_2) -> Self::Output {
                $unit_1(self.0 / rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! convert_units {
    ($from_unit:ident, $to_unit:ident, $factor:expr) => {
        impl From<$from_unit> for $to_unit {
            fn from(from: $from_unit) -> $to_unit {
                $to_unit(from.0 * $factor)
            }
        }
    };
}

#[cfg(test)]
mod tests {

    unit!(Volt, "V");
    unit!(Amp, "A");
    unit!(Ohm, "Ω");

    multiply_units!(Amp, Ohm, Volt);

    unit!(Degree, "°");
    unit!(Radian, "rad");

    const PI: f32 = 3.14159;

    convert_units!(Degree, Radian, PI / 180.0);
    convert_units!(Radian, Degree, 180.0 / PI);

    #[test]
    #[cfg(feature = "std")]
    fn display() {
        assert_eq!("1.5V", std::format!("{}", Volt(1.5)));
        assert_eq!("1.5A", std::format!("{}", Amp(1.5)));
        assert_eq!("1.5Ω", std::format!("{}", Ohm(1.5)));
        assert_eq!("120°", std::format!("{}", Degree(120.0)));
        assert_eq!("1.5rad", std::format!("{}", Radian(1.5)));
    }

    #[test]
    fn divide_scalar() {
        assert_eq!(Volt(10.0) / 5.0, Volt(2.0));
    }

    #[test]
    fn divide_units() {
        assert_eq!(Volt(10.0) / Amp(2.0), Ohm(5.0));
    }

    #[test]
    fn multiply_scalar() {
        assert_eq!(Volt(10.0) * 5.0, Volt(50.0));
    }

    #[test]
    fn multiply_units() {
        assert_eq!(Amp(10.0) * Ohm(2.0), Volt(20.0));
        assert_eq!(Ohm(10.0) * Amp(2.0), Volt(20.0));
    }

    #[test]
    fn add() {
        assert_eq!(Volt(1.0) + Volt(2.0), Volt(3.0));
    }

    #[test]
    fn add_assign() {
        let mut volts = Volt(1.0);
        volts += Volt(0.5);
        assert_eq!(Volt(1.5), volts);
    }

    #[test]
    fn subtract() {
        assert_eq!(Volt(3.0) - Volt(1.0), Volt(2.0));
    }

    #[test]
    fn subtract_assign() {
        let mut volts = Volt(1.0);
        volts -= Volt(0.5);
        assert_eq!(Volt(0.5), volts);
    }

    #[test]
    fn convert() {
        assert!(Degree(180.0) - Radian(PI).into() < Degree(0.000001));
        assert!(Radian(PI) - Degree(180.0).into() < Radian(0.000001));
    }

    #[test]
    fn use_case_accel() {
        unit!(Meter, "m");
        unit!(Second, "s");
        unit!(MetersPerSecond, "m/s");
        unit!(MetersPerSecond2, "m/s²");

        divide_units!(Meter, Second, MetersPerSecond);
        divide_units!(MetersPerSecond, Second, MetersPerSecond2);

        let m = Meter(9.8);
        let s1 = Second(1.0);
        let s2 = Second(10.0);

        let a = m / s1 / s2;

        assert_eq!(MetersPerSecond2(0.98), a);
    }

    #[test]
    fn use_case_distance() {
        unit!(Meter, "m");
        unit!(Millimeter, "mm");

        convert_units!(Meter, Millimeter, 1000.0);
        convert_units!(Millimeter, Meter, 0.001);

        let meters = Meter(2.0);
        let millimeters: Millimeter = meters.into();

        assert_eq!(Millimeter(2000.0), millimeters);
        assert_eq!(Meter(2.0), millimeters.into());
    }
}
