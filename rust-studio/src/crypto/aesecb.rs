use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine as _;

use aes::cipher::{block_padding::{AnsiX923, Iso10126, Iso7816, NoPadding, Pkcs7, ZeroPadding}, BlockDecryptMut, BlockEncryptMut, KeyInit};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

/// 十六进制转成u8 array，Java解密也是先讲hex转换为bytes
fn hex_to_u8_array(hex_str: &str, len: usize) -> Result<Vec<u8>, String> {
    if hex_str.len() != len * 2 {
        return Err("Invalid length".to_string());
    }

    // println!("hex_str: {}", hex_str);
    let mut result = Vec::new();
    for (i, chunk) in hex_str.chars().collect::<Vec<char>>().chunks(2).enumerate() {
        // println!("i={i} chunk: {:?}", chunk);
        let byte = u8::from_str_radix(&chunk.iter().collect::<String>(), 16)
            .map_err(|_| "Invalid hexadecimal value".to_string())?;
        result.push(byte);
    }

    Ok(result)
}

/// u8 array转成十六进制,Java加密后将bytes array转换为了hex
fn u8_array_to_hex(arr: &[u8]) -> String {
    arr.iter()
        .map(|&byte| format!("{:02X}", byte))
        .collect::<String>()
}

/// key 的len都是16, 执行后，data已变为加密内容
fn aes_ecb_encrypt(key: &str, data: String) -> String {
    let mut secret = [0u8; 16];
    secret[..key.len()].copy_from_slice(&key.as_bytes());

    let data = data.as_bytes();

    // 目前data长度不会超过32
    let mut buf = [0u8; 32];
    let pt_len = data.len();
    buf[..pt_len].copy_from_slice(&data);

    let encrypt = Aes128EcbEnc::new(&secret.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        // .encrypt_padded_mut::<NoPadding>(&mut buf, pt_len)
        .map_err(|e| println!("encrypt err : {}", e))
        .unwrap();

    println!("{:?}", encrypt);

    let hex_str = u8_array_to_hex(encrypt);

    println!("hex_str: {hex_str}");

    hex_str
}

/// key 的len都是16，执行后，data已变为解密内容
fn aes_ecb_decrypt(key: &str, data: String) -> String {
    if data.len() < 1 {
        return "".to_string();
    }

    let mut secret = [0u8; 16];
    secret[..key.len()].copy_from_slice(key.as_bytes());

    let mut en_data = match hex_to_u8_array(&data, data.len() / 2) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("data is err : {}, because : {}", data, e);
            return "".to_string();
        }
    };

    println!("{:?}", en_data);

    // let size = (en_data.len() + 15) / 16;

    let decrypt_padded_mut = Aes128EcbDec::new(&secret.into())
        .decrypt_padded_mut::<Pkcs7>(&mut en_data);
        // .decrypt_padded_mut::<NoPadding>(&mut en_data);

    let decrypt = decrypt_padded_mut
        .map_err(|e| println!("decrypt err : {}", e))
        .unwrap();

    String::from_utf8_lossy(decrypt).to_string()
}

mod test {
    use super::{hex_to_u8_array, u8_array_to_hex};

    #[test]
    fn test_decrypt() {
        // 临时起来个加密key
        let key = "6661428";
        // java用上key加密后的bytes array转换为十六进制字符串结果
        let code = "02C282402EF9671561D56F9693AD6FBDAFA0AC62497EC19E1C6470E177142D48".to_string();
        let text: String = crate::utils::crypto::aes_ecb_decrypt(key, code);
        println!("text: {}", text);
        // 明文
        // println!("expect: 1726104884_1428_c816bd50266a");
        assert_eq!(text, "1726104884_1428_c816bd50266a");
    }

    #[test]
    fn test_aes() {
        let key = "6661428";
        let data = "1726104884_1428_c816bd50266a".to_string();

        println!("raw: {:?}", data);

        let code = crate::utils::crypto::aes_ecb_encrypt(key, data);

        println!("output: {}", code);
        println!("expect: 02C282402EF9671561D56F9693AD6FBDAFA0AC62497EC19E1C6470E177142D48");

        let text = crate::utils::crypto::aes_ecb_decrypt(key, code);
        println!("text: {}", text);
        println!("expect: 1726104884_1428_c816bd50266a");
    }


}
