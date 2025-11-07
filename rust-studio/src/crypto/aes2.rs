use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;

const AES_BLOCK_SIZE: usize = 16;

pub fn aes_cbc_decrypt(ciphertext: &[u8], key: [u8; 24], iv: [u8; 16]) -> Vec<u8> {
    let mut buf = ciphertext.to_owned();

    let pt = Aes192CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<NoPadding>(&mut buf)
        .unwrap();
    let pad_trim = pkcs5_trim(pt);
    pad_trim.to_vec()
}

pub fn aes_cbc_encrypt(plaintext: &[u8], key: [u8; 24], iv: [u8; 16]) -> Vec<u8> {
    let mut buf = pkcs5_pad(plaintext, AES_BLOCK_SIZE);
    let pt_len = buf.len();

    let ct = Aes192CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<NoPadding>(&mut buf, pt_len)
        .unwrap();

    ct.to_vec()
}

fn pkcs5_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let pad_len = block_size - data.len() % block_size;
    let mut padded_data = data.to_vec();
    padded_data.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded_data
}

fn pkcs5_trim(data: &[u8]) -> &[u8] {
    let pad_len = data[data.len() - 1] as usize;
    &data[..data.len() - pad_len]
}


