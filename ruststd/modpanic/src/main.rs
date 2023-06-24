use std::panic::{
    self,
    catch_unwind,
};

fn main() {
    // 设置恐慌钩子,传入PanicInfo
    panic::set_hook(Box::new(|panic_info| {
        println!("panic: {panic_info}");
    }));

    let result = catch_unwind(|| {
        println!("Hello normal!");
    });

    assert!(result.is_ok());

    let result = catch_unwind(|| {
        panic!("Hello panic!");
    });

    assert!(result.is_err());
    println!("========2========");

    if let Err(err) = result {
        panic::resume_unwind(err);
    }
    println!("catch_unwind: {:?}", result);
}
