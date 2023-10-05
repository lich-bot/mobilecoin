// Copyright (c) 2018-2022 The MobileCoin Foundation

//!  data structures not defined elsewhere.

use crate::mealy::{Input as MealyInput, Output as MealyOutput};
use alloc::vec::Vec;
use core::marker::PhantomData;
use mc_attest_core::VerificationReport;
use mc_attest_verifier_types::EvidenceKind;
use mc_attestation_verifier::TrustedIdentity;
use mc_crypto_keys::Kex;
use mc_crypto_noise::{
    HandshakeIX, HandshakeNX, HandshakePattern, NoiseCipher, NoiseDigest, ProtocolName,
};

/// An input used to inject the relevant local data needed to transform Start
/// into an AuthPending for node-to-node authentication.
pub struct NodeInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    /// This is the local node's identity key
    pub(crate) local_identity: KexAlgo::Private,
    /// This is the local node's ias report.
    pub(crate) ias_report: VerificationReport,

    _kex: PhantomData<KexAlgo>,
    _cipher: PhantomData<Cipher>,
    _digest: PhantomData<DigestAlgo>,
}

impl<KexAlgo, Cipher, DigestAlgo> NodeInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    /// Create a new input event to initiate a node-to-node channel.
    pub fn new(local_identity: KexAlgo::Private, ias_report: VerificationReport) -> Self {
        Self {
            local_identity,
            ias_report,
            _kex: PhantomData,
            _cipher: PhantomData,
            _digest: PhantomData,
        }
    }
}

impl<KexAlgo, Cipher, DigestAlgo> MealyInput for NodeInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
}

/// An input used to transform a Start into an AuthPending for client-to-node
/// authentication.
pub struct ClientInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    _kex: PhantomData<KexAlgo>,
    _cipher: PhantomData<Cipher>,
    _digest: PhantomData<DigestAlgo>,
}

impl<KexAlgo, Cipher, DigestAlgo> Default for ClientInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    fn default() -> Self {
        Self {
            _kex: PhantomData,
            _cipher: PhantomData,
            _digest: PhantomData,
        }
    }
}

impl<KexAlgo, Cipher, DigestAlgo> MealyInput for ClientInitiate<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
}

/// An opaque blob of noise protocol handshake bytes, generated by an initiator,
/// and consumed by a responder.
pub struct AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>
where
    Handshake: HandshakePattern,
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    /// The actual AuthRequest data
    pub(crate) data: Vec<u8>,

    /// Type consumption
    _protocol_name: ProtocolName<Handshake, KexAlgo, Cipher, DigestAlgo>,
}

impl<Handshake, KexAlgo, Cipher, DigestAlgo> From<Vec<u8>>
    for AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>
where
    Handshake: HandshakePattern,
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    fn from(data: Vec<u8>) -> Self {
        Self {
            data,
            _protocol_name: ProtocolName::<Handshake, KexAlgo, Cipher, DigestAlgo>::default(),
        }
    }
}

impl<Handshake, KexAlgo, Cipher, DigestAlgo>
    From<AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>> for Vec<u8>
where
    Handshake: HandshakePattern,
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    fn from(src: AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>) -> Vec<u8> {
        src.data
    }
}

impl<Handshake, KexAlgo, Cipher, DigestAlgo> AsRef<[u8]>
    for AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>
where
    Handshake: HandshakePattern,
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

/// An authentication request is output from an initiator
impl<Handshake, KexAlgo, Cipher, DigestAlgo> MealyOutput
    for AuthRequestOutput<Handshake, KexAlgo, Cipher, DigestAlgo>
where
    Handshake: HandshakePattern,
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
}

/// An input used to transform a Start into a Ready for a client-to-node
/// responder.
///
/// It contains the AuthRequestOutput generated by an initiator, and the
/// supporting data a responder will need to complete the NX handshake.
pub struct ClientAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    /// This is the local node's identity key
    pub(crate) local_identity: KexAlgo::Private,
    /// This is the local node's ias report.
    pub(crate) ias_report: VerificationReport,

    /// The auth request input, including payload, if any
    pub(crate) data: AuthRequestOutput<HandshakeNX, KexAlgo, Cipher, DigestAlgo>,
}

impl<KexAlgo, Cipher, DigestAlgo> MealyInput for ClientAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
}

impl<KexAlgo, Cipher, DigestAlgo> ClientAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    pub fn new(
        data: AuthRequestOutput<HandshakeNX, KexAlgo, Cipher, DigestAlgo>,
        local_identity: KexAlgo::Private,
        ias_report: VerificationReport,
    ) -> Self {
        Self {
            local_identity,
            ias_report,
            data,
        }
    }
}

/// An input used to transform a Start into a Ready for a node-to-node
/// responder.
///
/// It contains the AuthRequestOutput generated by an initiator, and the
/// supporting data a responder will need to complete the handshake.
pub struct NodeAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    /// This is the local node's identity key
    pub(crate) local_identity: KexAlgo::Private,
    /// This is the local node's ias report.
    pub(crate) ias_report: VerificationReport,
    /// The identities that the initiator's IAS report must conform to
    pub(crate) identities: Vec<TrustedIdentity>,

    /// The auth request input, including payload, if any
    pub(crate) data: AuthRequestOutput<HandshakeIX, KexAlgo, Cipher, DigestAlgo>,
}

impl<KexAlgo, Cipher, DigestAlgo> MealyInput for NodeAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
}

impl<KexAlgo, Cipher, DigestAlgo> NodeAuthRequestInput<KexAlgo, Cipher, DigestAlgo>
where
    KexAlgo: Kex,
    Cipher: NoiseCipher,
    DigestAlgo: NoiseDigest,
{
    pub fn new(
        data: AuthRequestOutput<HandshakeIX, KexAlgo, Cipher, DigestAlgo>,
        local_identity: KexAlgo::Private,
        ias_report: VerificationReport,
        identities: impl Into<Vec<TrustedIdentity>>,
    ) -> Self {
        Self {
            local_identity,
            ias_report,
            identities: identities.into(),
            data,
        }
    }
}

/// An opaque blob containing output by a responder to complete a noise
/// handshake.
pub struct AuthResponseOutput(Vec<u8>);

impl From<Vec<u8>> for AuthResponseOutput {
    fn from(src: Vec<u8>) -> Self {
        Self(src)
    }
}

impl AsRef<[u8]> for AuthResponseOutput {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<AuthResponseOutput> for Vec<u8> {
    fn from(src: AuthResponseOutput) -> Vec<u8> {
        src.0
    }
}

/// An authentication response is output from a responder
impl MealyOutput for AuthResponseOutput {}

/// The authentication response is combined with identities for the initiator.
pub struct AuthResponseInput {
    pub(crate) data: Vec<u8>,
    pub(crate) identities: Vec<TrustedIdentity>,
}

impl AuthResponseInput {
    pub fn new(data: AuthResponseOutput, identity: impl Into<Vec<TrustedIdentity>>) -> Self {
        Self {
            data: data.0,
            identities: identity.into(),
        }
    }
}

impl AsRef<[u8]> for AuthResponseInput {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl From<AuthResponseInput> for Vec<u8> {
    fn from(src: AuthResponseInput) -> Vec<u8> {
        src.data
    }
}

/// An authentication response input to a responder
impl MealyInput for AuthResponseInput {}

/// An unverified report is used when the initiator may not know the identity of
/// the enclave.
pub struct UnverifiedReport {
    pub(crate) data: Vec<u8>,
}

impl UnverifiedReport {
    pub fn new(data: AuthResponseOutput) -> Self {
        Self { data: data.0 }
    }
}

impl AsRef<[u8]> for UnverifiedReport {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

/// An authentication response from a responder
impl MealyInput for UnverifiedReport {}

/// The IAS report is the final output when authentication succeeds.
impl MealyOutput for VerificationReport {}
impl MealyOutput for EvidenceKind {}

/// A type similar to aead::Payload used to distinguish writer inputs from
/// outputs.
pub struct Plaintext<'aad, 'msg> {
    pub aad: &'aad [u8],
    pub msg: &'msg [u8],
}

impl<'aad, 'msg> Plaintext<'aad, 'msg> {
    pub fn new(aad: &'aad [u8], msg: &'msg [u8]) -> Self {
        Self { aad, msg }
    }
}

/// Plaintext may be provided to an FST for encryption into a vector
impl MealyInput for Plaintext<'_, '_> {}

/// A type similar to aead::Payload used to distinguish reader inputs from
/// outputs.
pub struct Ciphertext<'aad, 'msg> {
    pub aad: &'aad [u8],
    pub msg: &'msg [u8],
}

impl<'aad, 'msg> Ciphertext<'aad, 'msg> {
    pub fn new(aad: &'aad [u8], msg: &'msg [u8]) -> Self {
        Self { aad, msg }
    }
}

/// A ciphertext may be provided to a FST for decryption into a vector
impl MealyInput for Ciphertext<'_, '_> {}

/// Our outputs may be simple vectors for the proto-inside-grpc use case.
impl MealyOutput for Vec<u8> {}

/// A type similar to [`aead::Payload`] used to distinguish writer inputs from
/// outputs when there's an explicit nonce.
pub struct NoncePlaintext<'aad, 'msg>(Plaintext<'aad, 'msg>);

impl<'aad, 'msg> NoncePlaintext<'aad, 'msg> {
    /// Create a new NoncePlaintext object from the given slices.
    pub fn new(aad: &'aad [u8], msg: &'msg [u8]) -> Self {
        Self(Plaintext::new(aad, msg))
    }

    /// Grab a reference to the internal `aad` slice.
    pub fn aad(&self) -> &[u8] {
        self.0.aad
    }

    /// Grab a reference to the internal `msg` slice.
    pub fn msg(&self) -> &[u8] {
        self.0.msg
    }
}

/// Plaintext may be provided to an FST for encryption into a vector
impl MealyInput for NoncePlaintext<'_, '_> {}

/// A tuple of bytes and a u64 can be output from an FST for the
/// encrypt-for-explicit nonce case.
impl MealyOutput for (Vec<u8>, u64) {}

/// A type similar to [`aead::Payload`] used to distinguish reader inputs from
/// outputs when there's an explicit nonce.
pub struct NonceCiphertext<'aad, 'msg> {
    pub ciphertext: Ciphertext<'aad, 'msg>,
    pub nonce: u64,
}

impl<'aad, 'msg> NonceCiphertext<'aad, 'msg> {
    pub fn new(aad: &'aad [u8], msg: &'msg [u8], nonce: u64) -> Self {
        Self {
            ciphertext: Ciphertext::new(aad, msg),
            nonce,
        }
    }
}

/// Plaintext may be provided to an FST for decryption into a vector
impl MealyInput for NonceCiphertext<'_, '_> {}
