// The version of the game the replay was made for.
// PartialEq only matches the major version.
//
// E.g. 2.206  -> major: 22, minor: 06
//      2.2074 -> major: 22, minor: 074
//      2.113  -> major: 21, minor: 13
#[derive(Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
    }

    fn ne(&self, other: &Self) -> bool {
        self.major != other.major
    }
}
