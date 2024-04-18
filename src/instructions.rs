use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String
    }
}
#[derive(BorshDeserialize)]
pub struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();

        Ok(match variant {
            0 => Self::AddMovieReview{
                title: payload.title,
                description: payload.description,
                rating: payload.rating
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}

