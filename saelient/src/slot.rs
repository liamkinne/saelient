/// Scaling limit offset transfer function.
struct Slot {
    scaling: f64,
    offset: f64,
    range_maximum: f64,
    bits: u16,
    #[cfg(feature = "slot-units")]
    unit: &'static str,
}

impl Slot {
    /// Scaling factor.
    pub fn scaling_factor(&self) -> f64 {
        self.scaling
    }

    /// Offset.
    pub fn offset(&self) -> f64 {
        self.offset
    }

    /// Minimum value.
    pub fn min(&self) -> f64 {
        // need to include offsets
        self.offset - self.range_maximum
    }

    /// Maximum value.
    pub fn max(&self) -> f64 {
        self.range_maximum
    }

    /// Convert wire encoded data into a real value.
    pub fn deserialize(&self, data: u32) -> Result<f64, &'static str> {
        let value = ((data as f64) * self.scaling) + self.offset;

        if value < self.min() || value > self.max() {
            Err("Value outside of bounds")
        } else {
            Ok(value)
        }
    }

    pub fn serialize(&self, value: f64) -> Result<u32, &'static str> {
        if value < self.min() || value > self.max() {
            return Err("Value outside of bounds");
        }

        Ok(((value - self.offset) / self.scaling) as u32)
    }
}

/// Angular Acceleration
#[allow(non_snake_case)]
const SAEaa01: Slot = Slot {
    scaling: 1.0,
    offset: 0.0,
    range_maximum: 64255.0,
    bits: 16,
    #[cfg(feature = "slot-units")]
    unit: "rpm/s",
};
