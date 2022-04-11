use attribute_macros::add_attributes;

#[add_attributes]
#[derive(Debug)]
struct Struct {}

fn main() {
    let s = Struct {
        val1: -1,
        val2: "qwerty".to_string(),
        val3: 1,
        val4: 1.2,
    };

    println!("{:#?}", s);
}
