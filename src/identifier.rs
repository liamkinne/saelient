use embedded_can::{ExtendedId, Id, StandardId};

pub trait SourceAddress {
    /// Message source address.
    fn source_address(&self) -> u8;
}

impl SourceAddress for Id {
    fn source_address(&self) -> u8 {
        match self {
            Id::Standard(id) => id.source_address(),
            Id::Extended(id) => id.source_address(),
        }
    }
}

pub trait Priority {
    /// Message priority.
    fn priority(&self) -> u8;
}

impl Priority for Id {
    fn priority(&self) -> u8 {
        match self {
            Id::Standard(id) => id.priority(),
            Id::Extended(id) => id.priority(),
        }
    }
}

/// Standard 11-bit identifier.
pub trait Standard {}

impl SourceAddress for StandardId {
    fn source_address(&self) -> u8 {
        (self.as_raw() & 0xFF) as u8
    }
}

impl Priority for StandardId {
    fn priority(&self) -> u8 {
        ((self.as_raw() >> 8) & 0b111) as u8
    }
}

pub trait Extended {
    /// Destination address.
    fn destination_address(&self) -> Option<u8>;

    /// Group extension.
    fn group_extension(&self) -> Option<u8>;

    /// PDU specific.
    fn pdu_specific(&self) -> u8;

    /// PDU format.
    fn pdu_format(&self) -> u8;

    /// Data page.
    fn data_page(&self) -> bool;

    /// Extended data page.
    fn extended_data_page(&self) -> bool;
}

impl Extended for ExtendedId {
    fn destination_address(&self) -> Option<u8> {
        let value = self.pdu_format();

        match value {
            0..=239 => Some(value),
            240..=255 => None,
        }
    }

    fn group_extension(&self) -> Option<u8> {
        let value = self.pdu_format();

        match value {
            0..=239 => None,
            240..=255 => Some(value),
        }
    }

    fn pdu_specific(&self) -> u8 {
        ((self.as_raw() >> 8) & 0xFF) as u8
    }

    fn pdu_format(&self) -> u8 {
        ((self.as_raw() >> 16) & 0xFF) as u8
    }

    fn data_page(&self) -> bool {
        ((self.as_raw() >> 24) & 1) == 1
    }

    fn extended_data_page(&self) -> bool {
        ((self.as_raw() >> 25) & 1) == 1
    }
}

impl SourceAddress for ExtendedId {
    fn source_address(&self) -> u8 {
        (self.as_raw() & 0xFF) as u8
    }
}

impl Priority for ExtendedId {
    fn priority(&self) -> u8 {
        ((self.as_raw() >> 26) & 0b111) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        let id = Id::Standard(StandardId::new(0x755).unwrap());
        assert_eq!(id.priority(), 0b111);

        let id = Id::Standard(StandardId::new(0x555).unwrap());
        assert_eq!(id.priority(), 0b101);

        let id = Id::Extended(ExtendedId::new(0x1FFF_FFFF).unwrap());
        assert_eq!(id.priority(), 0b111);

        let id = Id::Extended(ExtendedId::new(0x17FF_FFFF).unwrap());
        assert_eq!(id.priority(), 0b101);
    }

    #[test]
    fn std_source_address() {
        let standard = StandardId::new(0x755).unwrap();
        assert_eq!(standard.source_address(), 0x55);
    }

    #[test]
    fn std_priority() {
        let standard = StandardId::new(0x755).unwrap();
        assert_eq!(standard.priority(), 0b111);
    }

    #[test]
    fn ext_source_address() {
        let extended = ExtendedId::new(0x7665544).unwrap();
        assert_eq!(extended.source_address(), 0x44);
    }

    #[test]
    fn ext_priority() {
        let extended = ExtendedId::new(0x1FFF_FFFF).unwrap();
        assert_eq!(extended.priority(), 0b111);
    }

    #[test]
    fn ext_pdu_specific() {
        let extended = ExtendedId::new(0x7665544).unwrap();
        assert_eq!(extended.pdu_specific(), 0x55);
    }

    #[test]
    fn ext_pdu_format() {
        let extended = ExtendedId::new(0x7665544).unwrap();
        assert_eq!(extended.pdu_format(), 0x66);
    }

    #[test]
    fn ext_data_page() {
        let extended = ExtendedId::new(0x1FFF_FFFF).unwrap();
        assert_eq!(extended.data_page(), true);
    }

    #[test]
    fn ext_extended_data_page() {
        let extended = ExtendedId::new(0x1FFF_FFFF).unwrap();
        assert_eq!(extended.extended_data_page(), true);
    }
}
