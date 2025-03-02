use crate::{CDRomError, CDRomTrait};

pub struct CDRomWindows {}

impl CDRomWindows {
    pub fn new() -> Result<Self, CDRomError> {
        Ok(Self {  })
    }
}

impl CDRomTrait for CDRomWindows {

}