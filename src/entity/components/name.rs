use std::fmt::{Display, Error, Formatter};

use evenio::prelude::Component;

#[derive(Component)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}
