use did_types::DummyStruct;

fn main() {
    let dummy_obj = DummyStruct("Hello, world!".to_string());
    println!("{}", dummy_obj.0);
}
