extern crate bs58;
extern crate sha2;

use sha2::{Digest, Sha256};

const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const BASE58_CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn base_encode(mut num: u64, base_chars: &[u8]) -> String {
    let base = base_chars.len() as u64;
    let mut result = Vec::new();

    while num > 0 {
        let rem = (num % base) as usize;
        result.push(base_chars[rem]);
        num /= base;
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

fn encrypt_and_encode(input: &str) -> String {
    // 创建一个 SHA-256 哈希器
    let mut hasher = Sha256::new();

    // 更新哈希器的输入数据
    hasher.update(input.as_bytes());

    // 计算哈希值
    let result = hasher.finalize();

    // 将哈希值转换为 Base58 编码的字符串
    let hash_base58 =
        bs58::encode(result.iter().map(|byte| *byte).collect::<Vec<u8>>()).into_string();

    hash_base58
}

fn main() {
    // 创建一个 SHA-256 哈希器
    let mut hasher = Sha256::new();

    // 输入数据
    let data = b"hello, world!";

    // 更新哈希器的输入数据
    hasher.update(data);

    // 计算哈希值
    let result = hasher.finalize();

    // 将哈希值转换为十六进制字符串
    let hash_hex = result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    println!("SHA-256 Hash: {}", hash_hex);

    let mut num_to_encode = 12345;
    for i in 0..1000 {
        num_to_encode = i;
        let base62_encoded = base_encode(num_to_encode, BASE62_CHARS);
        println!("Base62 encoded: {},num:{}", base62_encoded, num_to_encode);

        let base58_encoded = base_encode(num_to_encode, BASE58_CHARS);
        println!("Base58 encoded: {}", base58_encoded);
    }
    for i in 0..10 {
        let input = &i.to_string(); //"hello, world!";
        let encrypted_and_encoded = encrypt_and_encode(input);

        println!("Input: {}", input);
        println!("Encrypted and Base58 Encoded: {}", encrypted_and_encoded);
    }
}
