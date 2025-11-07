use aes::cipher::block_padding::Pkcs7;
use anyhow::anyhow;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const AES_KEY: [u8; 32] = *b"1d64dce239c4437d1d64dce239c4437d";
const AES_IV: [u8; 16] = *b"45154c1fc4541161";

/// aes cbc pkcs7 enc
pub fn enc(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let res =
        Aes256CbcEnc::new(&AES_KEY.into(), &AES_IV.into()).encrypt_padded_vec_mut::<Pkcs7>(input);
    Ok(res)
}

/// aes cbc pkcs7 dec
pub fn dec(cipher: &[u8]) -> anyhow::Result<Vec<u8>> {
    let res = Aes256CbcDec::new(&AES_KEY.into(), &AES_IV.into())
        .decrypt_padded_vec_mut::<Pkcs7>(&cipher)
        .map_err(|e| anyhow!(format!("dec err {:?}", e.to_string())))?;
    Ok(res)
}
