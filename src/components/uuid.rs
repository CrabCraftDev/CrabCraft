use std::fmt::{Display, Error, Formatter};

use derive_more::From;
use evenio::prelude::Component;
use uuid::Uuid;

#[derive(Component, Clone, Copy, From, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct UuidComponent(pub Uuid);

impl Display for UuidComponent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

mod test {
    #[test]
    fn test_uuid_string_representation() {
        // Tests that the string represetation returned by uuid crate is the same as the one used in minecraft (with dashes)
        let uuid = uuid::uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(uuid.to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8")
    }
}
