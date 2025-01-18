use spec::Vm::{saveCall, UniswapSend};

use crate::{Cheatcode, Cheatcodes, Result};

impl Cheatcode for saveCall {
    fn apply(&self, _state: &mut Cheatcodes) -> Result {
        let Self { data } = self;

        let UniswapSend { Addr1, Addr2 } = data;
        println!("address_indexed 1: {:#?}", Addr1);
        println!("address_indexed 2: {:#?}", Addr2);

        Ok(Default::default())
    }
}
