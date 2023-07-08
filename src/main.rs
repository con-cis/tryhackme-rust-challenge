use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    match read_flag_from_file() {
        Ok(flag_encoded) => {
            println!("Flag in chal.text: {}", flag_encoded);

            // Convert String to Vec<u8>
            let buffer: Vec<u8> = flag_encoded.into_bytes();

            // Convert From ROT13 to Vec<u8>
            let decrypted_rot13: Vec<u8> = rot13_decrypt(&buffer);
            let decrypted_rot13_str: String =
                String::from_utf8_lossy(&decrypted_rot13).into_owned();
            println!("Decrypted from ROT13: {:?}", decrypted_rot13_str);

            // Convert From Base64 to Vec<u8>
            let mut buffer: Vec<u8> = Vec::new();
            base64_decode(&decrypted_rot13, &mut buffer);
            let base64_decoded_str: String = String::from_utf8_lossy(&buffer).into_owned();
            println!("Decripted from Base64: {:?}", base64_decoded_str);

            // Convert From Vec<u8> to ROT13
            let decrypted_rot13_second: Vec<u8> = rot13_decrypt(&buffer);
            let decrypted_rot13_str: String =
                String::from_utf8_lossy(&decrypted_rot13_second).into_owned();
            println!("Decrypted from ROT13: {:?}", decrypted_rot13_str);
        }
        Err(error) => println!("Error reading chal.txt: {}", error),
    }
}

fn read_flag_from_file() -> Result<String, io::Error> {
    let mut f: File = File::open("chal.txt")?;
    let mut s: String = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn rot13_decrypt(data: &[u8]) -> Vec<u8> {
    rot13::rot13_slice(rot13::Mode::Decrypt, data).to_vec()
}

fn base64_decode(decrypted_rot13: &[u8], buffer: &mut Vec<u8>) {
    use base64::engine::general_purpose;
    use base64::Engine;
    general_purpose::STANDARD
        .decode_vec(decrypted_rot13, buffer)
        .unwrap();
}