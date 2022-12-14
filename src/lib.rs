/*
const fn sample() -> &'static str {
    include_str!(concat!(
        concat!("../samples/", env!("CARGO_BIN_NAME")),
        ".txt"
    ))
}
*/
#[macro_export]
macro_rules! sample {
    () => {
        include_str!(concat!(
            concat!("../samples/", env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}
#[macro_export]
macro_rules! input {
    () => {
        include_str!(concat!(
            concat!("../inputs/", env!("CARGO_BIN_NAME")),
            ".txt"
        ))
    };
}
