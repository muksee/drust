use std::alloc::Layout;

fn main() {
    // 创建1
    let l = Layout::new::<i32>();
    println!(
        "size: {}, align: {}",
        l.size(),
        l.align(),
    );
    // 创建2
    let l = Layout::from_size_align(16, 8).unwrap();
    println!(
        "size: {}, align: {}",
        l.size(),
        l.align(),
    );
    unsafe {
        let l = Layout::from_size_align_unchecked(16, 9);
        println!(
            "size: {}, align: {}",
            l.size(),
            l.align(),
        );
    }
    // 创建3
    let  l = Layout::array::<i32>(10).unwrap();
    println!(
        "array -> size: {}, align: {}",
        l.size(),
        l.align(),
    );

    let s = String::from("Lily");
    let s = s.as_str();
    let l = Layout::for_value(s);
    println!(
        "size: {}, align: {}",
        l.size(),
        l.align(),
    );


    // 修改
    let l = l
        .align_to(32)
        .unwrap();
    println!(
        "size: {}, align: {}",
        l.size(),
        l.align(),
    );
    let l = l.pad_to_align();
    println!(
        "size: {}, align: {}",
        l.size(),
        l.align(),
    );

    // extend
    extend();
}

fn extend() {
    #[repr(C)]
    struct S {
        a: u64,
        b: u32,
        c: u16,
        d: u32,
    }

    let s = Layout::new::<S>();
    let u16 = Layout::new::<u16>();
    let u32 = Layout::new::<u32>();
    let u64 = Layout::new::<u64>();

    let fields = vec![u64, u32, u16, u32];

    let mut offsets = Vec::new();
    let mut l = Layout::from_size_align(0, 1).unwrap();
    for field in fields {
        let (new_layout, offset) = l
            .extend(field)
            .unwrap();
        l = new_layout;
        offsets.push(offset);
        println!(
            "{},{},{:?}",
            l.size(),
            l.align(),
            offsets
        );
    }
    println!("L1: {},{}", l.size(), l.align());
    let l = l.pad_to_align();
    println!("L2: {},{}", l.size(), l.align());
    println!("s: {},{}", s.size(), s.align());
}
