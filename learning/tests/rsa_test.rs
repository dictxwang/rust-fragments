
// cargo test --release -- --nocapture -- --test test_rsa_01
#[test]
pub fn test_rsa_01() {
    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

// cargo test --release -- --nocapture -- --test test_rsa_02
#[test]
pub fn test_rsa_02() {

    let private_pkcs8_pem  =
"-----BEGIN PRIVATE KEY-----
XXXXXX
-----END PRIVATE KEY-----";

    use rsa::RsaPrivateKey;
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::pkcs1v15::SigningKey;
    use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
    use rsa::sha2::Sha256;
    use base64::{Engine as _, engine::general_purpose};

    let private_key = RsaPrivateKey::from_pkcs8_pem(private_pkcs8_pem).unwrap();
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let verifying_key = signing_key.verifying_key();

    let mut rng = rand::thread_rng();

    // Sign
    let data = b"hello world"; 
    let signature = signing_key.sign_with_rng(&mut rng, data);
    assert_ne!(signature.to_bytes().as_ref(), data.as_slice());

    let sign_text = general_purpose::STANDARD.encode(signature.clone().to_bytes());
    println!("signature: {:?}", sign_text);

    // Verify
    verifying_key.verify(data, &signature).expect("failed to verify");
}

// cargo test --release -- --nocapture -- --test test_rsa_03
#[test]
fn test_rsa_03() {

    let private_pkcs8_pem  =
"-----BEGIN PRIVATE KEY-----
XXXXXX
-----END PRIVATE KEY-----";

    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
    use rsa::pkcs8::DecodePrivateKey;
    use base64::{Engine as _, engine::general_purpose};

    let mut rng = rand::thread_rng();

    // Generate Key-Pair
    // let bits = 2048;
    // let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // let pub_key = RsaPublicKey::from(&priv_key);

    // Use Exists Key
    let priv_key = RsaPrivateKey::from_pkcs8_pem(private_pkcs8_pem).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);
    
    let enc_text = general_purpose::STANDARD.encode(enc_data.clone());
    println!("{:?}", enc_text);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}