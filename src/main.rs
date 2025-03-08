use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: u32,
}

fn encode_unsigned_int32_to_leb128(input: u32) -> String {
    let raw_binary = format!("{:b}", input);

    let padded_length = ((raw_binary.len() + 7) / 7) * 7;
    let padded = format!("{:0>width$b}", input, width = padded_length);

    let split: Vec<&str> = padded
        .as_bytes()
        .chunks(7)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();

    let mut result = Vec::new();
    // let y = split.len() - 1;

    for (i, &group) in split.iter().enumerate() {
        let str = if i == 0 {
            format!("0{}", group)
        } else {
            format!("1{}", group)
        };
        let byte = u8::from_str_radix(&str, 2).unwrap();
        result.push(format!("{:02x}", byte));
    }

    result.join("")
}

fn main() {
    let args = Args::parse();

    let encoded = encode_unsigned_int32_to_leb128(args.input);

    println!(
        "Encoded: {input} -> LEB128: {encoded}",
        input = args.input,
        encoded = encoded
    );
}

#[cfg(test)]
mod tests {
    use crate::encode_unsigned_int32_to_leb128;

    #[test]
    fn test_encode_unsigned_int32_to_leb128() {
        //its ok
        assert_eq!(encode_unsigned_int32_to_leb128(0), "00");
        assert_eq!(encode_unsigned_int32_to_leb128(1), "01");

        // not ok
        assert_eq!(encode_unsigned_int32_to_leb128(127), "7f");
        assert_eq!(encode_unsigned_int32_to_leb128(128), "8001");
        assert_eq!(encode_unsigned_int32_to_leb128(1000), "07e8");
        assert_eq!(encode_unsigned_int32_to_leb128(u32::MAX), "ffffffff07");
    }
}
