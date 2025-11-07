// # 生成私钥（PKCS#8 格式）
// openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048

// # 提取公钥
// openssl pkey -in private_key.pem -pubout -out public_key.pem

//JAVA CODE

// import javax.crypto.Cipher;
// import javax.crypto.spec.OAEPParameterSpec;
// import javax.crypto.spec.PSource;
// import java.nio.charset.StandardCharsets;
// import java.nio.file.Files;
// import java.nio.file.Paths;
// import java.security.KeyFactory;
// import java.security.PrivateKey;
// import java.security.PublicKey;
// import java.security.spec.MGF1ParameterSpec;
// import java.security.spec.PKCS8EncodedKeySpec;
// import java.security.spec.X509EncodedKeySpec;
// import java.util.Base64;

// public class SimpleTest {

//     public static final String KEY_RSA_TYPE = "RSA";

//     public static final String ALGORITHM = "RSA/ECB/OAEPWITHSHA-256ANDMGF1PADDING";

//     /**
//      * RSA 解密
//      *
//      * @param sourceBase64RSA     base64 编码后的密文
//      * @param privateKeyBase64Str base64 编码后的私钥
//      * @return 明文
//      * @throws Exception 抛出异常
//      */
//     public static String decrypt(String sourceBase64RSA, String privateKeyBase64Str) throws Exception {

//         Cipher oaepFromInit = Cipher.getInstance(ALGORITHM);
//         OAEPParameterSpec oaepParams = new OAEPParameterSpec("SHA-256", "MGF1",
//                 new MGF1ParameterSpec("SHA-256"), PSource.PSpecified.DEFAULT);

//         byte[] privateBytes = decryptBASE64(privateKeyBase64Str);
//         PKCS8EncodedKeySpec pkcs8EncodedKeySpec = new PKCS8EncodedKeySpec(privateBytes);
//         KeyFactory keyFactory = KeyFactory.getInstance(KEY_RSA_TYPE);
//         PrivateKey privkey = keyFactory.generatePrivate(pkcs8EncodedKeySpec);

//         oaepFromInit.init(Cipher.DECRYPT_MODE, privkey, oaepParams);

//         byte[] ct = decryptBASE64(sourceBase64RSA);
//         byte[] pt = oaepFromInit.doFinal(ct);
//         return new String(pt, StandardCharsets.UTF_8);
//     }

//     /**
//      * RSA 加密
//      *
//      * @param plainTextBytes     明文字节数组
//      * @param publicKeyBase64Str base64 编码后的公钥
//      * @return base64 编码后的密文
//      * @throws Exception 抛出异常
//      */
//     public static String encrypt(byte[] plainTextBytes, String publicKeyBase64Str) throws Exception {
//         Cipher cipher = Cipher.getInstance(ALGORITHM);
//         OAEPParameterSpec oaepParams = new OAEPParameterSpec("SHA-256", "MGF1",
//                 new MGF1ParameterSpec("SHA-256"), PSource.PSpecified.DEFAULT);

//         byte[] publicBytes = decryptBASE64(publicKeyBase64Str);
//         X509EncodedKeySpec x509EncodedKeySpec = new X509EncodedKeySpec(publicBytes);
//         KeyFactory keyFactory = KeyFactory.getInstance(KEY_RSA_TYPE);
//         PublicKey publicKey = keyFactory.generatePublic(x509EncodedKeySpec);

//         cipher.init(Cipher.ENCRYPT_MODE, publicKey, oaepParams);

//         byte[] ct = cipher.doFinal(plainTextBytes);
//         return Base64.getEncoder().encodeToString(ct);
//     }

//     /**
//      * BASE64 解码，返回字节数组
//      *
//      * @param key 待解码的字符串
//      * @return 解码后的字节数组
//      */
//     public static byte[] decryptBASE64(String key) {
//         return Base64.getDecoder().decode(key);
//     }

//     /**
//      * 从文件读取私钥
//      *
//      * @param privateKeyPath 私钥文件路径
//      * @return base64 编码后的私钥字符串
//      * @throws Exception 抛出异常
//      */
//     public static String readPrivateKeyFromFile(String privateKeyPath) throws Exception {
//         String content = new String(Files.readAllBytes(Paths.get(privateKeyPath)), StandardCharsets.UTF_8);
//         // 移除PEM格式的头部和尾部，只保留密钥内容
//         content = content.replace("-----BEGIN PRIVATE KEY-----", "")
//                        .replace("-----END PRIVATE KEY-----", "")
//                        .replaceAll("\\s+", ""); // 移除所有空白字符
//         return content;
//     }

//     /**
//      * 从文件读取公钥
//      *
//      * @param publicKeyPath 公钥文件路径
//      * @return base64 编码后的公钥字符串
//      * @throws Exception 抛出异常
//      */
//     public static String readPublicKeyFromFile(String publicKeyPath) throws Exception {
//         String content = new String(Files.readAllBytes(Paths.get(publicKeyPath)), StandardCharsets.UTF_8);
//         // 移除PEM格式的头部和尾部，只保留密钥内容
//         content = content.replace("-----BEGIN PUBLIC KEY-----", "")
//                         .replace("-----END PUBLIC KEY-----", "")
//                         .replaceAll("\\s+", ""); // 移除所有空白字符
//         return content;
//     }

//     /**
//      * 主方法，用于测试程序
//      */
//     public static void main(String[] args) {
//         System.out.println("RSA加密测试程序启动...");
        
//         try {

//             String publicKeyBase64 = readPublicKeyFromFile("public_key.pem");
//             String privateKeyBase64 = readPrivateKeyFromFile("private_key.pem");
            
//             // 使用公钥进行加密
//             String plainText = "Hello, Java!";
//             String encryptedText = encrypt(plainText.getBytes(StandardCharsets.UTF_8), publicKeyBase64);
//             System.out.println("加密后的密文: " + encryptedText);

//             // 使用私钥进行解密
//             String decryptedText = decrypt(encryptedText, privateKeyBase64);
//             System.out.println("解密后的明文: " + decryptedText);
           

//         } catch (Exception e) {
//             System.err.println("程序运行出错: " + e.getMessage());
//             e.printStackTrace();
//         }
//     }
// }


//Go Code


// package main

// import (
//     "crypto/rand"
//     "crypto/rsa"
//     "crypto/sha256"
//     "crypto/x509"
//     "encoding/base64"
//     "encoding/pem"
//     "fmt"
//     "io/ioutil"
//     "log"
// )

// func main() {
//     fmt.Println("=== Go RSA加解密简化示例 ===\n")

//     // 读取密钥
//     privateKey, publicKey, err := loadKeys()
//     if err != nil {
//         log.Fatal("读取密钥失败:", err)
//     }

//     // 测试数据
//     data := "Hello, Go!"
//     fmt.Printf("原始数据: %s\n", data)

//     // 公钥加密 - OAEP填充
//     encrypted, err := rsa.EncryptOAEP(sha256.New(), rand.Reader, publicKey, []byte(data), nil)
//     if err != nil {
//         log.Fatal("加密失败:", err)
//     }
//     fmt.Printf("加密后: %s\n", base64.StdEncoding.EncodeToString(encrypted))

//     // 私钥解密 - OAEP填充
//     decrypted, err := rsa.DecryptOAEP(sha256.New(), rand.Reader, privateKey, encrypted, nil)
//     if err != nil {
//         log.Fatal("解密失败:", err)
//     }
//     fmt.Printf("解密后: %s\n", string(decrypted))

//     fmt.Printf("填充方式: OAEPwithSHA-256andMGF1padding\n")
// }

// // 加载密钥文件
// func loadKeys() (*rsa.PrivateKey, *rsa.PublicKey, error) {
//     // 读取私钥
//     privateKeyBytes, err := ioutil.ReadFile("private_key.pem")
//     if err != nil {
//         return nil, nil, err
//     }

//     block, _ := pem.Decode(privateKeyBytes)
//     if block == nil {
//         return nil, nil, fmt.Errorf("无法解析PEM格式的私钥")
//     }

//     // 尝试解析PKCS8格式的私钥
//     privateKey, err := x509.ParsePKCS8PrivateKey(block.Bytes)
//     if err != nil {
//         // 如果PKCS8解析失败，尝试PKCS1格式作为备选
//         privateKey, err = x509.ParsePKCS1PrivateKey(block.Bytes)
//         if err != nil {
//             return nil, nil, fmt.Errorf("无法解析私钥（PKCS8和PKCS1格式都失败）: %v", err)
//         }
//     }

//     // 类型断言确保是RSA私钥
//     rsaPrivateKey, ok := privateKey.(*rsa.PrivateKey)
//     if !ok {
//         return nil, nil, fmt.Errorf("私钥不是RSA类型")
//     }

//     // 读取公钥
//     publicKeyBytes, err := ioutil.ReadFile("public_key.pem")
//     if err != nil {
//         return nil, nil, err
//     }

//     block, _ = pem.Decode(publicKeyBytes)
//     if block == nil {
//         return nil, nil, fmt.Errorf("无法解析PEM格式的公钥")
//     }

//     publicKey, err := x509.ParsePKIXPublicKey(block.Bytes)
//     if err != nil {
//         return nil, nil, err
//     }

//     rsaPublicKey := publicKey.(*rsa.PublicKey)
//     return rsaPrivateKey, rsaPublicKey, nil
// }

//python code

// #!/usr/bin/env python3
// # -*- coding: utf-8 -*-
// """
// RSA加解密示例
// 使用RSA/ECB/OAEPWithSHA-256AndMGF1Padding加密方式
// """

// from cryptography.hazmat.primitives import hashes, serialization
// from cryptography.hazmat.primitives.asymmetric import rsa, padding
// from cryptography.hazmat.backends import default_backend
// import base64
// import os

// class RSACrypto:
//     """RSA加解密类"""
    
//     def __init__(self, key_size=2048):
//         """
//         初始化RSA加密类
        
//         Args:
//             key_size (int): RSA密钥长度，默认2048位
//         """
//         self.key_size = key_size
//         self.private_key = None
//         self.public_key = None
        
//     def load_keys(self, private_key_path="private_key.pem", public_key_path="public_key.pem"):
//         """
//         从文件加载密钥
        
//         Args:
//             private_key_path (str): 私钥文件路径
//             public_key_path (str): 公钥文件路径
//         """
//         # 加载私钥
//         with open(private_key_path, "rb") as f:
//             self.private_key = serialization.load_pem_private_key(
//                 f.read(),
//                 password=None,
//                 backend=default_backend()
//             )
        
//         # 加载公钥
//         with open(public_key_path, "rb") as f:
//             self.public_key = serialization.load_pem_public_key(
//                 f.read(),
//                 backend=default_backend()
//             )
        
//         print("密钥加载完成！")
    
//     def encrypt(self, message):
//         """
//         使用公钥加密消息
        
//         Args:
//             message (str): 要加密的消息
            
//         Returns:
//             str: Base64编码的加密结果
//         """
//         if not self.public_key:
//             raise ValueError("请先加载或生成公钥")
        
//         # 将消息转换为字节
//         if isinstance(message, str):
//             message = message.encode('utf-8')
        
//         # 使用OAEP填充和SHA-256哈希进行加密
//         encrypted = self.public_key.encrypt(
//             message,
//             padding.OAEP(
//                 mgf=padding.MGF1(algorithm=hashes.SHA256()),
//                 algorithm=hashes.SHA256(),
//                 label=None
//             )
//         )
        
//         # 返回Base64编码的结果
//         return base64.b64encode(encrypted).decode('utf-8')
    
//     def decrypt(self, encrypted_message):
//         """
//         使用私钥解密消息
        
//         Args:
//             encrypted_message (str): Base64编码的加密消息
            
//         Returns:
//             str: 解密后的原始消息
//         """
//         if not self.private_key:
//             raise ValueError("请先加载或生成私钥")
        
//         # 将Base64编码的消息转换为字节
//         if isinstance(encrypted_message, str):
//             encrypted_message = base64.b64decode(encrypted_message.encode('utf-8'))
        
//         # 使用OAEP填充和SHA-256哈希进行解密
//         decrypted = self.private_key.decrypt(
//             encrypted_message,
//             padding.OAEP(
//                 mgf=padding.MGF1(algorithm=hashes.SHA256()),
//                 algorithm=hashes.SHA256(),
//                 label=None
//             )
//         )
        
//         # 返回解密后的消息
//         return decrypted.decode('utf-8')

// def main():
//     """主函数 - 演示RSA加解密"""
//     print("RSA加密测试程序启动...")
//     print("加密方式: RSA/ECB/OAEPWithSHA-256AndMGF1Padding")
//     print()
    
//     # 创建RSA加密实例
//     rsa_crypto = RSACrypto(key_size=2048)
//     rsa_crypto.load_keys()
    
//     # 测试消息
//     test_message = "Hello, Python！"
//     print(f"原始消息: {test_message}")
//     print()
    
//     # 加密
//     print("正在加密...")
//     encrypted = rsa_crypto.encrypt(test_message)
//     print(f"加密后的密文 (Base64): {encrypted}")
//     print()
    
//     # 解密
//     print("正在解密...")
//     decrypted = rsa_crypto.decrypt(encrypted)
//     print(f"解密结果: {decrypted}")
//     print()

// if __name__ == "__main__":
//     main()



//Php code

// // 安装第三方库 phpseclib
// composer require phpseclib/phpseclib


// <?php


// include_once "vendor/autoload.php";

// $rsa = new RsaUtil();

// echo "RSA加密测试程序启动...".PHP_EOL;
// $encrypted = $rsa->encryptSHA256OAEP('Hello, PHP!');
// echo "加密后的密文: $encrypted".PHP_EOL;
// $decrypted = $rsa->decryptSHA256OAEP($encrypted);
// echo "解密后的明文: $decrypted".PHP_EOL;

// class RsaUtil
// {
//     private $publicKey;
//     private $privateKey;

//     /**
//      * 构造函数
//      */
//     public function __construct()
//     {
//         try {
//             $this->publicKey = \phpseclib3\Crypt\RSA::loadPublicKey(file_get_contents('public_key.pem'));
//             $this->privateKey = \phpseclib3\Crypt\RSA::loadPrivateKey(file_get_contents('private_key.pem'));
//         } catch (Exception $e) {
//             return false;
//         }
//     }

//     /**
//      * 使用SHA-256 OAEP填充加密（与Go完全兼容）
//      * @param string $data 待加密的数据
//      * @param bool $base64 加密的数据是否是base64编码的
//      * @return string 加密后的数据
//      */
//     public function encryptSHA256OAEP($data, $base64 = true)
//     {
//         try {
//             $this->publicKey->withPadding(\phpseclib3\Crypt\RSA::ENCRYPTION_OAEP);
//             $this->publicKey->withHash('sha256');
//             $this->publicKey->withMGFHash('sha256');

//             $encrypted = $this->publicKey->encrypt($data);
//             return $base64 ? base64_encode($encrypted) : $encrypted;
//         } catch (\Throwable $th) {
//             //throw $th;
//             return false;
//         }
//     }

//     /**
//      * 使用SHA-256 OAEP填充解密（与Go完全兼容）
//      * @param string $encrypted_data 加密后的数据
//      * @param bool $base64 加密的数据是否是base64编码的
//      * @return string 解密后的数据
//      */
//     public function decryptSHA256OAEP($encrypted_data, $base64 = true)
//     {
//         try {
//             $this->privateKey->withPadding(\phpseclib3\Crypt\RSA::ENCRYPTION_OAEP);
//             $this->privateKey->withHash('sha256');
//             $this->privateKey->withMGFHash('sha256');
//             if ($base64) {
//                 $encrypted_data = base64_decode($encrypted_data);
//             }
//             return $this->privateKey->decrypt($encrypted_data);
//         } catch (\Throwable $th) {
//             //throw $th;
//             return false;
//         }
//     }
// }




// let public_key = RsaPublicKey::from_public_key_pem(&public_key)?;
// let padding = Oaep::new_with_mgf_hash::<Sha256, Sha1>();

// let encrypted_data = public_key.encrypt(&mut rng, padding, data.as_bytes())?;


// [dependencies]
// rsa-oaep-pss = "1"
// Check out the crates.io page to see what is the latest version of this crate.

// How to use ?
// Keys generation
// let (public_key, private_key) = rsa_oaep_pss::generate_rsa_keys(&mut rng, 2048)
//     .expect("keys generation error");
// Encryption using OAEP scheme
// let message = b"some secret";

// let mut oaep = rsa_oaep_pss::RsaOaep::new(rand::rngs::OsRng, &sha2::Sha256::new());

// let ciphertext = oaep
//     .encrypt(&public_key, message)
//     .expect("encryption error");

// let recovered = oaep
//     .decrypt(&private_key, &ciphertext)
//     .expect("decryption error");

// assert_eq!(recovered, message);
// Signature using PSS scheme
// let message = b"message to sign";

// let mut pss = rsa_oaep_pss::RsaPss::new(rand::rngs::OsRng, &sha2::Sha256::new());

// let signature = pss.sign(&private_key, message).expect("signature error");

// let verification = pss.verify(&public_key, message, &signature);

// assert!(verification.is_ok());
// Importing and exporting of keys
// use rsa_oaep_pss::{FromPem, ToPem};

// let pem_public_key = std::fs::read_to_string("public.pem")?;

// let public_key = RsaPublicKey::from_pem(&pem_public_key)?;

// let re_exported_pem_public_key = public_key.to_pem()?;

// assert_eq!(pem_public_key, re_exported_pem_public_key);
// You can also use FromDer and ToDer for dealing with raw DER data.

// Run the examples
// You can run examples contained in the examples folder by using the following command:

// cargo run --example <filename> --release 




// use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
// use rand::rngs::OsRng;

// let mut rng = OsRng;
// let bits = 2048;
// let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
// let pub_key = RsaPublicKey::from(&priv_key);

// // Encrypt
// let data = b"hello world";
// let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
// assert_ne!(&data[..], &enc_data[..]);

// // Decrypt
// let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data).expect("failed to decrypt");
// assert_eq!(&data[..], &dec_data[..]);

