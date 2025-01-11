fn main() {}

#[cfg(test)]
mod test {
    use std::{mem, ptr, rc::Rc};

    ///
    /// 向量v的类型Vec<Rc<i32>>,有两个元素100,200
    /// 1.释放向量的第二个元素,并修正向量长度
    /// 2.断言修改后的向量是否正确
    /// 3.利用Rc的弱引用断言第二个元素是否释放
    ///
    #[test]
    fn test_drop_in_place() {
        let last = Rc::new(100);
        let weak = Rc::downgrade(&last);

        let mut v = vec![Rc::new(200), last];
        let ptr = &mut v[1] as *mut _;

        unsafe {
            v.set_len(1);
            ptr::drop_in_place(ptr);
        }

        assert_eq!(v, vec![Rc::new(200)]);
        assert!(weak.upgrade().is_none());
    }
}
