use crate::encoder::EncodingError::{
    CodewordBufferTooSmall, InvalidSymbolsUsed, UninitialisedCodewordBuffer,
};
use crate::{RsSymbol, SYMBOL_SIZE};

/// Reed-Solomon Error Correction encoder module.

// Maximum length of the codeword. Codeword buffers must be exactly this size for space efficiency
// reasons (the buffer for calculations must be this size so it is more efficient to use the
// codeword buffer for calculations than have both).
pub const MAX_MESSAGE_LEN: usize = (1 << SYMBOL_SIZE) - 1;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum EncodingError {
    CodewordBufferTooSmall, // not enough space allocated for the codeword buffer
    InvalidSymbolsUsed,     // symbol sizes used exceed maximum value (2^m - 1)
    UninitialisedCodewordBuffer, // Codeword buffer is not entirely empty
}

// pub type CodewordBuffer = [RsSymbol; MAX_LENGTH];

// I WANT TO AVOID HAVING TO ALLOCATE A FULL LENGTH ARRAY

// TODO: Rust API and C API with different arguments
// Design in Rust, then make C compatible API
// Could have it so the msg_buffer has enough space for the parity symbols as well so even less
// space required

/// Encode an RS message with specified parameters. Returns success or fail, output is made to a
/// passed out_buffer.
/// TODO: size types for k,t,m? They might just be able to be RsSymbol types due to maximum length
/// of RS codewords etc.
// #[no_mangle] pub extern fn rs_encode(
pub fn rs_encode(
    msg: &[RsSymbol],          // the message to be encoded
    t: usize,                  // number of parity symbols
    m: usize,                  // symbol size
    codeword: &mut [RsSymbol], // buffer to store the codeword into. Can set size requirements here
) -> core::result::Result<(), EncodingError> {
    /* Argument checking */

    // Check the out buffer has been allocated enough space to store the length of the codeword
    // assert!(codeword.len() >= msg.len() + t);
    if codeword.len() < msg.len() + t {
        return Err(CodewordBufferTooSmall);
    }

    // Check all the passed symbols are 0 <= 2^m - 1 (unsigned type so always >= 0)
    for symbol in msg {
        if *symbol >= 1 << m {
            return Err(InvalidSymbolsUsed);
        }
    }

    // Ensure the codeword buffer is empty
    for symbol in codeword {
        if *symbol != 0 {
            return Err(UninitialisedCodewordBuffer);
        }
    }

    /* Encoding process */
    let parity_start_pos = msg.len();

    // How much the message has been shortened compared to the maximum possible length
    let shortened = MAX_MESSAGE_LEN - msg.len();

    // success
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::encoder::{rs_encode, EncodingError, MAX_MESSAGE_LEN};
    use crate::RsSymbol;

    extern crate std;

    #[test]
    fn codeword_buffer_too_small() {
        let msg: [RsSymbol; 6] = [1, 2, 3, 4, 5, 6];
        let t = 2;
        let m = 4;

        let mut codeword_buffer = [0; 6];
        let result = rs_encode(&msg, t, m, &mut codeword_buffer);

        assert_eq!(result, Err(EncodingError::CodewordBufferTooSmall));

        // Test now redundant as buffer can /*never*/ be too small at compile time
    }

    #[test]
    fn symbol_sizes_too_large() {
        let msg: [RsSymbol; 6] = [16, 2, 3, 4, 5, 6];
        let t = 2;
        let m = 4;
        let mut codeword_buffer: [RsSymbol; 8] = [0; 8];

        let result = rs_encode(&msg, t, m, &mut codeword_buffer);

        assert_eq!(result, Err(EncodingError::InvalidSymbolsUsed));
    }
}
