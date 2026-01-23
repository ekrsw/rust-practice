enum CustomOption<T> {
    CustomSome(T),
    CustomNone,
}

fn main() {
    let some_number: CustomOption<String> = CustomOption::CustomSome(String::from("HELLO"));
    let none_number: CustomOption<String> = CustomOption::CustomNone;

    export_custom(some_number);
    export_custom(none_number);
}

fn export_custom<T: std::fmt::Display>(value: CustomOption<T>) {
    match value {
        CustomOption::CustomSome(value) => println!("Some_number is {}", value),
        CustomOption::CustomNone => println!("some_number is None"),
    };
}