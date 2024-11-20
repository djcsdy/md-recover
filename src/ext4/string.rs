use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

#[derive(Eq, Ord, Clone, Hash, Debug)]
pub struct Ext4String<S: AsRef<[u8]>>(S);

impl<'s> Ext4String<&'s [u8]> {
    pub fn from_null_terminated_bytes(bytes: &'s [u8]) -> Self {
        Self(bytes.split(|c| *c == 0).next().unwrap_or_default())
    }
}

impl<S: AsRef<[u8]>> Ext4String<S> {
    pub fn to_owned(&self) -> Ext4String<Vec<u8>> {
        Ext4String(self.0.as_ref().to_owned())
    }

    pub fn to_str_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.0.as_ref())
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'s, S: AsRef<[u8]>> From<&'s Ext4String<S>> for &'s [u8] {
    fn from(value: &'s Ext4String<S>) -> Self {
        value.0.as_ref()
    }
}

impl<S: AsRef<[u8]>> TryFrom<&Ext4String<S>> for String {
    type Error = FromUtf8Error;

    fn try_from(value: &Ext4String<S>) -> Result<Self, Self::Error> {
        String::from_utf8(value.0.as_ref().to_owned())
    }
}

impl<'s> From<&'s String> for Ext4String<&'s [u8]> {
    fn from(value: &'s String) -> Self {
        Self(value.as_bytes())
    }
}

impl<S: AsRef<[u8]>> PartialEq<Self> for Ext4String<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

impl<S: AsRef<[u8]>> PartialOrd for Ext4String<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = self.to_str_lossy();
        let right = self.to_str_lossy();
        left.partial_cmp(&right)
    }
}

impl<S: AsRef<[u8]>> Display for Ext4String<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str_lossy())
    }
}
