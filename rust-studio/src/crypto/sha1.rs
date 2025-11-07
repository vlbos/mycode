use sha1::{Digest, Sha1};

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

// sha1prng原理：
// 将SHA-1算法和PRNG算法结合，生成伪随机数，步骤：
// 计算熵值：熵值是随机性的度量，取自系统时间、内存使用情况等信息。
// 将计算出来的熵值做为SHA-1算法秘钥，再加上一个计数器作为消息，生成hash值
// 使用梅森旋转算法生成伪随机数。梅森旋转算法需要一个初始向量，将hash值作为初始向量，通过迭代来生成一序列随机数。SHA1PRNG算法每生成一个随机数，更新一次hash值
// 初始化计数器，计数器用于防止攻击者通过短时间内的暴力攻击得到相同的随机数。SHA1PRNG算法会记录生成的随机数的计数器值，每次重新初始化时，计数器值夜一并重新初始化。
// // Java 代码，key是32位 hex string

// SecureRandom secureRandom = SecureRandom.getInstance("SHA1PRNG");
// secureRandom.setSeed(pwdKey.getBytes());
// //对应js处理

// CryptoJS.SHA1(CryptoJS.SHA1(key)).toString().substring(0, 32)
// CryptoJS.enc.Hex.parse(hex-string)