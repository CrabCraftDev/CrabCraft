use std::fmt::{Display, Error, Formatter};

use evenio::prelude::Component;
use uuid::Uuid as ActualUuid;

#[derive(Component)]
pub struct Uuid(pub ActualUuid);

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

mod test {
    // Tests that the string represetation returned by uuid crate is the same as the one used in minecraft (with dashes)
    #[test]
    fn test_uuid_string_representation() {
        let uuid = uuid::uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        assert_eq!(uuid.to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8")
    }
}
