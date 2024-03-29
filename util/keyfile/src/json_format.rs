//! JSON formats for private keys together with fog data.
//! Files formatted in this way are sufficient to derive an account key in
//! a self-contained way without any context, which is useful for many tools.

use mc_account_keys::{RootEntropy, RootIdentity};
use serde::{Deserialize, Serialize};

/// Historical JSON schema for a root identity
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug, Serialize, Deserialize)]
pub struct RootIdentityJson {
    /// Root entropy used to derive a user's private keys.
    pub root_entropy: [u8; 32],
    /// User's fog url, if any.
    pub fog_url: String,
    /// User's report id, if any.
    pub fog_report_id: String,
    /// User's fog authority subjectPublicKeyInfo bytes, if any
    pub fog_authority_spki: Vec<u8>,
}

impl From<&RootIdentity> for RootIdentityJson {
    fn from(src: &RootIdentity) -> Self {
        Self {
            root_entropy: src.root_entropy.bytes,
            fog_url: src.fog_report_url.clone(),
            fog_report_id: src.fog_report_id.clone(),
            fog_authority_spki: src.fog_authority_spki.clone(),
        }
    }
}

impl From<RootIdentityJson> for RootIdentity {
    fn from(src: RootIdentityJson) -> Self {
        Self {
            root_entropy: RootEntropy::from(&src.root_entropy),
            fog_report_url: src.fog_url,
            fog_report_id: src.fog_report_id,
            fog_authority_spki: src.fog_authority_spki,
        }
    }
}
