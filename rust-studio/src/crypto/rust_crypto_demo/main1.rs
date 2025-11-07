 use sha1::Sha1;
use sha2::{Sha256, Digest};
 use rsa::{RsaPublicKey, Oaep};
use rand_core::CryptoRng;
 use base64ct::{Base64, Encoding};
 use crypto_bigint::BoxedUint;
 use rand_chacha::{
        rand_core::{ SeedableRng},//RngCore
        ChaCha8Rng,
    };

fn main()
{ let n_bytes = Base64::decode_vec("seAOhmYFAjH6NOaB54dboqw86uPXV/oK9ayJGV4mVClbvsDBJmF3bVkOaVMp9ogcFJTFFSy5g2HsTZIfHyuQVUJADb+BeRnkYrYhRvNJOKj2pcDbkxYe9XGMx8pIvxkDFnIpusb3gUsuzMUAU5qIstjwQKzuD51c6uJi0HAtQkr6Wmlt34SX7xkD/MfRuTu9uqmHmkiiJaCDHB2reYTPguetSWfuvp1qBJDNgSsp7BjwYANWldyrmZ8cLXEXYMUG5vtsWMxUzl8ertEr6kbnGM0OJghNuEtittW/dfTPvk683R1jj0hNaMzvHK8xYldUlLuwmWCYIIvpHBaA/w+FwQ==").unwrap();
 let e_bytes = Base64::decode_vec("AQAB").unwrap();
 let n = BoxedUint::from_be_slice(&n_bytes, 2048).unwrap();
 let e = BoxedUint::from_be_slice(&e_bytes, 32).unwrap();

//  let mut rng = rand::thread_rng();
 let mut rng = ChaCha8Rng::from_seed([42; 32]);
 let key = RsaPublicKey::new(n, e).unwrap();
 let padding = Oaep::new::<Sha256>();
 let encrypted_data = key.encrypt(&mut rng, padding, b"secret").unwrap();
}