pub mod decode {
    use openssl::aes::{AesKey, aes_ige};
    use base64::{encode, decode};
    use std::io::Write;
    pub fn get_id(url: &str) -> String {
        url.split("id=").last().unwrap().to_string().split("&title=").next().unwrap().to_string()
    }

    pub fn get_ajax(_url: &str) -> String {
        // broken
        let key = b"\x00\x00\x00\x00\x00\x08\xd5\x11\x04c<\xbb\xb9\x17)\x9bv i.\x06F\xfa\xd3\xf4\xa89XPc\x1a\xcd";
        let key = AesKey::new_encrypt(key).unwrap();
        let mut output = [0u8; 64];
        let mut iv = *b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\xb1\xdb\x83RK\x9a\x83\xd2c\x1a\x08\xbc\xb5";
        let mut input: [u8; 64] = [0u8; 64];
        let _closure = |mut bytes: &mut[u8], s: &str| {
            bytes.write(s.as_bytes()).unwrap();
        };
        _closure(&mut input, _url);
        println!("in len {} out len {}", get_id(_url).as_bytes().len(), output.len());
        aes_ige(&input, &mut output, &key, &mut iv, openssl::symm::Mode::Encrypt);
        encode(&output).to_string()
    }
}