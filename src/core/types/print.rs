use super::string::String as SafeString;

pub trait Printable {
    fn to_printable_std_string(&self) -> std::string::String;
}

impl Printable for SafeString {
    fn to_printable_std_string(&self) -> std::string::String {
        self.to_std_string()
    }
}

impl Printable for str {
    fn to_printable_std_string(&self) -> std::string::String {
        self.to_string()
    }
}

impl Printable for std::string::String {
    fn to_printable_std_string(&self) -> std::string::String {
        self.clone()
    }
}

impl<T: Printable + ?Sized> Printable for &T {
    fn to_printable_std_string(&self) -> std::string::String {
        (*self).to_printable_std_string()
    }
}

macro_rules! impl_printable_for_display {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Printable for $ty {
                fn to_printable_std_string(&self) -> std::string::String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_printable_for_display!(
    bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize
);

pub fn format_printable<T: Printable + ?Sized>(value: &T) -> std::string::String {
    value.to_printable_std_string()
}

pub fn print_any<T: Printable + ?Sized>(value: &T) {
    std::print!("{}", format_printable(value));
}

pub fn printl_any<T: Printable + ?Sized>(value: &T) {
    print_any(value);
    std::println!();
}

pub fn print<T: Printable + ?Sized>(value: &T) {
    print_any(value);
}

pub fn printl<T: Printable + ?Sized>(value: &T) {
    printl_any(value);
}

#[cfg(test)]
mod tests {
    use super::format_printable;
    use crate::core::types::String as SafeString;

    #[test]
    fn test_format_printable_primitives() {
        assert_eq!(format_printable(&42), "42");
        assert_eq!(format_printable(&false), "false");
    }

    #[test]
    fn test_format_printable_safe_string_and_ref() {
        let text = SafeString::from("safe");
        assert_eq!(format_printable(&text), "safe");
        let text_ref = &text;
        assert_eq!(format_printable(&text_ref), "safe");
    }
}
