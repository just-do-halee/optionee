// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
// Copyright 2021 Hwakyeom Kim(=just-do-halee)

//! # `Optionee`
//!
//! The macro to creates option structure so easily.<br>
//! (no dependencies, 170 lines pure safe codes, supported no-std)  
//! ## How to
//! ```ignore
//! optionee! {
//!     InputOption {
//!         Id {
//!             min_length: u8 [>] 2, "id must be more than 3 bytes."
//!             max_length: u8 [<] 13, "id must be less than 12 bytes."
//!         }
//!         Password {
//!             encrypt: bool [=] true
//!             min_length: u8 [>] 5, "psasword must be more than 6 bytes."
//!             max_length: u8 [<] 20, "psasword must be less than 19 bytes."
//!         }
//!     }
//! }
//!
//! let mut id_t = InputOption.Id();
//! let user_input = 20;
//! assert!(id_t.min_length.check(user_input).is_ok());
//! ```
//!
//! ## More Examples
//! ```ignore
//! orderable! {
//!     pub struct Job {
//!         id: u32[*],
//!         name: String,
//!         salary: u16[*],
//!     }
//! }
//!
//! optionee! {
//!     pub TermOption {
//!             Password {
//!                 max_opportunity: u8 [=] 3, "if you don't really remember your own password, please consider to restart with --reset flag."
//!                 encrypt: bool [=] false
//!                 min_length: u8 [>] 7, "password must be more than 8 lengths bytes."
//!                 max_length: u8 [<] 21, "password must be less than 20 lengths bytes."
//!             }
//!         }
//!     SecondPrivateOpt {
//!             AnyName {
//!                 name: String [=] "john".to_string(), "you are not John."
//!                 job: Job [>] Job::new(0, "sales".to_string(), 29), "id must be more than 1, salary 30"
//!             }
//!             OldPerson {
//!                 name: &'static str [=] "mia"
//!                 age: u8 [>] 52
//!             }
//!         }
//! }
//!
//! let mut t1 = TermOption.Password().encrypt(true).min_length(3);
//! t1.min_length
//!     .set_error_message(Some("password must be more than 3 lengths bytes."));
//!
//! assert!(t1.encrypt.get_value());
//! assert!(t1.min_length.check(4).is_ok());
//!
//! let t2 = SecondPrivateOpt
//!     .AnyName()
//!     .job(Job::new(0, "writer".to_string(), 29));
//!
//! assert!(t2.name.check("john".to_string()).is_ok());
//! assert!(t2.job.check(Job::new(1, "artist".to_string(), 30)).is_ok());
//! ```

#![no_std]
extern crate alloc;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString};

    orderable! {
        pub struct Job {
            id: u32[*],
            name: String,
            salary: u16[*],
        }
    }

    optionee! {
        pub TermOption {
                Password {
                    max_opportunity: u8 [=] 3, "if you don't really remember your own password, please consider to restart with --reset flag."
                    encrypt: bool [=] false
                    min_length: u8 [>] 7, "password must be more than 8 lengths bytes."
                    max_length: u8 [<] 21, "password must be less than 20 lengths bytes."
                }
            }
        SecondPrivateOpt {
                AnyName {
                    name: String [=] "john".to_string(), "you are not John."
                    job: Job [>] Job::new(0, "sales".to_string(), 29), "id must be more than 1, salary 30"
                }
                OldPerson {
                    name: &'static str [=] "mia"
                    age: u8 [>] 52
                }
            }
    }
    #[test]
    fn it_works() {
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
    }
}
