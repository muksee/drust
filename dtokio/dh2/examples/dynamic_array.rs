use std::alloc::{
    alloc,
    Layout,
};

const N: usize = 3;

fn correct_write(ptr_array: *mut String, flag_array: [bool; N], position: usize) {
    // 指针前进
    let ptr_new = unsafe { ptr_array.add(position) };

    // 通过标记数组判断位置是否已经写入
    // 1.如果初始化了,则先读取再写入
    // 2.如果未初始化,则直接写入
    if flag_array[position] {
        unsafe {
            let a = ptr_new.read();
            println!("remove old element value: {:?}", a);
            ptr_new.write(format!("re write {}", position));
        }
    } else {
        unsafe {
            ptr_new.write(format!("re write {}", position));
        }
    }
}

fn main() {
    let mut flag_array = [false; N];

    let layout = Layout::array::<String>(N).unwrap();
    let ptr_array = unsafe { alloc(layout) } as *mut String;

    // 只初始化前N-1个元素,最后一个(第N个)没有初始化
    for (offset, index) in (0..N - 1).enumerate() {
        unsafe {
            ptr_array
                .add(offset)
                .write(format!("{} Element", index));
            flag_array[index] = true;
        }
    }
    // 注意:
    // 只有前N-1个元素初始化了,第N个没有初始化,所以只能打印前(N-1)个,如果打印N个会报错
    println!("init array: {:?}", unsafe { &*(ptr_array as *const [String; N-1]) });

    // 对位置0写入 - 已经初始化过,会先read再write
    let position = 0;
    correct_write(ptr_array, flag_array, position);
    // 对位置1写入 - 已经初始化过,会先read再write
    let position = 1;
    correct_write(ptr_array, flag_array, position);
    // 对位置2写入 - 没有初始化,会跳过read直接write
    let position = 2;
    correct_write(ptr_array, flag_array, position);

    println!("rewrite array: {:?}", unsafe {
        &*(ptr_array as *const [String; N])
    });
}
