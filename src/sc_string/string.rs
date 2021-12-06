    ///
    /// String &str
    ///
    /// String (Vec<u8> UTF-8序列) 堆分配 可增长 且非零结尾
    /// &str (&[u8]) 指向有效 UTF-8序列 的切片 并可查看String内容
    ///
    #[test]
    fn test_string() {
        //""
        let s1: &str = "";
        let s2 = "".to_string();
        let s3 = b"sdflkj";
        let s4 = r#"sdlk"f""#;
        let s5 = String::new();
        let s7 = String::from("");

        //[vector]reverse() [Chars]rev()
        let pangram: &str = "the quick brown fox jumps over the lazy dog";
        let s6: String = pangram.chars().rev().collect();

        println!("{:?}", s6);
        let unicode_codepoint = "\u{211D}";
        // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
        // 可使用的 # 的数目没有限制。但是必须前后数量一致
        let longer_delimiter = r###"A string with "# in it. And even "##"###;
        println!("{:?}", longer_delimiter);
    }