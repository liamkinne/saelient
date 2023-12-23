use crate::identifier::Extended;
use embedded_can::ExtendedId;

enum MessageError {
    TooManyControlBytes,
}

enum Message<'a> {
    Command {},
    Request {
        source: u8,
        format: u8,
        destination: u8,
    },
    Response {},
    Ack(Ack<'a>),
    Group {},
}

enum AckKind {
    /// Positive acknowledgement.
    Positive,
    /// Negative acknowledgement.
    Negative,
    /// Accesss denied.
    AccessDenied,
    /// Cannot respond.
    CannotRespond,
    /// Don't care/take no action.
    NoAction,
}

struct Ack<'a> {
    kind: AckKind,
    control_byte: u8,
    group_function_value: &'a [u8],
    address_acknowledged: u8,
    pgn_of_request: u32,
}

impl<'a> Ack<'a> {
    fn new_positive(
        address_acknowledged: u8,
        pgn_of_request: u32,
        group_function_value: &'a [u8],
    ) -> Result<Self, MessageError> {
        let control_byte = match group_function_value.len() {
            0 => 0,
            1 => 128,
            2 => 144,
            3 => 160,
            _ => return Err(MessageError::TooManyControlBytes),
        };

        Ok(Self {
            kind: AckKind::Positive,
            control_byte,
            group_function_value,
            address_acknowledged,
            pgn_of_request,
        })
    }
}
