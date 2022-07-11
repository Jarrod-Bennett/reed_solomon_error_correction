// #![no_std]

/// A simple and lightweight implementation of Reed-Solomon encoding and decoding for use on
/// resource constrained devices. Many of the design decisions trade increased space complexity for
/// cheaper calculations. This crate is no_std.
///
/// You can read about Reed-Solomon codes at
/// https://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction
///
/// Limitations and supported options:
/// Up to a maximum of 8-bit symbols is (theoretically) supported as 8-bit integers are used for
/// space efficiency. In practice, only up to 5-bit symbols are supported for space efficiency with
/// up to 8 parity symbols (though this number is more flexible and can be reconfigured by altering
/// the corresponding type with very little space consequences)
///
/// One design decision to save maximum space is for the passed codeword buffer to be larger than
/// theoretically necessary to allow for it to be used for the encoding process. This means an
/// additional buffer (which would have to be MAX_LENGTH anyway) does not need to be created.
///
/// This implementation is based off of the MATLAB rsenc() function. Many
/// components including the generator polynomial are fixed and have been
/// precomputed via a MATLAB script in order to reduce encoding complexity.
/// The API does not support configurable generator polynomials or similar.
/// See https://au.mathworks.com/help/comm/ref/rsenc.html for details.
///
/// @author Jarrod Bennett
mod decoder;
pub mod encoder;

/// Base type used for Reed-Solomon symbols. Should be the smallest type to support your symbol size
/// in order to save space.
type RsSymbol = u8;
pub const SYMBOL_SIZE: u8 = 4;

// Remaining work
// TODO: extern C, no_mangle etc.
// pub mod or mod?
// update comment to be for entire crate not just the decoder module
// comme of memory space with different parameters
// TODO: create example C project that calls these functions
// TODO: Features for only encoder or only decoder, individual headers for each for c lib
