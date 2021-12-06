    ///
    /// Box<T> 智能指针 分配堆 指向堆分配的T类型的值
    /// * 运算符 (解箱)解引用 移除一层装箱
    /// Box::new(T) (装箱)
    ///
    #[test]
    fn test_box() {
        let boxed = Box::new(Box::new("sdaf"));
        let one = **boxed;
        println!("{:?}", one.to_owned())
    }