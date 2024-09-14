#![deny(clippy::all)]
fn main() {

    let string1 = String::from("Hello,");
    let string2 = String::from(" world!");

    let concatenated_string = concatenate_strings(&string1,&string2);
    println!("Concatenated string is : {}  ", concatenated_string)
}


fn concatenate_strings (s1: &str , s2: &str) -> String
{
    let mut new_string = String :: new();

    new_string.push_str(s1);
    new_string.push_str(s2);

    new_string

}

