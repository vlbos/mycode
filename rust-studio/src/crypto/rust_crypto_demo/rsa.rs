use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{pkcs1, pkcs8, Pkcs1v15Encrypt, Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

pub enum RsaKey {
    Pkcs1 { key: String },
    Pkcs8 { key: String },
}

/// PKCS#1 v1.5 encryption
pub fn rsa_enc(data: &[u8], pub_key_type: RsaKey) -> anyhow::Result<Vec<u8>> {
    let public_key: RsaPublicKey;
    match pub_key_type {
        RsaKey::Pkcs1 { key } => {
            public_key = pkcs1::DecodeRsaPublicKey::from_pkcs1_pem(key.as_str())?;
        }
        RsaKey::Pkcs8 { key } => {
            public_key = pkcs8::DecodePublicKey::from_public_key_pem(key.as_str())?;
        }
    }
    let mut rng = ChaCha8Rng::from_seed([42; 32]);
    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)?;
    Ok(enc_data)
}

pub fn rsa_dec(cipher_text: &[u8], private_key_str: RsaKey) -> anyhow::Result<Vec<u8>> {
    let private_key: RsaPrivateKey;
    match private_key_str {
        RsaKey::Pkcs1 { key } => {
            private_key = pkcs1::DecodeRsaPrivateKey::from_pkcs1_pem(key.as_str())?;
        }
        RsaKey::Pkcs8 { key } => {
            private_key = pkcs8::DecodePrivateKey::from_pkcs8_pem(key.as_str())?;
        }
    }
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, cipher_text)?;
    Ok(dec_data)
}

/// PKCS#1 v1.5 signatures
pub fn rsa_sign(data: &[u8], private_key_str: RsaKey) -> anyhow::Result<Vec<u8>> {
    let private_key: RsaPrivateKey;
    match private_key_str {
        RsaKey::Pkcs1 { key } => {
            private_key = pkcs1::DecodeRsaPrivateKey::from_pkcs1_pem(key.as_str())?;
        }
        RsaKey::Pkcs8 { key } => {
            private_key = pkcs8::DecodePrivateKey::from_pkcs8_pem(key.as_str())?;
        }
    };
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key);

    let mut rng = ChaCha8Rng::from_seed([42; 32]);
    let signature = signing_key.sign_with_rng(&mut rng, data);
    Ok(signature.to_bytes().to_vec())
}

pub fn rsa_verify(data: &[u8], sign: &[u8], pub_key: RsaKey) -> anyhow::Result<()> {
    let public_key: RsaPublicKey;
    match pub_key {
        RsaKey::Pkcs1 { key } => {
            public_key = pkcs1::DecodeRsaPublicKey::from_pkcs1_pem(key.as_str())?;
        }
        RsaKey::Pkcs8 { key } => {
            public_key = pkcs8::DecodePublicKey::from_public_key_pem(key.as_str())?;
        }
    }
    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let signature = Signature::try_from(sign)?;
    verifying_key.verify(data, &signature)?;
    Ok(())
}
