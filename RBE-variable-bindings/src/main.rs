fn main() {
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        // 被不可变的 `_mutable_integer` 遮蔽， 下面这一行会报错，`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
    }
    _mutable_integer = 3;
}
