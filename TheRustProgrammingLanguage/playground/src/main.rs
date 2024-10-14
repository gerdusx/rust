fn main() {
    let my_char = 'á';

    if my_char.is_alphabetic() {
        println!("{} is an alphabetic character.", my_char);
    } else if my_char.is_alphanumeric() {
        println!("{} is a number", my_char);
    } else {
        println!("{} is a special character", my_char);
    }
}
