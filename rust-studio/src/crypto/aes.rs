use aes::cipher::{block_padding::Pkcs7, generic_array::GenericArray, BlockDecryptMut, BlockEncryptMut, KeyInit};
use log::error;
use sha1::{Digest, Sha1};

type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

pub(crate) fn md5_hex(msg: &str) -> String {
    let digest = md5::compute(msg);
    // :x 转为 十六进制字符串
    format!("{:x}", digest)
}


/// key 的len都是16, 执行后，data已变为加密内容
fn aes_ecb_encrypt(key: Vec<u8>, data: String) -> String {

    let key = sha1prng(key);
    let secret = GenericArray::from_slice(&key); 

    let data = data.as_bytes();

    // 目前data长度不会超过32
    let mut buf = [0u8; 32];
    let pt_len = data.len();
    buf[..pt_len].copy_from_slice(&data);

    let encrypt = Aes128EcbEnc::new(secret)
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .map_err(|e| error!("encrypt err : {}", e))
        .unwrap();

    // println!("{:?}", encrypt);

    let hex_str = hex::encode_upper(encrypt);

    // println!("hex_str: {hex_str}");

    hex_str
}

/// key 的len都是16，执行后，data已变为解密内容
fn aes_ecb_decrypt(key: Vec<u8>, data: String) -> String {
    if data.len() < 1 {
        return "".to_string();
    }

    let key = sha1prng(key);
    let secret = GenericArray::from_slice(&key); 

    let mut en_data = match hex::decode(&data) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("data is err : {}, because : {}", data, e);
            return "".to_string();
        }
    };

    // println!("{:?}", en_data);

    let decrypt_padded_mut = Aes128EcbDec::new(secret)
        .decrypt_padded_mut::<Pkcs7>(&mut en_data);

    let decrypt = decrypt_padded_mut
        .map_err(|e| println!("decrypt err : {}", e))
        .unwrap();

    String::from_utf8_lossy(decrypt).to_string()
}


/// sha1prng原理：
/// 1. 将SHA-1算法和PRNG算法结合，生成伪随机数，步骤：
/// * 计算熵值：熵值是随机性的度量，取自系统时间、内存使用情况等信息。
/// * 将计算出来的熵值做为SHA-1算法秘钥，再加上一个计数器作为消息，生成hash值
/// * 使用梅森旋转算法生成伪随机数。梅森旋转算法需要一个初始向量，将hash值作为初始向量，通过迭代来生成一序列随机数。SHA1PRNG算法每生成一个随机数，更新一次hash值
/// * 初始化计数器，计数器用于防止攻击者通过短时间内的暴力攻击得到相同的随机数。SHA1PRNG算法会记录生成的随机数的计数器值，每次重新初始化时，计数器值夜一并重新初始化。
/// 
/// 此函数相当于Java的
/// ```java
/// 
/// SecureRandom random=SecureRandom.getInstance("SHA1PRNG");
/// random.setSeed(key.getBytes());
/// 
/// KeyGenerator kgen = KeyGenerator.getInstance("AES");
/// kgen.init(128, random);
/// SecretKey secretKey = kgen.generateKey();
/// byte[] enCodeFormat = secretKey.getEncoded();
/// 
/// System.out.println("enCodeFormat : " + Arrays.toString(enCodeFormat)+" hex: " + HexFormat.of().formatHex(enCodeFormat));
/// ```
fn sha1prng(key: Vec<u8>) -> Vec<u8>  {
    
    // 第一次 SHA-1 哈希
    let mut hasher = Sha1::default();
    hasher.update(key);
    let first_hash = hasher.finalize();

    // 第二次 SHA-1 哈希
    let mut hasher = Sha1::default();
    hasher.update(first_hash);
    let second_hash = hasher.finalize();

    // 转换为十六进制字符串，并取前 32 字节
    // hex::encode_upper(&second_hash[..]).chars().take(32).collect()

    // 未转换，则取前16字节
    second_hash[0..16].into()
}

mod test {
    use hex::encode_upper;
    use hex_literal::hex;


    #[test]
    fn test_rand() {
        
        let key = "6661428";
        
        let hash = crate::utils::crypto::sha1prng(key.as_bytes().to_vec());
        println!("hash : {:?} ", hash.clone());
        println!("hash hex : {:?} ", encode_upper(hash.clone()));

        let expect = hex!("E343A4A2F36F5315015A067F493071A8");

        assert_eq!(hash, expect);

    }

    #[test]
    fn test_decrypt() {
        let key = "6661428".as_bytes().to_vec();
        let code = "02C282402EF9671561D56F9693AD6FBDAFA0AC62497EC19E1C6470E177142D48".to_string();
        let text: String = crate::utils::crypto::aes_ecb_decrypt(key, code);
        println!("text: {}", text);
        println!("expect: 1726104884_1428_c816bd50266a");
        assert_eq!(text, "1726104884_1428_c816bd50266a");
    }

    #[test]
    fn test_aes() {
        let key = "6661428".as_bytes().to_vec();
        let data = "1726104884_1428_c816bd50266a".to_string();

        println!("raw: {:?}", data);

        let code: String = crate::utils::crypto::aes_ecb_encrypt(key.clone(), data.clone());
        let expect = "02C282402EF9671561D56F9693AD6FBDAFA0AC62497EC19E1C6470E177142D48";

        println!("output: {}", code);
        println!("40的结果, expect: {}", expect);

        assert_eq!(code, expect);
        
        let text: String = crate::utils::crypto::aes_ecb_decrypt(key, code);
        println!("text: {}", text);
        println!("expect: {data}");
        
        assert_eq!(text, data);

    }


    #[test]
    fn test_md5_hex() {
        let mac = "c8:16:bd:50:26:6a";
        let uuid = "20160812200513000mxZJgJEe11425";
        let msg = format!("{}{}", mac, uuid);

        let md5_api_key = crate::utils::crypto::md5_hex(&msg);

        assert_eq!(md5_api_key, "b733342c47986b9e3712e9792e9a3b63");
    }

    #[test]
    fn test_base64() {
        let code = "YTY2MjA1ZGI2MThjLzIwMTYwODEyMjAwNTEzMDAwbXhaSmdKRWUxMTQyNS8yLjEuMC42ODAxOQ";

        let text = match base64::Engine::decode(&base64::prelude::BASE64_STANDARD_NO_PAD, code) {
            Ok(bytes) => String::from_utf8(bytes).unwrap_or_default(),
            Err(e) => {
                println!("decode_key error: {:?} for {}", e, code);
                "empty".to_string()
            }
        };

        println!("{text}");
        assert_eq!(
            "a66205db618c/20160812200513000mxZJgJEe11425/2.1.0.68019",
            text
        );

        let code2 = base64::Engine::encode(&base64::prelude::BASE64_STANDARD_NO_PAD, text);
        println!("{code2}");

        assert_eq!(code, code2);
    }
}
