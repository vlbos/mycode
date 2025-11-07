use crate::rsa::RsaKey;

mod aes;
mod rsa;

fn main() -> anyhow::Result<()> {
    let plain_text = "hello rust aes";
    let enc = aes::enc(plain_text.as_bytes())?;
    let dec = aes::dec(enc.as_slice())?;
    println!("dec res {:?}", String::from_utf8(dec));

    let rsa_pub_key = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3BwFYz7BpeQEfJfe7JA5
KqFaCjJzURkR8mOC+e7TsMGrXc/N/KhHOliYZCiN24+Fa/WnkWO4Bqvm9hOByaOB
wY1Eps1L46o24uaPti4sRs57WJTDfQObeq5OgTaiq79HZaDSo4K5sMdCllSjBcZy
67Y5os8jmfk0K7tgecSiZ1WHk5Arb2uf3j+SgUPzalcTLY9QvI+XqwiAHKjuRKQo
SN2PSTogNUeqKK3wbD4VvBC8gw0JqH6Y/p5Exsi8HPAkKL+DHhfBi2zFR1WeDXk9
pw4y59+M3RjwVYrvq8BFbgsQ8xpF2c9UPEpu2t1sRx/dZLn2CFef1aVLcFjCcjiq
DwIDAQAB
-----END PUBLIC KEY-----";

    let rsa_private_key = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDcHAVjPsGl5AR8
l97skDkqoVoKMnNRGRHyY4L57tOwwatdz838qEc6WJhkKI3bj4Vr9aeRY7gGq+b2
E4HJo4HBjUSmzUvjqjbi5o+2LixGzntYlMN9A5t6rk6BNqKrv0dloNKjgrmwx0KW
VKMFxnLrtjmizyOZ+TQru2B5xKJnVYeTkCtva5/eP5KBQ/NqVxMtj1C8j5erCIAc
qO5EpChI3Y9JOiA1R6oorfBsPhW8ELyDDQmofpj+nkTGyLwc8CQov4MeF8GLbMVH
VZ4NeT2nDjLn34zdGPBViu+rwEVuCxDzGkXZz1Q8Sm7a3WxHH91kufYIV5/VpUtw
WMJyOKoPAgMBAAECggEAOBPnh36ApbwvLHMg95JTDjpnjzuFR78kZGXYA8dGJb1h
5JfB+kIBS6swwCvY4HxzWKsQkykdxI1dXYpgwbenomFUgU7Cq/E85JgrOxYOb8fB
gzL7j0kw/pOUKrV5wloMXacYo1H170UFAPn1Qs5pjwYxLvqpLlxgX1fw33Uqjzqx
LC0riQUJmKbqchURbmhE+FlBequY/ZqtsegeEs7YEKULq+oRZMT1Dv7kzIZiQVCB
Zl7L5aJDHdrzmAHdX4ernQIlCqzLau71smq35jxRsBJeLxcXZ3s97mGjGt6wKHpi
1qpNEKZGzolPNKLOCWl6iUN6u2asxBVPjF13ycFVYQKBgQD3sVX75iSQz9/JymVo
CFhV0bBs93NgTwD1xsnhkfEj7g273sZdzEwRKw/wbKK5V2Y3X+MF6sw4De5BCdnE
0q2nY/IPHxREaQkBUTqiomhZofcjVvMb1KUAZRmzpf8s0GftC4NCYmyj5SWcskDl
bHK+FnaTxMoohwO7LI6AXQTO3wKBgQDjfduu3cttOwtFo+k1La/CHh/2NzH2XReq
R08FK88LyIbPPwFJB0QXJNZhFxfE1xAB8UkqU0Y27a5xn7Ao7enxPH0U3P6FELDQ
nrDVSbsMJz/HZ/9940sWE2/RAxMHyz3toNSDxB4OLPqR03Txvwo4NYBrxYgomIvE
VmsW53r60QKBgDbJYtt07GGbmURpIEUCk4dn+j4f/cUvtGaosb7TKSNpZGBJxla4
+ZpSjBQB83xLeLCG5RBJ/yHm8uwv4ZtHfmGoGcJ+fV0kXnmMHgwbIDmzOZb3hI6D
Wnvb4PN7pBWlg1dLVJ06aA1YghlG1FPqyPaCwXet74lHU+vyzxJVgNrBAoGBAKGX
TOyxb0ZvolodTBISGb5xV5K1MjAlgCHb9S1fKPrdvaSIxIjzOFKYQA9HMAMKILzA
f5ApBr8NpdPf8mmgVYSdytt+/uvWJ0KI9mMKtGq1loA2Ry1MvpE3zT1SSbpHi/23
OD1AmXjISlQ7N95gLQowNsBW15i+gsk7+WWTl6+BAoGAJMXu1uXyrhs85yBwaTWn
XLNPC2BtBZ13vvWPrLkveHzDkiQNeI7nK0UFnWxG6NgDIha2O0aqvMJzhLusruTk
BadH6QchpA6Z+SygnUJjXuo+6VagwV9afwY0Wo7AHJy3yfbpbPf0idhboa4rDXgs
2D60PA+UJ5yDDH+BKu6EzXU=
-----END PRIVATE KEY-----";
    let rsa_enc = rsa::rsa_enc(
        plain_text.as_bytes(),
        RsaKey::Pkcs8 {
            key: rsa_pub_key.to_string(),
        },
    )?;
    let rsa_dec = rsa::rsa_dec(
        rsa_enc.as_slice(),
        RsaKey::Pkcs8 {
            key: rsa_private_key.to_string(),
        },
    )?;
    println!("rsa dec is {:?}", String::from_utf8(rsa_dec));

    let sign = rsa::rsa_sign(
        plain_text.as_bytes(),
        RsaKey::Pkcs8 {
            key: rsa_private_key.to_string(),
        },
    )?;
    let verify = rsa::rsa_verify(
        plain_text.as_bytes(),
        sign.as_slice(),
        RsaKey::Pkcs8 {
            key: rsa_pub_key.to_string(),
        },
    )?;
    println!("verify OK  ");
    Ok(())
}
