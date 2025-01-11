use std::borrow::Borrow;

/**
 * ### Borrow<T>:
 * 获取某类型值的底层数据的其它形式的引用
 * 方法borrow(&self)->&T
 *
 * ### ToOwned<Owned: Borrow<Self>>
 * 从引用创建具有所有权的某种类型的值..是Clone的一般形式.
 * 方法
 * 1.to_owned(&self) -> Self::Owned
 * 2.clone_into(&self, target: &mut Self::Owned)
 * 与Clone区别
 * 1.Clone: 只能通过借用 &T 克隆创建出同类型的具有所有权的 T 值
 * 2.ToOwned: 可以通过任意类型的借用&X 创建出具有所有权的Owned 类型值,比如 &[T]->Vec<T>
 */

fn main() {
    println!("Hello, world!");

    let ba = 100i32.borrow();
    let bb: &str = String::from("abc").borrow();
}
