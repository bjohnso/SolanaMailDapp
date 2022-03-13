use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MailError {
    #[error("Invalid Instructions")]
    InvalidInstruction,
    #[error("Account Is Not Writable")]
    NotWritable,
}

impl From<MailError> for ProgramError {
    fn from(error: MailError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
