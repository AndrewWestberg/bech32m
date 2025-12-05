use bech32::{Bech32m, Hrp};
use clap::Parser;
use std::io::{self, BufRead};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(
    name = "bech32m",
    about = "Convert to and from bech32m strings. Data are read from standard input.",
    version = VERSION,
    after_help = r#"Supported encoding formats: Base16, Bech32m & Base58.

Examples:
  To Bech32m:
    $ bech32m base16_ <<< 706174617465
    base16_1wpshgcgvnlscf

    $ bech32m base58_ <<< Ae2tdPwUPEYy
    base58_1p58rejhd9592uusa8rvpy

    $ bech32m new_prefix <<< old_prefix1wpshgcge5yvsv
    new_prefix1wpshgcg703stt

  From Bech32m:
    $ bech32m <<< base16_1wpshgcgvnlscf
    706174617465"#
)]
struct Cli {
    /// An optional human-readable prefix (e.g. 'addr').
    /// When provided, the input text is decoded from various encoding formats and re-encoded to bech32m using the given prefix.
    /// When omitted, the input text is decoded from bech32m to base16.
    #[arg(name = "PREFIX")]
    prefix: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    let input = match input {
        Some(Ok(line)) => line.trim().to_string(),
        Some(Err(e)) => {
            eprintln!("Error reading input: {}", e);
            std::process::exit(1);
        }
        None => {
            eprintln!("No input provided");
            std::process::exit(1);
        }
    };

    if input.is_empty() {
        eprintln!("No input provided");
        std::process::exit(1);
    }

    match cli.prefix {
        Some(prefix) => {
            // Encode mode: decode input and re-encode with new prefix
            match encode_to_bech32m(&input, &prefix) {
                Ok(encoded) => println!("{}", encoded),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            // Decode mode: decode bech32m to hex
            match decode_bech32m(&input) {
                Ok(hex_str) => println!("{}", hex_str),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

/// Decode a bech32m string and return the data as hex
fn decode_bech32m(input: &str) -> Result<String, String> {
    let (_, data) = bech32::decode(input).map_err(|e| format!("Invalid bech32m string: {}", e))?;
    Ok(hex::encode(data))
}

/// Try to detect the input format and encode it to bech32m with the given prefix
fn encode_to_bech32m(input: &str, prefix: &str) -> Result<String, String> {
    let hrp = Hrp::parse(prefix).map_err(|e| format!("Invalid prefix: {}", e))?;

    // Try to decode as bech32/bech32m first
    if let Ok((_, data)) = bech32::decode(input) {
        let encoded =
            bech32::encode::<Bech32m>(hrp, &data).map_err(|e| format!("Encoding error: {}", e))?;
        return Ok(encoded);
    }

    // Try to decode as hex (base16)
    if let Ok(data) = hex::decode(input) {
        let encoded =
            bech32::encode::<Bech32m>(hrp, &data).map_err(|e| format!("Encoding error: {}", e))?;
        return Ok(encoded);
    }

    // Try to decode as base58
    if let Ok(data) = bs58::decode(input).into_vec() {
        let encoded =
            bech32::encode::<Bech32m>(hrp, &data).map_err(|e| format!("Encoding error: {}", e))?;
        return Ok(encoded);
    }

    Err("Unable to decode input. Supported formats: Base16, Bech32m, Base58".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_mn_shield_addr() {
        let bech32m_str = "mn_shield-addr_test1dfv46yhqklvgh4kzaw9p8dpezydetjeccssc7y2p32keeaqeuy4sxqqc60ndd9aahqlyyr7k8rhq5l2f7kc3y28759geed4clwzgdlg0ucgahz2x";
        let result = decode_bech32m(bech32m_str);
        assert!(
            result.is_ok(),
            "Failed to decode bech32m string: {:?}",
            result
        );
        let hex_data = result.unwrap();
        // Verify it's valid hex
        assert!(!hex_data.is_empty(), "Decoded data should not be empty");
        println!("Decoded hex: {}", hex_data);
        assert_eq!(hex_data, "6a595d12e0b7d88bd6c2eb8a13b439111b95cb38c4218f11418aad9cf419e12b030018d3e6d697bdb83e420fd638ee0a7d49f5b11228fea1519cb6b8fb8486fd0fe6", "Decoded hex does not match expected value");
    }

    #[test]
    fn test_roundtrip_hex() {
        let hex_input = "706174617465";
        let prefix = "test";

        let encoded = encode_to_bech32m(hex_input, prefix).unwrap();
        assert!(encoded.starts_with("test1"));

        let decoded = decode_bech32m(&encoded).unwrap();
        assert_eq!(decoded, hex_input);
    }

    #[test]
    fn test_roundtrip_bech32m() {
        let original = "test1wpshgct5v5hd5wlx";
        let new_prefix = "new";

        let encoded = encode_to_bech32m(original, new_prefix).unwrap();
        assert!(encoded.starts_with("new1"));

        // Both should decode to the same hex
        let original_hex = decode_bech32m(original).unwrap();
        let new_hex = decode_bech32m(&encoded).unwrap();
        assert_eq!(original_hex, new_hex);
    }

    #[test]
    fn test_base58_encoding() {
        let base58_input = "Ae2tdPwUPEYy";
        let prefix = "base58";

        let result = encode_to_bech32m(base58_input, prefix);
        assert!(result.is_ok(), "Failed to encode base58: {:?}", result);
        assert!(result.unwrap().starts_with("base581"));
    }

    #[test]
    fn test_invalid_bech32m() {
        let invalid = "not_a_valid_bech32m_string!!!";
        let result = decode_bech32m(invalid);
        assert!(result.is_err());
    }
}
