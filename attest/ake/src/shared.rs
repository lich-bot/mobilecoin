// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Common transitions between initiator and responder.

use crate::{
    event::{Ciphertext, NonceCiphertext, NoncePlaintext, Plaintext},
    mealy::Transition,
    state::Ready,
};
use alloc::vec::Vec;
use mc_crypto_noise::{CipherError, NoiseCipher};
use rand_core::{CryptoRng, RngCore};

/// Ready + Ciphertext => Ready + Vec-of-plaintext
impl<Cipher> Transition<Ready<Cipher>, Ciphertext<'_, '_>, Vec<u8>> for Ready<Cipher>
where
    Cipher: NoiseCipher,
{
    type Error = CipherError;

    fn try_next<R: CryptoRng + RngCore>(
        self,
        _csprng: &mut R,
        input: Ciphertext,
    ) -> Result<(Ready<Cipher>, Vec<u8>), Self::Error> {
        let mut retval = self;
        let plaintext = retval.decrypt(input.aad, input.msg)?;
        Ok((retval, plaintext))
    }
}

/// Ready + Plaintext => Ready + Vec-of-ciphertext
impl<Cipher> Transition<Ready<Cipher>, Plaintext<'_, '_>, Vec<u8>> for Ready<Cipher>
where
    Cipher: NoiseCipher,
{
    type Error = CipherError;

    fn try_next<R: CryptoRng + RngCore>(
        self,
        _csprng: &mut R,
        input: Plaintext,
    ) -> Result<(Ready<Cipher>, Vec<u8>), Self::Error> {
        let mut retval = self;
        let ciphertext = retval.encrypt(input.aad, input.msg)?;
        Ok((retval, ciphertext))
    }
}

/// Ready + NonceCiphertext => Ready + Vec
impl<Cipher> Transition<Ready<Cipher>, NonceCiphertext<'_, '_>, Vec<u8>> for Ready<Cipher>
where
    Cipher: NoiseCipher,
{
    type Error = CipherError;

    fn try_next<R: CryptoRng + RngCore>(
        self,
        _csprng: &mut R,
        input: NonceCiphertext<'_, '_>,
    ) -> Result<(Ready<Cipher>, Vec<u8>), Self::Error> {
        let mut retval = self;
        let plaintext =
            retval.decrypt_with_nonce(input.ciphertext.aad, input.ciphertext.msg, input.nonce)?;
        Ok((retval, plaintext))
    }
}

/// Ready + NoncePlaintext => Ready + (Vec, u64)
impl<Cipher> Transition<Ready<Cipher>, NoncePlaintext<'_, '_>, (Vec<u8>, u64)> for Ready<Cipher>
where
    Cipher: NoiseCipher,
{
    type Error = CipherError;

    fn try_next<R: CryptoRng + RngCore>(
        self,
        _csprng: &mut R,
        input: NoncePlaintext<'_, '_>,
    ) -> Result<(Ready<Cipher>, (Vec<u8>, u64)), Self::Error> {
        let mut retval = self;
        let output = retval.encrypt_with_nonce(input.aad(), input.msg())?;
        Ok((retval, output))
    }
}
