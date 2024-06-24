#[allow(clippy::needless_range_loop)]
fn main() {
    // 空数组
    let array1 = [10i32; 0];
    println!("{:?}", array1);

    // 元素列表
    let array2 = [1i32, 2, 3];
    println!("{:?}", array2);

    // 表达式
    let mut array3 = [0i32; 10];
    println!("{:?}", array3);

    // 修改元素
    array3[0] = 100;

    // 利用索引遍历，索引类型为usize
    for i in 0..10usize {
        println!("{}", array3[i]);
    }

    let a = "hello".to_string().as_str();
}

// 声明一个名字为 country 的 mod 实体，产生一个代码域
mod country {
    #![allow(unused)]
    // 声明一个名字为 country 的 mod 实体，产生一个代码域
    mod province {

        // 声明一个名字为 Province 的 struct 实体，可见性为 pub
        pub struct Province {
            name: String,
        }

        // 声明一个名字为 new 的 fn 实体，可见性为 pub
        pub fn new(s: &str) -> Province {
            Province {
                name: String::from(s),
            }
        }
    }

    mod factory {
        // 通过路径引入名字为 province 的 mod 实体
        // 通过路径引入名字为 Province 的 struct 实体
        // 通过路径引入名字为 new 的 fn 实体
        use super::province::{
            self,
            new,
            Province,
        };
    }
}
