use core::ops::RangeInclusive;
use std::convert::TryFrom;

/// Transmitted values for distrete parameters (measured).
#[derive(Debug, PartialEq)]
pub enum Parameter {
    Disabled = 0x0,
    Enabled = 0x1,
    IsError = 0x2,
    NotAvailable = 0x3,
}

impl Default for Parameter {
    fn default() -> Self {
        Self::NotAvailable
    }
}

impl Into<u8> for Parameter {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Parameter {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0x0 => Ok(Parameter::Disabled),
            0x1 => Ok(Parameter::Enabled),
            0x2 => Ok(Parameter::IsError),
            0x3 => Ok(Parameter::NotAvailable),
            _ => Err(()),
        }
    }
}

/// Transmitted values for control commands (status).
#[derive(Debug, PartialEq)]
pub enum Command {
    Disable = 0x0,
    Enable = 0x1,
    // 0x2 reserved
    NoAction = 0x3,
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Command {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0x0 => Ok(Command::Disable),
            0x1 => Ok(Command::Enable),
            0x3 => Ok(Command::NoAction),
            _ => Err(()),
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::NoAction
    }
}

pub trait Signal<T> {
    /// The size of this type in bites.
    const BITS: u8;
    /// The smallest and largest value that can be represented by this type.
    const VALID_RANGE: RangeInclusive<T>;

    /// Parameter-specific indicator value range.
    const SPECIFIC_RANGE: RangeInclusive<T>;

    /// Error indicator value range.
    const ERROR_RANGE: RangeInclusive<T>;

    /// Not available/requested indicator value range.
    const NOT_AVAILABLE_RANGE: RangeInclusive<T>;

    /// Value is within the valid range.
    fn is_valid(&self) -> bool;

    /// Value is within the specific indicator range.
    fn is_specific(&self) -> bool;

    /// Value is within the error indicator range.
    fn is_error(&self) -> bool;

    /// Valid is within the not available indicator range.
    fn is_not_available(&self) -> bool;
}

macro_rules! signal {
    ($bits:literal, $TYPE:ident, $valid:expr, $specific:expr, $error:expr, $notavailable:expr) => {
        paste::item! {
            pub struct [<U $bits>]($TYPE);

            impl Signal<$TYPE> for [<U $bits>] {
                const BITS: u8 = $bits;
                #[doc = stringify!($valid)]
                const VALID_RANGE: RangeInclusive<$TYPE> = $valid;
                const SPECIFIC_RANGE: RangeInclusive<$TYPE> = $specific;
                const ERROR_RANGE: RangeInclusive<$TYPE> = $error;
                const NOT_AVAILABLE_RANGE: RangeInclusive<$TYPE> = $notavailable;

                #[inline]
                fn is_valid(&self) -> bool {
                    Self::VALID_RANGE.contains(&self.0)
                }

                #[inline]
                fn is_specific(&self) -> bool {
                    Self::VALID_RANGE.contains(&self.0)
                }

                #[inline]
                fn is_error(&self) -> bool {
                    Self::VALID_RANGE.contains(&self.0)
                }

                #[inline]
                fn is_not_available(&self) -> bool {
                    Self::VALID_RANGE.contains(&self.0)
                }
            }
        }
    };
}

signal!(4, u8, (0x0..=0xA), (0xB..=0xB), (0xE..=0xE), (0xF..=0xF));
signal!(
    8,
    u8,
    (0x00..=0xFA),
    (0xFB..=0xFB),
    (0xFE..=0xFE),
    (0xFF..=0xFF)
);
signal!(
    10,
    u16,
    (0x000..=0x3FA),
    (0x3FB..=0x3FB),
    (0x3FE..=0x3FE),
    (0x3FF..=0x3FF)
);
signal!(
    12,
    u16,
    (0x000..=0xFAF),
    (0xFB0..=0xFBF),
    (0xFE0..=0xFEF),
    (0xFF0..=0xFFF)
);
signal!(
    16,
    u16,
    (0x0000..=0xFAFF),
    (0xFB00..=0xFBFF),
    (0xFE00..=0xFEFF),
    (0xFF00..=0xFFFF)
);
signal!(
    20,
    u32,
    (0x00000..=0xFAFFF),
    (0xFB000..=0xFBFFF),
    (0xFE000..=0xFEFFF),
    (0xFF000..=0xFFFFF)
);
signal!(
    24,
    u32,
    (0x000000..=0xFAFFFF),
    (0xFB0000..=0xFBFFFF),
    (0xFE0000..=0xFEFFFF),
    (0xFF0000..=0xFFFFFF)
);
signal!(
    28,
    u32,
    (0x0000000..=0xFAFFFFF),
    (0xFB00000..=0xFBFFFFF),
    (0xFE00000..=0xFEFFFFF),
    (0xFF00000..=0xFFFFFFF)
);
signal!(
    32,
    u32,
    (0x00000000..=0xFAFFFFFF),
    (0xFB000000..=0xFBFFFFFF),
    (0xFE000000..=0xFEFFFFFF),
    (0xFF000000..=0xFFFFFFFF)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_failed_conversion() {
        let parameter = Parameter::try_from(99);

        assert_eq!(parameter, Err(()))
    }

    #[test]
    fn cmd_failed_conversion() {
        let command = Command::try_from(99);

        assert_eq!(command, Err(()))
    }
}
