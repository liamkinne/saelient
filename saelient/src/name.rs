//! Network management.

struct Name([u8; 8]);

impl Name {
    /// Identity number.
    pub fn identity(&self) -> u32 {
        // first 21 bits
        ((self.0[2] as u32) & 0b0001_1111) << 16
            | (self.0[1] as u32) << 8
            | self.0[0] as u32
    }

    /// Manufacturer code.
    pub fn manufacturer(&self) -> u16 {
        // bits 22 to 31
        ((self.0[2] as u16) >> 5) | ((self.0[3] as u16) << 3)
    }

    /// ECU instance.
    pub fn ecu_instance(&self) -> u8 {
        // bits 32 to 34
        self.0[4] & 0b0000_0111
    }

    /// Function instance.
    pub fn function_instance(&self) -> u8 {
        // bits 34 to 40
        (self.0[4]) >> 3 & 0b0011_1111
    }

    /// Function.
    pub fn function(&self) -> Option<u8> {
        Some(self.0[5])
    }

    /// Vehicle system.
    pub fn vehicle_system(&self) -> Option<u8> {
        Some((self.0[6] >> 1) & 0b0111_1111)
    }

    /// Vehicle system instance.
    pub fn vehicle_system_instance(&self) -> u8 {
        self.0[7] & 0b0000_1111
    }

    /// Industry group.
    pub fn industry_group(&self) -> Option<IndustryGroup> {
        IndustryGroup::try_from((self.0[7] >> 4) & 0b0000_0111).ok()
    }

    /// Arbitrary address capable.
    pub fn arbitrary_address_capable(&self) -> bool {
        (self.0[7] >> 7) == 1
    }
}

/// Industry group assignment.
#[derive(Debug, PartialEq, Eq)]
pub enum IndustryGroup {
    /// Global, applies to all.
    Global = 0,
    /// On-highway equipment.
    OnHighway = 1,
    /// Agricultural and forestry equipment.
    AgriculturalAndForestry = 2,
    /// Construction equipment.
    Construction = 3,
    /// Marine equipment.
    Marine = 4,
    /// Industrial-process control-stationary (gen-sets)
    IndustrialProcess = 5,
    // 6 = reserved
    // 7 = reserved
}

impl TryFrom<u8> for IndustryGroup {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == IndustryGroup::Global as u8 => Ok(IndustryGroup::Global),
            x if x == IndustryGroup::OnHighway as u8 => {
                Ok(IndustryGroup::OnHighway)
            }
            x if x == IndustryGroup::AgriculturalAndForestry as u8 => {
                Ok(IndustryGroup::AgriculturalAndForestry)
            }
            x if x == IndustryGroup::Construction as u8 => {
                Ok(IndustryGroup::Construction)
            }
            x if x == IndustryGroup::Marine as u8 => Ok(IndustryGroup::Marine),
            x if x == IndustryGroup::IndustrialProcess as u8 => {
                Ok(IndustryGroup::IndustrialProcess)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // example taken from J1939-81 4.2.1.1
    const EXAMPLE: Name = Name([
        0b00001110, 0b10011001, 0b10100010, 0b00000011, 0b00011001, 0b10000000,
        0b00001000, 0b10110010,
    ]);

    #[test]
    fn example_identity() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.identity(), 170254);
    }

    #[test]
    fn example_manufacturer() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.manufacturer(), 29);
    }

    #[test]
    fn example_ecu_instance() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.ecu_instance(), 1);
    }

    #[test]
    fn example_function_instance() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.function_instance(), 3);
    }

    #[test]
    fn example_function() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.function(), Some(128));
    }

    #[test]
    fn example_vehicle_system() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.vehicle_system(), Some(4));
    }

    #[test]
    fn example_vehicle_system_instance() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.vehicle_system_instance(), 2);
    }

    #[test]
    fn example_industry_group() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.industry_group(), Some(IndustryGroup::Construction));
    }

    #[test]
    fn example_arbitrary_address_capable() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.arbitrary_address_capable(), true);
    }
}
