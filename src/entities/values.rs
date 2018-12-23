macro_rules! declare_values {
    ($($value:ident => $format:expr),*) => {
        use std::io;
        use std::str::FromStr;
        use std::fmt;

        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub enum Value {
            $($value),*,

            None
        }

        pub const VALUES: [Value; Value::None as usize] = [
            $(Value::$value),*
        ];

        impl Default for Value {
            fn default() -> Self {
                Value::None
            }
        }

        impl FromStr for Value {
            type Err = io::Error;

            fn from_str(input: &str) -> io::Result<Self> {
                Ok(match input {
                    $($format => Value::$value),*,
                    "0" | " " => Value::None,
                    _ => {
                        return Err(io::ErrorKind::InvalidInput.into());
                    }
                })
            }
        }

        impl fmt::Display for Value {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", match *self {
                    $(Value::$value => $format),*,
                    Value::None => "0"
                })
            }
        }
    }
}

declare_values! {
    One => "1",
    Two => "2",
    Three => "3",
    Four => "4",
    Five => "5",
    Six => "6",
    Seven => "7",
    Eight => "8",
    Nine => "9"
}
