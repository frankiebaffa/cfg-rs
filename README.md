# cfg-rs

A simple crate to parse config files into their component key-value pairs.

```cfg
foo=bar
baz=biz
lorem=ipsum

// meant to be blank
blank=

list=1
list=2
list=3
list=4
list=5

this= that
```

The configuration file format is straight forward. An empty line is skipped. A
line beginning with 2 forward slashes (`//`) is skipped. A line containing
anything else is parsed as a key-value pair split on the first equals sign
(`=`). A key is a string, while a value is a vector of strings. The "_value_"
of a key is always the last value set, but all values are tracked and can be
retrieved as a vector if need be.

```rust
use cfg_rs::Config;

/// The aforementioned configuration.
let cfg = Config::from_file("./config.cfg").unwrap();

assert_eq!("bar", cfg.value("foo").unwrap());
assert_eq!("", cfg.value("blank").unwrap());
assert_eq!(" that", cfg.value("this").unwrap());
assert_eq!("5", cfg.value("list").unwrap());
assert_eq!("4", cfg.values("list").unwrap().get(3).unwrap());
```
