
    #[test]
    fn test_rand() {
        
        let key = "6661428";
        let mut key_bytes = [0u8; 16];
        key_bytes[..key.len()].copy_from_slice(&key.as_bytes());
        let nonce = [0u8; 8];
        let counter = 0;

        let mut prng = Aes128Ctr64::from_seed(Aes128Ctr64Seed::new(key_bytes, nonce, counter));
        
        let mut seckey = [0u8; 16]; 
        prng.fill(&mut seckey);

        println!("seckey: {:?} hex: {:?}", seckey, hex::encode_upper(seckey));
    }
#[test]
    fn test_rand() {
        
        let key = "6661428";
        let mut key_bytes = [0u8; 16];
        key_bytes[..key.len()].copy_from_slice(&key.as_bytes());
        let counter = 0;

        let mut prng = Aes128Ctr128::from_seed(Aes128Ctr128Seed::new(key_bytes, counter));
        
        let mut seckey = [0u8; 16]; 
        prng.fill(&mut seckey);

        println!("seckey: {:?} hex: {:?}", seckey, hex::encode_upper(seckey));
    }

fn test()
{
  let key = "6661428";
        let mut key_bytes = [0u8; 32];
        key_bytes[..key.len()].copy_from_slice(&key.as_bytes());

        // let mut seed = ChaCha8Rng::from_seed(key_bytes);
        let mut seed = ChaCha12Rng::from_seed(key_bytes);
        let mut seckey = [0u8; 16]; 
        seed.fill(&mut seckey);

        println!("seckey: {:?} hex: {:?}", seckey, hex::encode_upper(seckey));

}
    //     String key = "6661428";
	// SecureRandom random=SecureRandom.getInstance("SHA1PRNG");
	// random.setSeed(key.getBytes());
			
	// KeyGenerator kgen = KeyGenerator.getInstance("AES");
	// kgen.init(128, random);
	// SecretKey secretKey = kgen.generateKey();
	// byte[] enCodeFormat = secretKey.getEncoded();
			
	// System.out.println("enCodeFormat : " + Arrays.toString(enCodeFormat)+" hex: " + HexFormat.of().formatHex(enCodeFormat));

