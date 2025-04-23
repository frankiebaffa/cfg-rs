use crate::Config;

#[test]
fn test_1() {
    let contents = concat!(
        "// a comment\n",
        "text=Here is some text.\n",
        "number=1\n",
        "url=https://github.com\n",
    ).to_string();

    let cfg = Config::from_string(contents).unwrap();

    assert!(cfg.contains_key("text"));
    assert_eq!("Here is some text.", cfg.value("text").unwrap());

    assert!(cfg.contains_key("number"));
    assert_eq!("1", cfg.value("number").unwrap());

    assert!(cfg.contains_key("url"));
    assert_eq!("https://github.com", cfg.value("url").unwrap());
}

#[test]
fn test_2() {
    let path = "./resources/test.cfg";

    let cfg = Config::from_file(path).unwrap();

    assert!(cfg.contains_key("text"));
    assert_eq!("This is the value of the property text.", cfg.value("text").unwrap());

    assert!(cfg.contains_key("url"));
    assert_eq!("https://duckduckgo.com", cfg.value("url").unwrap());

    assert!(cfg.contains_key("description"));
    assert_eq!("Here is some more text defining another property.", cfg.value("description").unwrap());

    assert!(cfg.contains_key("list"));

    let list = cfg.values("list").unwrap();
    assert_eq!("One", list.get(0).unwrap());
    assert_eq!("Two", list.get(1).unwrap());
    assert_eq!("Three", list.get(2).unwrap());
    assert_eq!("Four", list.get(3).unwrap());
    assert_eq!("Five", list.get(4).unwrap());

    assert!(cfg.is_truthy("firstistrue"));
    assert!(cfg.is_truthy("secondistrue"));
    assert!(cfg.is_truthy("thirdistrue"));
    assert!(cfg.is_truthy("fourthistrue"));
    assert!(!cfg.is_truthy("fifthistrue"));
    assert!(!cfg.is_truthy("sixthistrue"));
    assert!(!cfg.is_truthy("seventhistrue"));
    assert!(!cfg.is_truthy("eighthistrue"));
    assert!(!cfg.is_truthy("ninethistrue"));
    assert!(!cfg.is_truthy("tenthistrue"));
}
