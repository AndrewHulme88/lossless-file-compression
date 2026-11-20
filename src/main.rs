fn main() {
    let input: &[u8] = b"AAAAABBCCC";

    let compressed = compress(input);

    println!("Original size: {} bytes", input.len());
    println!("Compressed size: {} bytes", compressed.len());
    println!("Compressed bytes: {:?}", compressed);

    let decompressed = decompress(&compressed);

    println!("Decompressed bytes: {:?}", decompressed);

    assert_eq!(input, decompressed.as_slice());
    println!("Success: the original bytes were recovered!");
}

fn compress(input: &[u8]) -> Vec<u8> {
    let mut compressed: Vec<u8> = Vec::new();

    if input.is_empty() {
        return compressed;
    }

    let mut current_byte = input[0];
    let mut count: u8 = 0;

    for &byte in input {
        if byte == current_byte && count < u8::MAX {
            count += 1;
        } else {
            compressed.push(count);
            compressed.push(current_byte);

            current_byte = byte;
            count = 1;
        }
    }

    compressed.push(count);
    compressed.push(current_byte);

    compressed
}

fn decompress(input: &[u8]) -> Vec<u8> {
    assert!(
        input.len() % 2 == 0,
        "Compressed input must contain count/value pairs"
    );

    let mut decompressed: Vec<u8> = Vec::new();

    for pair in input.chunks_exact(2) {
         let count = pair[0];
         let byte = pair[1];

         for _ in 0..count {
            decompressed.push(byte);
         }
    }

    decompressed
}