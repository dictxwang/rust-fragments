
// cargo test --release -- --nocapture -- --test test_ed25519_01
#[test]
fn test_ed25519_01() {

    use rand_core::OsRng;
    use ed25519_dalek::{SigningKey, Signature, Signer};
    use pkcs8::EncodePrivateKey;
    use base64::{Engine as _, engine::general_purpose};

    let mut csprng = OsRng;

    // Use Generate Key-Pair
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    println!("skey bytes: {:?}", signing_key.as_bytes());
    println!("skey bytes: {:?}", signing_key.to_pkcs8_der().unwrap());

    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = signing_key.sign(message);
    println!("signature: {:?}", signature.to_string());

    let base64_value = general_purpose::STANDARD.encode(signature.to_bytes());
 
    println!("base64: {:?}", base64_value);
}

#[test]
fn test_ed25519_02() {

    use ed25519_dalek::{SigningKey, Signature, Signer};
    use base64::{Engine as _, engine::general_purpose};
    use pkcs8::DecodePrivateKey;
    

    let private_pkcs8_pem = 
"-----BEGIN PRIVATE KEY-----
XXXXX
-----END PRIVATE KEY-----";

    let signing_key = SigningKey::from_pkcs8_pem(private_pkcs8_pem).unwrap();

    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = signing_key.sign(message);
    println!("signature: {:?}", signature.to_string());

    let base64_value = general_purpose::STANDARD.encode(signature.to_bytes());
 
    println!("base64: {:?}", base64_value);
}
