//! Lab 07 — validate BIP39 recovery words and derive seeds safely.

use bip39::Mnemonic;

use crate::error::LabError;
use crate::model::{MnemonicReport, PassphraseComparison};
use crate::LabResult;

const PUBLIC_TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

fn parse_mnemonic(mnemonic: &str) -> LabResult<Mnemonic> {
    mnemonic
        .parse::<Mnemonic>()
        .map_err(|e| LabError::InvalidMnemonic(e.to_string()))
}

/// Validate an English mnemonic and report its entropy/checksum structure.
pub fn inspect_mnemonic(mnemonic: &str) -> LabResult<MnemonicReport> {
    let m = parse_mnemonic(mnemonic)?;
    let word_count = m.word_count();
    // ENT bits = word_count * 11 * 32 / 33
    let entropy_bits = word_count * 11 * 32 / 33;
    let checksum_bits = entropy_bits / 32;
    Ok(MnemonicReport {
        word_count,
        entropy_bits,
        checksum_bits,
    })
}

/// Derive the 512-bit BIP39 seed from words plus an optional passphrase.
pub fn mnemonic_seed_hex(mnemonic: &str, passphrase: &str) -> LabResult<String> {
    let m = parse_mnemonic(mnemonic)?;
    let seed = m.to_seed(passphrase);
    Ok(hex::encode(seed))
}

/// Demonstrate that the same words with a different passphrase make a different seed.
pub fn compare_passphrases(
    mnemonic: &str,
    protected_passphrase: &str,
) -> LabResult<PassphraseComparison> {
    let empty_seed = mnemonic_seed_hex(mnemonic, "")?;
    let protected_seed = mnemonic_seed_hex(mnemonic, protected_passphrase)?;
    Ok(PassphraseComparison {
        seeds_differ: empty_seed != protected_seed,
        empty_passphrase_seed_hex: empty_seed,
        protected_seed_hex: protected_seed,
    })
}

/// Recognize the public BIP39 test mnemonic used in the class labs.
pub fn is_public_test_mnemonic(mnemonic: &str) -> bool {
    mnemonic.split_whitespace().collect::<Vec<_>>().join(" ") == PUBLIC_TEST_MNEMONIC
}
