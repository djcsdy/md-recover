use os_display::Quotable;
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Eq, Clone, Debug)]
pub struct MdDeviceId {
    internal_index: u64,
    user_reference: Option<OsString>,
}

impl MdDeviceId {
    const NEXT_INDEX: AtomicU64 = AtomicU64::new(0);

    pub fn new<S: AsRef<OsStr>>(user_reference: Option<S>) -> Self {
        Self {
            internal_index: Self::next_index(),
            user_reference: user_reference.map(|reference| OsString::from(&reference)),
        }
    }

    fn next_index() -> u64 {
        let index = Self::NEXT_INDEX.fetch_add(1, Ordering::AcqRel);
        if index == u64::MAX {
            panic!("MdDeviceId index overflow");
        }
        index
    }
}

impl PartialEq for MdDeviceId {
    fn eq(&self, other: &Self) -> bool {
        self.internal_index == other.internal_index
    }
}

impl Hash for MdDeviceId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_index.hash(state)
    }
}

impl Display for MdDeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.user_reference {
            None => write!(f, "[anonymous]"),
            Some(reference) => write!(f, "{}", reference.maybe_quote()),
        }
    }
}
