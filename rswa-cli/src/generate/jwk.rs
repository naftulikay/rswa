use crate::CLI;

use anyhow::{anyhow, Error};
use clap::{ArgEnum, Parser};
use josekit::jwk::alg::ed::EdCurve;
use josekit::jwk::{Jwk, KeyPair};
use josekit::jws::alg::rsassa::RsassaJwsAlgorithm;
use josekit::jws::{EdDSA, RS256, RS384, RS512};

#[derive(Debug, Parser)]
pub struct GenerateJwkCommand {
    /// A key id to assign to the generated key. Key ids can be any string value.
    #[clap(short = 'i', long = "key-id")]
    pub key_id: Option<String>,
    /// The key algorithm to use
    #[clap(short, long, arg_enum, default_value_t)]
    pub algorithm: JwkAlgorithm,
    /// The Edwards elliptic curve to use for key generation when generating EdDSA keys
    #[clap(short, long, arg_enum, default_value_t)]
    pub curve: EdCurveType,
    /// The key size in bits when generating an RSA key
    #[clap(long, default_value = "2048")]
    pub rsa_key_size: u16,
    /// The output hash digest size for RSA keys
    #[clap(long, arg_enum, default_value_t)]
    pub rsa_digest_size: RsaDigestSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, ArgEnum)]
pub enum EdCurveType {
    #[clap(name = "ed25519")]
    Ed25519,
    #[clap(name = "ed448")]
    Ed448,
}

impl Default for EdCurveType {
    fn default() -> Self {
        Self::Ed25519
    }
}

impl From<EdCurveType> for EdCurve {
    fn from(value: EdCurveType) -> Self {
        match value {
            EdCurveType::Ed25519 => Self::Ed25519,
            EdCurveType::Ed448 => Self::Ed448,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, ArgEnum)]
pub enum JwkAlgorithm {
    RSA,
    #[clap(name = "eddsa")]
    EdDSA,
}

impl Default for JwkAlgorithm {
    fn default() -> Self {
        Self::EdDSA
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, ArgEnum)]
pub enum RsaDigestSize {
    #[clap(name = "256")]
    RS256,
    #[clap(name = "384")]
    RS384,
    #[clap(name = "512")]
    RS512,
}

impl Default for RsaDigestSize {
    fn default() -> Self {
        Self::RS256
    }
}

impl From<RsaDigestSize> for RsassaJwsAlgorithm {
    fn from(value: RsaDigestSize) -> Self {
        match value {
            RsaDigestSize::RS256 => RS256,
            RsaDigestSize::RS384 => RS384,
            RsaDigestSize::RS512 => RS512,
        }
    }
}

impl GenerateJwkCommand {
    pub async fn execute(&self, _root: &CLI) -> Result<(), Error> {
        let mut keypair = match self.algorithm {
            JwkAlgorithm::EdDSA => self.gen_eddsa_key(),
            JwkAlgorithm::RSA => self.gen_rsa_key(),
        }?;

        if let Some(key_id) = self.key_id.as_ref() {
            keypair.set_key_id(key_id);
        }

        println!(
            "{}",
            serde_json::to_string_pretty(keypair.as_ref())
                .map_err(|e| anyhow!("unable to serialize jwk to json: {e}"))?
        );

        Ok(())
    }

    fn gen_eddsa_key(&self) -> Result<Jwk, Error> {
        EdDSA
            .generate_key_pair(self.curve.into())
            .map_err(|e| {
                anyhow!(
                    "unable to generate eddsa keypair with curve {:?}: {e}",
                    self.curve
                )
            })
            .map(|k| k.to_jwk_key_pair())
    }

    fn gen_rsa_key(&self) -> Result<Jwk, Error> {
        Into::<RsassaJwsAlgorithm>::into(self.rsa_digest_size)
            .generate_key_pair(self.rsa_key_size.into())
            .map_err(|e| anyhow!("unable to generate rsa keypair: {e}"))
            .map(|k| k.to_jwk_key_pair())
    }
}
