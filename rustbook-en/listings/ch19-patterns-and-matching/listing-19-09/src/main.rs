fn main() {
    let some_option_value: Option<i32> = None;
    // ANCHOR: here
    let Some(x) = some_option_value else {
        return;
    };
    // ANCHOR_END: here
}
