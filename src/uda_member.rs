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
            self.membership_number().cmp(other.membership_number())
        }
    }
}

#[cfg(test)]
mod tests {
    mod ord {
        use crate::uda_member::UdaMember;

        #[test]
        fn equal() {
            let member_1 = UdaMember::new(1, None, "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);
            let member_2 = UdaMember::new(1, None, "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);

            assert_eq!(member_1, member_2);
        }


        #[test]
        fn greater_by_last_name() {
            let member_1 = UdaMember::new(1, None, "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);
            let member_2 = UdaMember::new(1, None, "John".to_string(), "Jacques".to_string(), "john.doe@email.com".to_string(), None, true);

            assert!(member_1 < member_2);
        }

        #[test]
        fn greater_by_first_name() {
            let member_1 = UdaMember::new(1, None, "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);
            let member_2 = UdaMember::new(1, None, "William".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);

            assert!(member_1 < member_2);
        }

        #[test]
        fn greater_by_membership_number() {
            let member_1 = UdaMember::new(1, Some("123456".to_string()), "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);
            let member_2 = UdaMember::new(1, Some("789123".to_string()), "John".to_string(), "Doe".to_string(), "john.doe@email.com".to_string(), None, true);

            assert!(member_1 < member_2);
        }
    }
}
