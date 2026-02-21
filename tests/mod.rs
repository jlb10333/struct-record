use struct_record::record;

#[test]
fn test_record() {
    #[record(i32, TestEnumToI32, "#[derive(Clone)]")]
    enum TestEnum {
        Foo(i32),
        BarValue,
        BazValue,
    }

    let test = TestEnumToI32 {
        foo: 1,
        bar_value: 2,
        baz_value: 3,
    };

    test.clone();
}
