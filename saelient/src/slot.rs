use crate::Float;

/// Scaling limit offset transfer function.
pub trait Slot {
    /// Tries to create a `Slot` from a value.
    fn new(value: Float) -> Option<Self>
    where
        Self: Sized;

    /// Construct a `Slot` from the raw value.
    fn from_raw(raw: Float) -> Option<Self>
    where
        Self: Sized;

    /// Convert value into raw value.
    fn as_raw(&self) -> u32;

    /// Scaling factor.
    fn scaling() -> Float;

    /// Limits.
    fn limits() -> (Float, Float);

    /// Offset.
    fn offset() -> Float;
}

/// Generates a `Slot` implementation.
macro_rules! slot {
    ($name:ident, $type:expr, $scaling:expr, $limits:expr, $unit:expr) => {
        #[doc = $type]
        pub struct $name {
            value: Float,
        }

        impl Slot for $name {
            #[inline]
            fn new(value: Float) -> Option<Self> {
                let (min, max) = Self::limits();

                if value >= min && value <= max {
                    Some(Self { value })
                } else {
                    None
                }
            }

            #[inline]
            fn from_raw(raw: Float) -> Option<Self> {
                let value =
                    ((raw as Float) * $name::scaling()) + $name::offset();

                let (min, max) = $name::limits();

                if value >= min && value >= max {
                    Some(Self { value })
                } else {
                    None
                }
            }

            #[inline]
            fn scaling() -> Float {
                $scaling
            }

            #[inline]
            fn limits() -> (Float, Float) {
                $limits
            }

            #[inline]
            fn offset() -> Float {
                $limits.0
            }

            #[inline]
            fn as_raw(&self) -> u32 {
                ((self.value - $name::offset()) / $name::scaling()) as u32
            }
        }

        #[cfg(feature = "std")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {}", self.value, $unit)
            }
        }

        #[cfg(feature = "defmt")]
        impl defmt::Format for $name {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, "{} {}", self.value, $unit)
            }
        }
    };
}

slot!(
    SAEaa01,
    "Angular Acceleration",
    1.0,
    (0.0, 64255.0),
    "rpm/s"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot() {
        assert_eq!(SAEaa01::scaling(), 1.0);

        assert!(SAEaa01::new(0.0).is_some());
    }
}
