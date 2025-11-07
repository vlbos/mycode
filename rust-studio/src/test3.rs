用rand和rand_chacha

Cargo.toml

rand_chacha = "0.3.1"
rand = {version = "0.8.5", features = ["std"]}
        let key = "6661428";
        let mut key_bytes = [0u8; 32];
        key_bytes[..key.len()].copy_from_slice(&key.as_bytes());

        // let mut seed = ChaCha8Rng::from_seed(key_bytes);
        let mut seed = ChaCha12Rng::from_seed(key_bytes);
        let mut seckey = [0u8; 16]; 
        seed.fill(&mut seckey);

        println!("seckey: {:?} hex: {:?}", seckey, hex::encode_upper(seckey));
结果：

seckey: [185, 231, 201, 161, 57, 20, 227, 186, 148, 173, 65, 160, 43, 224, 136, 239] hex: "B9E7C9A13914E3BA94AD41A02BE088EF"
都与Java的不一致：

        String key = "6661428";
	SecureRandom random=SecureRandom.getInstance("SHA1PRNG");
	random.setSeed(key.getBytes());
			
	KeyGenerator kgen = KeyGenerator.getInstance("AES");
	kgen.init(128, random);
	SecretKey secretKey = kgen.generateKey();
	byte[] enCodeFormat = secretKey.getEncoded();
			
	System.out.println("enCodeFormat : " + Arrays.toString(enCodeFormat)+" hex: " + HexFormat.of().formatHex(enCodeFormat));

