

#[proc_macro]
pub fn to_string_from_str(array:[&str]) {
    for i in array {
        i.to_owned();
    }
}
