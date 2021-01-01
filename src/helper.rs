pub(crate) trait StringExt {
    fn trim_end_inplace(&mut self);
}

impl StringExt for String {
    fn trim_end_inplace(&mut self) {
        self.truncate(self.trim_end().len());
    }
}

#[test]
fn test_trim_end_inplace() {
    let mut s = String::from("test string \n\n");
    s.trim_end_inplace();

    assert_eq!(s, "test string");
}
