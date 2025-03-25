mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::use_keyword::front_of_house::hosting;

use std::collections::HashMap;

// using paths with same names
use std::fmt;
use std::io;

// as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// re-export names
mod front_of_house_reexport {
    pub mod hosting_reexport {
        pub fn add_to_waitlist() {}
    }
}
pub use front_of_house_reexport::hosting_reexport;

// use external packages
use rand::Rng;

// use nested paths
/*
    use std::cmp::Ordering;
    use std::io;

    use std::{cmp::Ordering, io};

    use std::io
    use std::io::Write;

    use std::io::{self, Write};
*/

// the glob operator (importing everything from a module)
use std::collections::*;

pub fn use_keyword_intro() {
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    println!("{:?}", map);
    println!("---------------------------------------------");

    fn function1() -> fmt::Result {
        Ok(())
    }
    fn function2() -> io::Result<()> {
        Ok(())
    }

    fn function3() -> Result {
        Ok(())
    }
    fn function4() -> IoResult<()> {
        Ok(())
    }

    hosting_reexport::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("---------------------------------------------");
}
