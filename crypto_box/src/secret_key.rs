use crate::{PublicKey, KEY_SIZE};
use core::fmt::{self, Debug};
use curve25519_dalek::{MontgomeryPoint, Scalar};
use zeroize::Zeroize;

#[cfg(feature = "rand_core")]
use aead::rand_core::CryptoRngCore;

#[cfg(feature = "serde")]
use serdect::serde::{de, ser, Deserialize, Serialize};

/// A `crypto_box` secret key.
#[derive(Clone)]
pub struct SecretKey(pub(crate) Scalar);

impl SecretKey {
    /// Generate a random [`SecretKey`].
    #[cfg(feature = "rand_core")]
    pub fn generate(csprng: &mut impl CryptoRngCore) -> Self {
        let mut bytes = [0u8; KEY_SIZE];
        csprng.fill_bytes(&mut bytes);
        bytes.into()
    }

    /// Get the [`PublicKey`] which corresponds to this [`SecretKey`]
    pub fn public_key(&self) -> PublicKey {
        PublicKey(MontgomeryPoint::mul_base(&self.0))
    }

    /// Serialize [`SecretKey`] to bytes.
    ///
    /// # ⚠️Warning
    ///
    /// The serialized bytes are secret key material. Please treat them with
    /// the care they deserve!
    pub fn to_bytes(&self) -> [u8; KEY_SIZE] {
        self.0.to_bytes()
    }
}

impl From<Scalar> for SecretKey {
    fn from(value: Scalar) -> Self {
        SecretKey(value)
    }
}

impl From<[u8; KEY_SIZE]> for SecretKey {
    fn from(bytes: [u8; KEY_SIZE]) -> SecretKey {
        SecretKey(Scalar::from_bits_clamped(bytes))
    }
}

impl Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretKey").finish_non_exhaustive()
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(feature = "serde")]
impl Serialize for SecretKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serdect::array::serialize_hex_upper_or_bin(self.0.as_bytes(), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut bytes = [0u8; KEY_SIZE];
        serdect::array::deserialize_hex_or_bin(&mut bytes, deserializer)?;
        Ok(SecretKey::from(bytes))
    }
}