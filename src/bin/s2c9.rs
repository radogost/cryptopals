fn pcks7_padding(bytes: &[u8], block_length: usize) -> Vec<u8> {
    let missing_bytes = block_length - (bytes.len() % block_length);
    let mut padding = vec![missing_bytes as u8; missing_bytes];

    let mut padded = Vec::from(bytes);
    padded.append(&mut padding);

    padded
}

fn main() {
    let s = "YELLOW SUBMARINE";
    let bytes = s.as_bytes();
    let padded = pcks7_padding(&bytes, 20);

    let original_bytes = &padded[..s.len()];
    let padding = &padded[s.len()..];

    println!("Padded: {:x?}", padded);

    assert_eq!(original_bytes, bytes);
    assert_eq!(padding.len(), 4);
    assert_eq!(padding.iter().all(|&byte| byte == 4), true);
}
