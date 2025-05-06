use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// An [UdaMember] is a participant imported from UDA.
/// It has a few fields, which can help to manage this member - confirm them, email them, ...
#[derive(Debug, Getters, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct UdaMember {
    id: u16,
    membership_number: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
    club: Option<String>,
    confirmed: bool,
}

impl UdaMember {
    pub fn new(
        id: u16,
        membership_number: Option<String>,
        first_name: String,
        last_name: String,
        email: String,
        club: Option<String>,
        confirmed: bool,
    ) -> Self {
        Self {
            id,
            membership_number,
            first_name,
            last_name,
            email,
            club,
            confirmed,
        }
    }
}

impl PartialOrd for UdaMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UdaMember {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.last_name() != other.last_name() {
            self.last_name().cmp(other.last_name())
        } else if self.first_name() != other.first_name() {
            self.first_name().cmp(other.first_name())
        } else {
            self.membership_number().cmp(&other.membership_number())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uda_member::UdaMember;

    fn get_id() -> u16 {
        42
    }
    fn get_membership_number() -> Option<String> {
        Some("0123456789".to_owned())
    }
    fn get_first_name() -> String {
        "Jon".to_owned()
    }
    fn get_last_name() -> String {
        "Snow".to_owned()
    }
    fn get_email() -> String {
        "jon.snow@email.com".to_owned()
    }
    fn get_club() -> Option<String> {
        Some("My club".to_owned())
    }
    fn get_confirmed() -> bool {
        true
    }

    fn get_uda_member() -> UdaMember {
        UdaMember::new(
            get_id(),
            get_membership_number(),
            get_first_name(),
            get_last_name(),
            get_email(),
            get_club(),
            get_confirmed(),
        )
    }
}
