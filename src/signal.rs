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
