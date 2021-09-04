# optionee

The macro to create option struct easily.<br>
(no-std support)

[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[crates-badge]: https://img.shields.io/crates/v/optionee.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/optionee?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee

[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/optionee
[license-url]: https://github.com/just-do-halee/optionee
| [Docs](https://docs.rs/optionee) | [Latest Note](https://github.com/just-do-halee/optionee/blob/main/CHANGELOG.md) |

```toml
[dependencies]
optionee = "0.2.0"
```

or

```toml
[dependencies]
optionee = { version = "0.2.0", default-features = false } # no-std(alloc)
```

## How to
```rust
optionee! {

    InputOption {

        Id {
            min_length: u8 [>] 2, "id must be more than 3 bytes."
            max_length: u8 [<] 13, "id must be less than 12 bytes."
        }
        Password {
            encrypt: bool [=] true
            min_length: u8 [>] 5, "psasword must be more than 6 bytes."
            max_length: u8 [<] 20, "psasword must be less than 19 bytes."
        }

    }

}

let mut id_t = InputOption.Id();

let user_input = 20;

assert!(id_t.min_length.check(user_input).is_ok());
```
## More Examples
```rust
orderable! {

    pub struct Job {
        id: u32[*], // mark for comparing
        name: String,
        salary: u16[*], // ..
    }

}
optionee! {

    pub TermOption {
            Password {
                max_opportunity: u8 [=] 3, "if you don't really remember your own password, please cider o restart with --reset flag."
                encrypt: bool [=] false
                min_length: u8 [>] 7, "password must be more than 8 lengths bytes."
                max_length: u8 [<] 21, "password must be less than 20 lengths bytes."
            }
    }
    SecondPrivateOpt {
            AnyName {
                name: String [=] "john".to_string()
                job: Job [>] Job::new(0, "sales".to_string(), 29), "id must be more than 1, salary 30"
            }
            OldPerson {
                name: &'static str [=] "mia"
                age: u8 [>] 52
            }

    }

}

let mut t1 = TermOption.Password().encrypt(true).min_length(3);
t1.min_length
    .set_error_message(Some("password must be more than 3 lengths bytes."));

assert!(t1.encrypt.get_value());
assert!(t1.min_length.check(4).is_ok());

let t2 = SecondPrivateOpt
    .AnyName()
    .job(Job::new(0, "writer".to_string(), 29));

assert!(t2.name.check("john".to_string()).is_ok());
assert!(t2.job.check(Job::new(1, "artist".to_string(), 30)).is_ok());
```