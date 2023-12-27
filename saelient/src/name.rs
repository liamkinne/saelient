//! Network management.

use bitfield::bitfield;
bitfield! {
    /// NAME Field
    ///
    /// See SAEJ1939-81 4.2.1.1
    struct Fields(u64);
    impl Debug;
    /// Identity Number.
    #[inline]
    u32, identity, set_identity: 20, 0;
    /// Manufacturer Code.
    #[inline]
    u16, manufacturer_code, set_manufacturer_code: 31, 21;
    /// ECU Instance
    #[inline]
    u8, ecu_instance, set_ecu_instance: 34, 32;
    /// Function Instance.
    #[inline]
    u8, function_instance, set_function_instance: 39, 35;
    /// Function
    #[inline]
    u8, function, set_function: 47, 40;
    /// Vehicle System Field.
    #[inline]
    u8, vehicle_system, set_vehicle_system: 56, 49;
    /// Vehicle System Instance.
    #[inline]
    u8, vehicle_system_instance, set_vehicle_system_instance: 59, 56;
    /// Industry Group.
    #[inline]
    u8, industry_group, set_industry_group: 62, 60;
    /// Arbitrary Address Capable.
    #[inline]
    u8, arbitrary_address_capable, set_arbitrary_address_capable: 63, 63;
}

/// NAME Definition
pub struct Name(Fields);

impl Name {
    pub fn new(
        identity: u32,
        manufacturer_code: u16,
        ecu_instance: u8,
        function_instance: u8,
        function: u8,
        vehicle_system: u8,
        vehicle_system_instance: u8,
        industry_group: IndustryGroup,
        arbitrary_address_capable: bool,
    ) -> Option<Self> {
        let mut fields = Fields(0);

        if identity < 2u32.pow(21) {
            fields.set_identity(identity)
        } else {
            return None;
        }

        if manufacturer_code < 2u16.pow(11) {
            fields.set_manufacturer_code(manufacturer_code)
        } else {
            return None;
        }

        if ecu_instance < 2u8.pow(3) {
            fields.set_ecu_instance(ecu_instance)
        } else {
            return None;
        }

        if function_instance < 2u8.pow(5) {
            fields.set_function_instance(function_instance)
        } else {
            return None;
        }

        if function < 2u8.pow(8) {
            fields.set_function(function)
        } else {
            return None;
        }

        if vehicle_system < 2u8.pow(7) {
            fields.set_vehicle_system(vehicle_system)
        } else {
            return None;
        }

        if vehicle_system_instance < 2u8.pow(4) {
            fields.set_vehicle_system_instance(vehicle_system_instance)
        } else {
            return None;
        }

        let industry_group = industry_group as u8;
        if industry_group < 2u8.pow(3) {
            fields.set_industry_group(industry_group)
        } else {
            return None;
        }

        fields.set_arbitrary_address_capable(arbitrary_address_capable as u8);

        Some(Self(fields))
    }

    /// Returns the bytes of the NAME data in platform native byte order.
    pub fn as_raw(&self) -> [u8; 8] {
        self.0 .0.to_ne_bytes()
    }

    /// A 21-bit field assigned by the manufacturer and should be unique across units.
    ///
    /// The interpretation of this number is generally not significant other than the necessity for it to be unique.
    pub fn identity(&self) -> u32 {
        self.0.identity()
    }

    /// An 11-bit field indicating the company that created the electronic control module.
    pub fn manufacturer_code(&self) -> u16 {
        self.0.manufacturer_code()
    }

    /// A 3-bit field that indicates the which of a group of electronic control modules is being referenced.
    pub fn ecu_instance(&self) -> u8 {
        self.0.ecu_instance()
    }

    /// A 5-bit field identifying the instance of a function on the same vehicle system for a given network.
    pub fn function_instance(&self) -> u8 {
        self.0.function_instance()
    }

    /// An 8-bit field defining the function performed by the controller application.
    pub fn function(&self) -> u8 {
        self.0.function()
    }

    /// A 7-bit field identifying the vehicle system or system group the controller application belongs to.
    pub fn vehicle_system(&self) -> u8 {
        self.0.vehicle_system()
    }

    /// A 4-bit field identifying the instance of a vehicle system within a given network.
    pub fn vehicle_system_instance(&self) -> u8 {
        self.0.vehicle_system_instance()
    }

    /// A 3-bit field specifying which industry group the NAME function is associated with.
    ///
    /// This is essentially multi-plexing for the function field.
    pub fn industry_group(&self) -> Option<IndustryGroup> {
        IndustryGroup::try_from(self.0.industry_group()).ok()
    }

    /// A 1-bit fied indicating whether a controller application supports using an arbitrary source address to resolve an address claim conflict.
    pub fn arbitrary_address_capable(&self) -> bool {
        self.0.arbitrary_address_capable() == 1
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
    const EXAMPLE: Fields = Fields(u64::from_le_bytes([
        0b00001110, 0b10011001, 0b10100010, 0b00000011, 0b00011001, 0b10000000,
        0b00001000, 0b10110010,
    ]));

    #[test]
    fn example_identity() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.identity(), 170254);
    }

    #[test]
    fn example_manufacturer() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.manufacturer_code(), 29);
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
        assert_eq!(EXAMPLE.function(), 128);
    }

    #[test]
    fn example_vehicle_system() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.vehicle_system(), 4);
    }

    #[test]
    fn example_vehicle_system_instance() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.vehicle_system_instance(), 2);
    }

    #[test]
    fn example_industry_group() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.industry_group(), 3);
    }

    #[test]
    fn example_arbitrary_address_capable() {
        // see J1939-81 4.2.1.1
        assert_eq!(EXAMPLE.arbitrary_address_capable(), 1);
    }
}
