use findtext_sheet::search;

const RESOURCE_FILEPATH_01: &str = "./tests/fixtures/file1.xlsx";
const DUMMY_FILEPATH_01: &str = "./tests/fixtures/_file0.xlsx";

#[test]
fn found_01() {
    const EXPECTED_FOUND_COUNT: usize = 3;

    let ret = search("hej", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        let len = value.len();
        assert_eq!(
            len, EXPECTED_FOUND_COUNT,
            "Expected count to be {}, got {}",
            EXPECTED_FOUND_COUNT, len
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn found_02() {
    const EXPECTED_FOUND_COUNT: usize = 2;

    let ret = search("hejhej", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        let len = value.len();
        assert_eq!(
            len, EXPECTED_FOUND_COUNT,
            "Expected count to be {}, got {}",
            EXPECTED_FOUND_COUNT, len
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn found_03() {
    const EXPECTED_FOUND_COUNT: usize = 1;

    let ret = search("hejX", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        let len = value.len();
        assert_eq!(
            len, EXPECTED_FOUND_COUNT,
            "Expected count to be {}, got {}",
            EXPECTED_FOUND_COUNT, len
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn missing_01() {
    const EXPECTED_FOUND_COUNT: usize = 0;

    let ret = search("heJ", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);
    if let Ok(value) = ret {
        let len = value.len();
        assert_eq!(
            len, EXPECTED_FOUND_COUNT,
            "Expected count to be {}, got {}",
            EXPECTED_FOUND_COUNT, len
        );
    } else {
        panic!("Unexpected Err value");
    }
}

#[test]
fn error_01() {
    let ret = search("hej", DUMMY_FILEPATH_01);

    assert!(ret.is_err(), "Expected Err, got {:?}", ret);
}
