use defmt::Format;
use bitfield_struct::{bitfield, FromBits, IntoBits};
use num_enum::FromPrimitive;
use num_enum_derive::IntoPrimitive;
use crate::sx1280::commands::{NullArgumentsBufferType, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280Mode;

pub struct GetStatusCommand;

#[derive(Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum CommandStatus {
    CommandOk = 1,
    DataAvailable = 2,
    CommandTimeout = 3,
    CommandProcessError = 4,
    CommandExecutionError = 5,
    TxDone = 6,
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum CircuitMode {
    StandbyRC = 2,
    StandbyXOSC = 3,
    FrequencySynthesis = 4,
    Reception = 5,
    Transmission = 6,
    #[num_enum(default)]
    Unknown,
}

#[bitfield(u8, defmt=true)]
pub struct Status {
    #[bits(2)]
    _pad: u8,
    #[bits(3)]
    command_status: CommandStatus,
    #[bits(3)]
    circuit_mode: CircuitMode,
}

impl<MODE: SX1280Mode> SX1280Command<MODE> for GetStatusCommand {
    const OPCODE: u8 = 0xC0;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = Status;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([0; 0])
    }
}

impl TryFrom<(u8, [u8; 0])> for Status {
    type Error = SX1280CommandError;

    fn try_from(value: (u8, [u8; 0])) -> Result<Self, Self::Error> {
        Ok(Self(value.0))
    }
}