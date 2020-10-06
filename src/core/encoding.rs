/* $Id: blast_encoding.c 500404 2016-05-04 14:59:01Z camacho $ */

/// Size of compressed nucleic acid alphabet
pub(crate) const BLAST2NA_SIZE: usize = 4;
/// Size of nucleic acid alphabet
pub(crate) const BLASTNA_SIZE: usize = 16;
/// Size of aminoacid alphabet
pub(crate) const BLASTAA_SIZE: usize = 28;

/// Identifies the blastna alphabet, for use in blast only
pub(crate) const BLASTNA_SEQ_CODE: u8 = 99;
/// Seq_code_ncbistdaa
pub(crate) const BLASTAA_SEQ_CODE: u8 = 11;
/// Seq_code_ncbi4na
pub(crate) const NCBI4NA_SEQ_CODE: u8 = 4;

// /** Sentinel byte for protein sequences */
// NCBI_XBLAST_EXPORT extern pub(crate) const Uint1 kProtSentinel;
// /** Sentinel nibble for nucleotide sequences */
// NCBI_XBLAST_EXPORT extern pub(crate) const Uint1 kNuclSentinel;

pub(crate) const NCBI4NA_TO_BLASTNA: [u8; BLASTNA_SIZE] = [
    15, // Gap, 0
    0,  // A,   1
    1,  // C,   2
    6,  // M,   3
    2,  // G,   4
    4,  // R,   5
    9,  // S,   6
    13, // V,   7
    3,  // T,   8
    8,  // W,   9
    5,  // Y,   A
    12, // H,   B
    7,  // K,   C
    11, // D,   D
    10, // B,   E
    14, // N,   F
];

pub(crate) const BLASTNA_TO_NCBI4NA: [u8; BLASTNA_SIZE] = [
    1,  // A,   0
    2,  // C,   1
    4,  // G,   2
    8,  // T,   3
    5,  // R,   4
    10, // Y,   5
    3,  // M,   6
    12, // K,   7
    9,  // W,   8
    6,  // S,   9
    14, // B,   A
    13, // D,   B
    11, // H,   C
    7,  // V,   D
    15, // N,   E
    0,  // Gap, F
];

pub(crate) const BLASTNA_TO_IUPACNA: [char; BLASTNA_SIZE] = [
    'A', 'C', 'G', 'T', 'R', 'Y', 'M', 'K', 'W', 'S', 'B', 'D', 'H', 'V', 'N', '-',
];

pub(crate) const NCBI4NA_TO_IUPACNA: [char; BLASTNA_SIZE] = [
    '-', 'A', 'C', 'M', 'G', 'R', 'S', 'V', 'T', 'W', 'Y', 'H', 'K', 'D', 'B', 'N',
];

#[rustfmt::skip]
pub(crate) const IUPACNA_TO_BLASTNA: [u8; 128]= [
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15, 0,10, 1,11,15,15, 2,12,15,15, 7,15, 6,14,15,
    15,15, 4, 9, 3,15,13, 8,15, 5,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,
    15,15,15,15,15,15,15,15,15,15,15,15,15,15,15,15];

#[rustfmt::skip]
pub(crate) const IUPACNA_TO_NCBI4NA: [u8; 128]= [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1,14, 2,13, 0, 0, 4,11, 0, 0,12, 0, 3,15, 0,
    0, 0, 5, 6, 8, 0, 7, 9, 0,10, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[rustfmt::skip]
pub(crate) const AMINOACID_TO_NCBISTDAA: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,25, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,27,10,11,12,13,26,
   14,15,16,17,18,24,19,20,21,22,23, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub(crate) const NCBISTDAA_TO_AMINOACID: [char; BLASTAA_SIZE] = [
    '-', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
    'V', 'W', 'X', 'Y', 'Z', 'U', '*', 'O', 'J',
];

#[allow(non_upper_case_globals)]
pub(crate) const kProtSentinel: u8 = b'\0';
pub(crate) const kNuclSentinel: u8 = 0xF;

/// Different types of sequence encodings for sequence
/// retrieval from the BLAST database
enum EBlastEncoding {
    BlastEncodingProtein = 0,    // NCBIstdaa
    BlastEncodingNucleotide = 1, // Special encoding for preliminary stage of BLAST: permutation of NCBI4na. A.k.a.: BLASTNA encoding
    BlastEncodingNcbi4na = 2,    // NCBI4na
    BlastEncodingNcbi2na = 3,    // NCBI2na
    BlastEncodingError = 255,    // Error value for encoding
}
