//! 字节操作工具模块
//!
//! 提供字节数组操作功能，与 Java Bytes.java 功能对应

/// 字节操作工具结构体
pub struct Bytes;

impl Bytes {
    /// 将 i32 转换为字节数组 (小端序)
    pub fn int_to_bytes(value: i32) -> Vec<i32> {
        let bytes = value.to_le_bytes();
        vec![bytes[0] as i32, bytes[1] as i32, bytes[2] as i32, bytes[3] as i32]
    }

    /// 将 i64 转换为字节数组 (小端序)
    pub fn long_to_bytes(value: i64) -> Vec<i32> {
        let bytes = value.to_le_bytes();
        vec![
            bytes[0] as i32, bytes[1] as i32, bytes[2] as i32, bytes[3] as i32,
            bytes[4] as i32, bytes[5] as i32, bytes[6] as i32, bytes[7] as i32,
        ]
    }

    /// 从字节数组读取 i32 (小端序)
    pub fn bytes_to_int(bytes: &[i32]) -> i32 {
        if bytes.len() >= 4 {
            ((bytes[0] & 0xFF) as i32)
                | (((bytes[1] & 0xFF) as i32) << 8)
                | (((bytes[2] & 0xFF) as i32) << 16)
                | (((bytes[3] & 0xFF) as i32) << 24)
        } else {
            0
        }
    }

    /// 合并两个字节数组
    pub fn add_bytes(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut result = a;
        result.extend(b);
        result
    }

    /// 将字节数组转换为十六进制字符串
    pub fn bytes_to_hex(bytes: &[i32]) -> String {
        bytes.iter()
            .map(|b| format!("{:02X}", b & 0xFF))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 合并两个字节数组 (便捷函数)
pub fn add_bytes(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    Bytes::add_bytes(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_conversion() {
        let value: i32 = 12345;
        let bytes = Bytes::int_to_bytes(value);
        assert_eq!(Bytes::bytes_to_int(&bytes), value);
    }

    #[test]
    fn test_add_bytes() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let result = add_bytes(a, b);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }
}
