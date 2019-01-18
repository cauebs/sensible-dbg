#![allow(clippy::match_bool)]

#[macro_export]
macro_rules! dbg {
    ($val:expr) => {
        match $val {
            tmp => {
                #[cfg(debug_assertions)]
                {
                    eprintln!(
                        "[{}:{}] {} = {:#?}",
                        file!(),
                        line!(),
                        stringify!($val),
                        &tmp
                    );
                }
                tmp
            }
        }
    };
}
