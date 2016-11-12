use std::error::Error as StdError;
use utils::string::repeat_str;

pub fn stack_printer(e: &StdError) {
    println!("{}", e);

    let mut e: &StdError = e;
    let mut level = 1;

    while let Some(cause) = e.cause() {
        println!("{}→ {}", repeat_str(" ", level * 4), cause);
        e = cause;
        level += 1;
    }
}
