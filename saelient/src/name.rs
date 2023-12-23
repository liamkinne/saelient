trait NAME {
    /// Identity number.
    fn identity(&self) -> u32;

    /// Manufacturer code.
    fn manufacturer(&self) -> u16;

    /// ECU instance.
    fn ecu_instance(&self) -> u8;

    /// Function instance.
    fn function_instance(&self) -> u8;

    /// Function.
    fn function(&self) -> Option<u8>;

    /// Vehicle system.
    fn vehicle_system(&self) -> Option<u8>;

    /// Vehicle system instance.
    fn vehicle_system_instance(&self) -> u8;

    /// Industry group.
    fn industry_group(&self) -> IndustryGroup;

    /// Arbitrary address capable.
    fn arbitrary_address_capable(&self) -> bool;
}

/// Industry group assignment.
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
