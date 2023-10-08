// fn main() {
//     if let Err(e) = wcr::get_args().and_then(wcr::run) {
//         eprintln!("{}", e);
//         std::process::exit(1);
//     }
// }


fn main() {
    // let options = Some(5);
    // match options {
    //     Some(number) =>  {
    //         println!("这是经过规则匹配{}", number);
    //     },
    //     None => {
    //         println!("这是一个错误")
    //     },
    // }

    // if options == Some(5) {
    //     println!("走到这里了")
    // }

    // if let Some(6) = options {
    //     println!("这是一个语法糖")
    // }
    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

