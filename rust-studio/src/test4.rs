use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use hex::encode as to_hex;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;

fn aes_ecb_encrypt(key: &str, data: &str) -> String {
    let mut secret = [0u8; 16];
    secret[..key.len()].copy_from_slice(&key.as_bytes());

    let mut buf = [0u8; 32];
    let pt_len = data.len();
    buf[..pt_len].copy_from_slice(data.as_bytes());

    let encrypt = Aes128EcbEnc::new(&secret.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .expect("Oh no!");

    let hex_str = to_hex(encrypt);

    hex_str
}

fn main() {
    let key = "6661428";
    let data = "1726104884_1428_c816bd50266a";
    println!("{}", aes_ecb_encrypt(key, data));
}
import javax.crypto.*;
import javax.crypto.spec.SecretKeySpec;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;

public class Main {
    public static byte[] encryptMessage(byte[] message, byte[] keyBytes)
         throws InvalidKeyException, NoSuchPaddingException, NoSuchAlgorithmException, BadPaddingException, IllegalBlockSizeException
    {
        Cipher cipher = Cipher.getInstance("AES/ECB/PKCS5Padding");
        SecretKey secretKey = new SecretKeySpec(keyBytes, "AES");
        cipher.init(Cipher.ENCRYPT_MODE, secretKey);
        return cipher.doFinal(message);
    }

    public static void main(String[] args) {
        try {
            byte[] bytes = encryptMessage("1726104884_1428_c816bd50266a".getBytes(), "6661428\0\0\0\0\0\0\0\0\0".getBytes());
            System.out.println(HexFormat.of().formatHex(bytes));
        } catch (Exception e) {
            System.out.println(e);
        }
    }
}