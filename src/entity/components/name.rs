use std::fmt::{Display, Error, Formatter};

use derive_more::From;
use evenio::prelude::Component;

#[derive(Component, Clone, From, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}
