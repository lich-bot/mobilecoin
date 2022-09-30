// Copyright (c) 2018-2022 The MobileCoin Foundation

//! Multi-signature implementation: A multi-signature is a protocol that allows
//! a group of signers, each possessing a distinct private/public keypair, to
//! produce a joint signature on a common message. The simplest multi-signature
//! of a message is just a set of signatures containing one signature over the
//! message from each member of the signing group. We say that a multi-signature
//! is a m-of-n threshold signature if only k valid signatures are required from
//! a signing group of size n.

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

extern crate alloc;

use alloc::vec::Vec;
use core::hash::Hash;
use mc_crypto_digestible::Digestible;
use mc_crypto_keys::{Ed25519Signature, PublicKey, Signature, SignatureError, Verifier};
use prost::Message;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// The maximum number of signatures that can be included in a multi-signature.
pub const MAX_SIGNATURES: usize = 10;

/*
/// A marker trait for obejcts that can sign a multi-sig.
pub trait Signer:
    Clone
    + DeserializeOwned
    + Default
    + Digestible
    + Eq
    + Hash
    + Message
    + Ord
    + PartialEq
    + PartialOrd
    + Serialize
{
}

impl<T> Signer for T where
    T: Clone
        + DeserializeOwned
        + Default
        + Digestible
        + Eq
        + Hash
        + Message
        + Ord
        + PartialEq
        + PartialOrd
        + Serialize
{
}

/// A marker trait for a signature that can be part of a multi-sig.
/// We do not use `Signature` directly because we are unable to implement
/// concise byte representations and that trait requires we implement from_bytes
/// ans as_bytes.
pub trait Sig:
    Clone + Default + Digestible + Eq + Hash + Message + Ord + PartialEq + PartialOrd + Serialize
{
}

impl<T> Sig for T where
    T: Clone
        + Default
        + Digestible
        + Eq
        + Hash
        + Message
        + Ord
        + PartialEq
        + PartialOrd
        + Serialize
{
}

/// An object that can verify a multi-sig.
pub trait MultiSigVerifier<T: Sig> {
    /// Verify a multi-signature.
    fn verify(&self, message: &[u8], sig: &T) -> Result<(), SignatureError>;
}

/// Blanket implementation of MultiSigVerifier for any type that implements
/// Verifier.
*/

/*impl<T, S: Sig + Signature> MultiSigVerifier<S> for T
where
    T: Verifier<S>,
{
    fn verify(&self, message: &[u8], sig: &S) -> Result<(), SignatureError> {
        self.verify(message, sig)
    }
}
*/

/*impl MultiSigVerifier<MultiSig<Ed25519Signature>> for SignerSet<Ed25519Public> {
    /// Verify a multi-signature.
    fn verify(
        &self,
        message: &[u8],
        sig: &MultiSig<Ed25519Signature>,
    ) -> Result<(), SignatureError> {
        self.verify(message, sig).map(|_| ())
    }
}*/

/*impl<S: Sig + Signature, PK: Signer> MultiSigVerifier<MultiSig<S>> for SignerSet<PK>
where
    PK: Verifier<S>,
{
    /// Verify a multi-signature.
    fn verify(&self, message: &[u8], sig: &MultiSig<S>) -> Result<(), SignatureError> {
        self.verify(message, sig).map(|_| ())
    }
}*/
/*

impl<S: Sig + Signature, PK: Signer> MultiSigVerifier<MultiSig<MultiSig<S>>>
    for SignerSet<SignerSet<PK>>
where
    PK: Verifier<S>,
{
    /// Verify a multi-signature.
    fn verify(&self, message: &[u8], sig: &MultiSig<MultiSig<S>>) -> Result<(), SignatureError> {
        self.verify(message, sig).map(|_| ())
    }
}
*/

/*
impl<S: Sig + Signature, PK: Signer> MultiSigVerifier<MultiSig<S>> for SignerSet<PK>
where
    PK: MultiSigVerifier<MultiSig<S>>,
{
    /// Verify a multi-signature.
    fn verify(&self, message: &[u8], sig: &MultiSig<S>) -> Result<(), SignatureError> {
        self.verify(message, sig).map(|_| ())
    }
}*/

/*

/// A multi-signature: a collection of one or more signatures.
#[derive(
    Clone, Deserialize, Digestible, Eq, Hash, Message, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct MultiSig<S: Sig> {
    #[prost(message, repeated, tag = "1")]
    signatures: Vec<S>,
}

impl<S: Sig> MultiSig<S> {
    /// Construct a new multi-signature from a collection of signatures.
    pub fn new(signatures: Vec<S>) -> Self {
        Self { signatures }
    }

    /// Get signatures
    pub fn signatures(&self) -> &[S] {
        &self.signatures
    }
}

/// A set of M-out-of-N public keys.
#[derive(
    Clone, Deserialize, Digestible, Eq, Hash, Message, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(bound = "")]
pub struct SignerSet<P: Signer> {
    /// List of potential signers.
    #[prost(message, repeated, tag = "1")]
    signers: Vec<P>,

    /// Minimum number of signers required.
    #[prost(uint32, tag = "2")]
    threshold: u32,
}

/// TODO
#[derive(
    Clone, Deserialize, Digestible, Eq, Hash, Message, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct VecMultiSig<S: Sig> {
    /// TODO
    #[prost(message, repeated, tag = "1")]
    multisigs: Vec<MultiSig<S>>,
}

/// TODO
pub trait ThresholdVerifier<T: Sig> {
    /// TODO
    fn verify(&self, message: &[u8], sigs: &[T]) -> Result<(), SignatureError>;
}

impl<T: Sig + Signature, PK: Verifier<T>> ThresholdVerifier<T> for PK {
    fn verify(&self, message: &[u8], sigs: &[T]) -> Result<(), SignatureError> {
        todo!()
    }
}

impl<T: Sig+ Signature, P: Signer + Verifier<T>> ThresholdVerifier<T> for SignerSet<P> {
    fn verify(&self, message: &[u8], sigs: &[T]) -> Result<(), SignatureError> {
        todo!()
    }
}

impl<P: Signer> SignerSet<P> {
    /// Construct a new `SignerSet` from a list of public keys and threshold.
    pub fn new(signers: Vec<P>, threshold: u32) -> Self {
        Self { signers, threshold }
    }

    /// Get the list of potential signers.
    pub fn signers(&self) -> &[P] {
        &self.signers
    }

    /// Get the threshold.
    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// TODO
    pub fn verify2<S: Sig>(
        &self,
        message: &[u8],
        multi_sig: &MultiSig<S>,
    ) -> Result<Vec<P>, SignatureError>
    where
        P: MultiSigVerifier<S>,
    {
        self.signers[0].verify(message, &multi_sig.signatures[0]);
        todo!()
    }
*/
/*

    /// Verify a message against a multi-signature, returning the list of
    /// signers that signed it.
    pub fn verify<S: Sig>(
        &self,
        message: &[u8],
        multi_sig: &MultiSig<S>,
    ) -> Result<Vec<P>, SignatureError>
    where
        P: MultiSigVerifier<S>,
    {
        // If the signature contains less than the threshold number of signers or more
        // than the hardcoded limit, there's no point in trying.
        if multi_sig.signatures.len() < self.threshold as usize
            || multi_sig.signatures.len() > MAX_SIGNATURES
        {
            return Err(SignatureError::new());
        }

        // Sort and dedup the list of signers and signatures.
        // While the verification code below should be immune to duplicate signers or
        // signatures, the overhead of deduping them is negligible and being
        // extra-safe is a good idea.
        let mut potential_signers = self.signers.clone();
        potential_signers.sort();
        potential_signers.dedup();

        let mut signatures = multi_sig.signatures.clone();
        signatures.sort_by(|a, b| a.cmp(b));
        signatures.dedup();

        // See which signatures which match signers.
        let mut matched_signers = Vec::new();
        for signature in signatures.iter() {
            let matched_signer = potential_signers.iter().find_map(|signer| {
                signer
                    .verify(message, signature)
                    .ok()
                    .map(|_| signer.clone())
            });
            if let Some(matched_signer) = matched_signer {
                potential_signers.retain(|signer| signer != &matched_signer);
                matched_signers.push(matched_signer);
            }
        }

        // Did we pass the threshold of verified signatures?
        if matched_signers.len() < self.threshold as usize {
            return Err(SignatureError::new());
        }

        Ok(matched_signers)
    }
}
    */

trait Signer {}

#[derive(Clone, Default)]
struct SignerSet<P: Signer> {
    pub signer_set: Vec<P>,
    pub threshold: u8,
}
impl<P: Signer> SignerSet<P> {
    fn new(signer_set: Vec<P>, threshold: u8) -> Self {
        Self {
            signer_set,
            threshold,
        }
    }
}

//impl Signer for Ed2Pub {}
impl<T> Signer for T where T: PublicKey {}
impl<P: Signer> Signer for SignerSet<P> {}

trait Sig {}

#[derive(Default)]
struct MultiSig<S: Sig> {
    pub sigs: Vec<S>,
}

impl<T> Sig for T where T: Signature {}
impl<S: Sig> Sig for MultiSig<S> {}

trait MultiSigVerifier<S: Sig> {
    fn verify(&self, msg: &[u8], sig: &S) -> Result<(), ()> {
        todo!()
    }
}

//impl Verifier<Ed2Sig> for Ed2Pub {}
impl<S: Sig + Signature, T: Verifier<S>> MultiSigVerifier<S> for T {
    fn verify(&self, msg: &[u8], sig: &S) -> Result<(), ()> {
        self.verify(msg, sig);
        todo!()
    }
}

impl<S: Sig, PK: Signer> MultiSigVerifier<MultiSig<S>> for SignerSet<PK>
where
    PK: MultiSigVerifier<S>,
{
    fn verify(&self, msg: &[u8], sig: &MultiSig<S>) -> Result<(), ()> {
        for signer in self.signer_set.iter() {
            signer.verify(msg, &sig.sigs[0])?;
        }
        Ok(())
    }
}

/// TODO
pub fn testz() {
    use alloc::vec;
    use mc_crypto_keys::{Ed25519Public, Ed25519Signature};

    let ss1 = SignerSet::new(vec![Ed25519Public::default()], 1);
    let ss2 = SignerSet::new(vec![Ed25519Public::default()], 1);

    let multi_ss = SignerSet::new(vec![ss1.clone(), ss2.clone()], 1);

    let ms1: MultiSig<Ed25519Signature> = MultiSig::default();
    ss1.verify(&[], &ms1).unwrap();

    let ms2: MultiSig<MultiSig<Ed25519Signature>> = MultiSig::default();
    let ss2: SignerSet<SignerSet<Ed25519Public>> = SignerSet::default();
    ss2.verify(&[], &ms2).unwrap();

    let ms3: MultiSig<MultiSig<MultiSig<Ed25519Signature>>> = MultiSig::default();
    let ss3: SignerSet<SignerSet<SignerSet<Ed25519Public>>> = SignerSet::default();
    ss3.verify(&[], &ms3).unwrap();
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use mc_crypto_keys::{Ed25519Pair, Ed25519Public, Signer};
    use mc_util_from_random::FromRandom;
    use rand_core::SeedableRng;
    use rand_hc::Hc128Rng;

    /// Helper method for comparing two signers list.
    /// In other places in the code we might convert to a HashSet first and then
    /// compare, but that would hide duplicate elements and we want to catch
    /// that.
    fn assert_eq_ignore_order(mut a: Vec<Ed25519Public>, mut b: Vec<Ed25519Public>) {
        a.sort();
        b.sort();

        assert_eq!(a, b);
    }

    #[test]
    fn ed25519_verify_signers_sanity_k_equals_3() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);
        let signer4 = Ed25519Pair::from_random(&mut rng);
        let signer5 = Ed25519Pair::from_random(&mut rng);

        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
            2,
        );
        let message = b"this is a test";

        // Try with just one valid signature, we should fail to verify.
        let multi_sig = MultiSig::new(vec![signer1.try_sign(message.as_ref()).unwrap()]);
        //assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        let signer21 = Ed25519Pair::from_random(&mut rng);
        let signer22 = Ed25519Pair::from_random(&mut rng);
        let signer23 = Ed25519Pair::from_random(&mut rng);
        let signer24 = Ed25519Pair::from_random(&mut rng);
        let signer25 = Ed25519Pair::from_random(&mut rng);

        let signer_set2 = SignerSet::new(
            vec![
                signer21.public_key(),
                signer22.public_key(),
                signer23.public_key(),
            ],
            1,
        );

        let multi_sig2 = MultiSig::new(vec![signer21.try_sign(message.as_ref()).unwrap()]);

        let ss2 = SignerSet::new(vec![signer_set, signer_set2], 1);
        let ms2 = MultiSig::new(vec![multi_sig, multi_sig2]);

        let ss3 = SignerSet::new(vec![ss2], 1);
        let ms3 = MultiSig::new(vec![ms2]);

        panic!("AAA {:?}", ss3.verify(message.as_ref(), &ms3));
    }
}
*/

/*
#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use mc_crypto_keys::{Ed25519Pair, Ed25519Public, Signer};
    use mc_util_from_random::FromRandom;
    use rand_core::SeedableRng;
    use rand_hc::Hc128Rng;

    /// Helper method for comparing two signers list.
    /// In other places in the code we might convert to a HashSet first and then
    /// compare, but that would hide duplicate elements and we want to catch
    /// that.
    fn assert_eq_ignore_order(mut a: Vec<Ed25519Public>, mut b: Vec<Ed25519Public>) {
        a.sort();
        b.sort();

        assert_eq!(a, b);
    }

    #[test]
    fn ed25519_verify_signers_sanity_k_equals_3() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);
        let signer4 = Ed25519Pair::from_random(&mut rng);
        let signer5 = Ed25519Pair::from_random(&mut rng);

        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
            2,
        );
        let message = b"this is a test";

        // Try with just one valid signature, we should fail to verify.
        let multi_sig = MultiSig::new(vec![signer1.try_sign(message.as_ref()).unwrap()]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        // With two valid signatures we should succeed to verify and get the correct
        // keys back.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer3.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer3.public_key()],
        );

        // If we alter the message, we should not pass verification.
        let message2 = b"different message";
        assert!(signer_set.verify(message2.as_ref(), &multi_sig).is_err());

        // With three valid signatures we should succeed to verify and get the correct
        // keys back.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
            signer3.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
        );

        // Trying to cheat by signing twice with the same signer will not work.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        // Using an unknown signer should not allow us to verify is we are under the
        // threshold
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        // Using an unknown signer does not get in the way of verifiying a valid set.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer3.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer3.public_key()],
        );

        // Bunch of duplicate signers and signatures, all do not match.
        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());
    }

    #[test]
    fn ed25519_verify_signers_sanity_k_equals_1() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);
        let signer4 = Ed25519Pair::from_random(&mut rng);
        let signer5 = Ed25519Pair::from_random(&mut rng);

        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
            1,
        );
        let message = b"this is a test";

        // Try with just no valid signatures, we should fail to verify.
        let multi_sig = MultiSig::new(vec![signer4.try_sign(message.as_ref()).unwrap()]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        // Add a valid signer, we should now verify successfully.
        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key()],
        );

        // With two valid signers we should get both back.
        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer2.public_key()],
        );

        // Add the same valid signers, they should not be returned twice.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer2.public_key()],
        );
    }

    #[test]
    fn ed25519_verify_with_duplicate_signers() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);
        let signer4 = Ed25519Pair::from_random(&mut rng);
        let signer5 = Ed25519Pair::from_random(&mut rng);

        // This signer set contains duplicate public keys but when verifying we should
        // not see the same key twice.
        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
                signer1.public_key(),
                signer2.public_key(),
            ],
            1,
        );
        let message = b"this is a test";

        // Try with just no valid signatures, we should fail to verify.
        let multi_sig = MultiSig::new(vec![signer4.try_sign(message.as_ref()).unwrap()]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
        ]);
        assert!(signer_set.verify(message.as_ref(), &multi_sig).is_err());

        // Add a valid signer, we should now verify successfully.
        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key()],
        );

        // With two valid signers we should get both back.
        let multi_sig = MultiSig::new(vec![
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer2.public_key()],
        );

        // Add the same valid signers, they should not be returned twice.
        let multi_sig = MultiSig::new(vec![
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
            signer4.try_sign(message.as_ref()).unwrap(),
            signer5.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
            signer1.try_sign(message.as_ref()).unwrap(),
            signer2.try_sign(message.as_ref()).unwrap(),
        ]);
        assert_eq_ignore_order(
            signer_set.verify(message.as_ref(), &multi_sig).unwrap(),
            vec![signer1.public_key(), signer2.public_key()],
        );
    }

    #[test]
    fn test_serde_works() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);

        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
            2,
        );

        assert_eq!(
            signer_set,
            mc_util_serial::deserialize(&mc_util_serial::serialize(&signer_set).unwrap()).unwrap(),
        );

        let message = b"this is a test";
        let multi_sig = MultiSig::new(vec![signer1.try_sign(message.as_ref()).unwrap()]);
        assert_eq!(
            multi_sig,
            mc_util_serial::deserialize(&mc_util_serial::serialize(&multi_sig).unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_prost_works() {
        let mut rng = Hc128Rng::from_seed([1u8; 32]);
        let signer1 = Ed25519Pair::from_random(&mut rng);
        let signer2 = Ed25519Pair::from_random(&mut rng);
        let signer3 = Ed25519Pair::from_random(&mut rng);

        let signer_set = SignerSet::new(
            vec![
                signer1.public_key(),
                signer2.public_key(),
                signer3.public_key(),
            ],
            2,
        );

        assert_eq!(
            signer_set,
            mc_util_serial::decode(&mc_util_serial::encode(&signer_set)).unwrap(),
        );

        let message = b"this is a test";
        let multi_sig = MultiSig::new(vec![signer1.try_sign(message.as_ref()).unwrap()]);
        assert_eq!(
            multi_sig,
            mc_util_serial::decode(&mc_util_serial::encode(&multi_sig)).unwrap(),
        );
    }
}

*/
