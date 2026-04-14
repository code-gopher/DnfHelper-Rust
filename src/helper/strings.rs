//! 字符串工具模块
//! 
//! 对应 Java: Strings.java

/// 将 Unicode 编码的字节数组转换为 ASCII 字符串
pub fn unicode_to_ascii(bytes: &[u8]) -> String {
    let mut result = String::new();
    
    // 每两个字节组成一个 Unicode 字符 (小端序)
    for i in (0..bytes.len()).step_by(2) {
        if i + 1 >= bytes.len() {
            break;
        }
        
        // 遇到双零结束符则停止
        if bytes[i] == 0 && bytes[i + 1] == 0 {
            break;
        }
        
        // 小端序：低位在前，高位在后
        let code = ((bytes[i + 1] as u16) << 8) | (bytes[i] as u16);
        
        if code != 0 {
            if let Some(c) = char::from_u32(code as u32) {
                result.push(c);
            }
        }
    }
    
    result
}

/// 将 ASCII 字符串转换为 Unicode 编码的字节数组 (小端序)
pub fn ascii_to_unicode(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    
    for c in s.chars() {
        let code = c as u32;
        // 小端序：低位在前，高位在后
        result.push((code & 0xFF) as u8);
        result.push(((code >> 8) & 0xFF) as u8);
    }
    
    // 添加结束符
    result.push(0);
    result.push(0);
    
    result
}

/// 将逗号分隔的字符串转换为整数数组
pub fn split_to_int_array(input: &str, separator: &str) -> Vec<i32> {
    input.split(separator)
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}

/// 取文本左边 N 个字符
pub fn get_left_text(text: &str, n: usize) -> String {
    text.chars().take(n).collect::<String>().trim().to_string()
}

/// 取文本右边 N 个字符
pub fn get_right_text(text: &str, n: usize) -> String {
    let len = text.chars().count();
    if n >= len {
        return text.to_string();
    }
    text.chars().skip(len - n).collect()
}

/// 字符串转整数
pub fn to_integer(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("转换整数失败：{}", e))
}

/// 数字转十六进制字符串 (大写)
pub fn to_hex(num: i64) -> String {
    format!("{:X}", num as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_to_ascii() {
        // 测试 "Hello" 的 Unicode 小端序编码
        let bytes = vec![72, 0, 101, 0, 108, 0, 108, 0, 111, 0, 0, 0];
        assert_eq!(unicode_to_ascii(&bytes), "Hello");
    }

    #[test]
    fn test_ascii_to_unicode() {
        let result = ascii_to_unicode("Hi");
        // 'H' = 72, 'i' = 105
        assert_eq!(result, vec![72, 0, 105, 0, 0, 0]);
    }

    #[test]
    fn test_split_to_int_array() {
        let result = split_to_int_array("1,2,3,4,5", ",");
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_get_left_text() {
        assert_eq!(get_left_text("Hello World", 5), "Hello");
    }

    #[test]
    fn test_get_right_text() {
        assert_eq!(get_right_text("Hello World", 5), "World");
    }

    #[test]
    fn test_to_integer() {
        assert_eq!(to_integer("123").unwrap(), 123);
        assert!(to_integer("abc").is_err());
    }

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(255), "FF");
        assert_eq!(to_hex(4096), "1000");
    }
}
