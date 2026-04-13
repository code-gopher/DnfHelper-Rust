//! 字节操作工具模块
//! 
//! 提供字节数组操作功能

/// 将 i32 转换为字节数组 (小端序)
pub fn int_to_bytes(value: i32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// 将 i64 转换为字节数组 (小端序)
pub fn long_to_bytes(value: i64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// 将 f32 转换为字节数组 (小端序)
pub fn float_to_bytes(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// 从字节数组读取 i32 (小端序)
pub fn bytes_to_int(bytes: &[u8]) -> i32 {
    if bytes.len() >= 4 {
        i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    } else {
        0
    }
}

/// 从字节数组读取 i64 (小端序)
pub fn bytes_to_long(bytes: &[u8]) -> i64 {
    if bytes.len() >= 8 {
        i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    } else {
        0
    }
}

/// 从字节数组读取 f32 (小端序)
pub fn bytes_to_float(bytes: &[u8]) -> f32 {
    if bytes.len() >= 4 {
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    } else {
        0.0
    }
}

/// 合并两个字节数组
pub fn concat_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend_from_slice(a);
    result.extend_from_slice(b);
    result
}

/// 合并多个字节数组
pub fn merge_bytes(arrays: &[&[u8]]) -> Vec<u8> {
    let total_len: usize = arrays.iter().map(|a| a.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    for array in arrays {
        result.extend_from_slice(array);
    }
    result
}

/// 将字节数组转换为十六进制字符串
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// 从十六进制字符串解析字节数组
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.replace(" ", "").replace(",", "");
    
    if hex.len() % 2 != 0 {
        return Err("十六进制字符串长度必须为偶数".to_string());
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i+2];
        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
            bytes.push(byte);
        } else {
            return Err(format!("无效的十六进制字符：{}", byte_str));
        }
    }
    
    Ok(bytes)
}

/// 填充字节数组到指定长度
pub fn pad_bytes(bytes: &[u8], length: usize, padding: u8) -> Vec<u8> {
    let mut result = Vec::with_capacity(length);
    result.extend_from_slice(bytes);
    while result.len() < length {
        result.push(padding);
    }
    result
}

/// 截取字节数组的一部分
pub fn slice_bytes(bytes: &[u8], start: usize, length: usize) -> Vec<u8> {
    if start >= bytes.len() {
        return vec![];
    }
    
    let end = std::cmp::min(start + length, bytes.len());
    bytes[start..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_int_conversion() {
        let value: i32 = 12345;
        let bytes = int_to_bytes(value);
        assert_eq!(bytes_to_int(&bytes), value);
    }
    
    #[test]
    fn test_concat_bytes() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let result = concat_bytes(&a, &b);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }
    
    #[test]
    fn test_hex_conversion() {
        let hex = "48 65 6C 6C 6F";
        let bytes = hex_to_bytes(hex).unwrap();
        assert_eq!(bytes, vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
        assert_eq!(bytes_to_hex(&bytes), hex);
    }
}
