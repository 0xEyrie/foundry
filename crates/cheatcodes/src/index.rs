use spec::Vm::saveCall;

use crate::{Cheatcode, Cheatcodes, Result};

impl Cheatcode for saveCall {
    fn apply(&self, _state: &mut Cheatcodes) -> Result {
        let Self { data } = self;

        println!("Save data: {data:#?}");

        Ok(Default::default())
    }
}
