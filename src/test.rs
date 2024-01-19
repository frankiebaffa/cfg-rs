use crate::{ Cfg, FromCfg, };

#[test]
fn test_1() {
    let contents = concat!(
        "// a comment\n",
        "text = Here is some text.\n",
        "number = 1\n",
        "url = \"https://github.com\"\n",
    ).to_string();

    let cfg = Cfg::parse(contents).unwrap();

    assert!(cfg.get("text").is_some());
    assert_eq!("Here is some text.", cfg.get("text").unwrap());

    assert!(cfg.get("number").is_some());
    assert_eq!("1", cfg.get("number").unwrap());

    assert!(cfg.get("url").is_some());
    assert_eq!("https://github.com", cfg.get("url").unwrap());
}

#[test]
fn test_2() {
    let path = "./test.cfg";

    let cfg = Cfg::from_file(path).unwrap();

    assert!(cfg.get("text").is_some());
    assert_eq!("This is the value of the property text.", cfg.get("text").unwrap());

    assert!(cfg.get("url").is_some());
    assert_eq!("https://duckduckgo.com", cfg.get("url").unwrap());

    assert!(cfg.get("description").is_some());
    assert_eq!("Here is some more text defining another property.", cfg.get("description").unwrap());
}
