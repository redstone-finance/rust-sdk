use crate::network::error::Error;
use alloc::vec::Vec;

#[cfg(feature = "radix")]
use scrypto::prelude::*;

use crate::types::{Sanitized, VALUE_SIZE};

const CHAR_START: u8 = 48; // Start at ASCII b'0'
const CHAR_END: u8 = 90; // End at ASCII b'Z'
const CHARS_COUNT: usize = (CHAR_END - (CHAR_START - 1)) as usize;

/// Type describing feed ids.
/// We expect FeedId to be byte string like b"EUR"
/// converted to bytearray and padded with zeroes to the right.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "radix", derive(ScryptoSbor))]
pub struct FeedId(pub [u8; VALUE_SIZE]);

// trim zeros from both sides
fn trim_zeros(v: Vec<u8>) -> Vec<u8> {
    if v.is_empty() {
        return v;
    }
    let l_index = match v.iter().position(|&byte| byte != 0) {
        Some(position) => position,
        _ => return vec![], // vec of all zeroes
    };

    let r_index = match v.iter().rposition(|&byte| byte != 0) {
        Some(position) => position,
        _ => return vec![], // not possible but vec of all zeroes
    };

    v[l_index..=r_index].into()
}

impl From<Vec<u8>> for FeedId {
    fn from(value: Vec<u8>) -> Self {
        let value = trim_zeros(value);
        let value = value.sanitized();

        let mut buff = [0; VALUE_SIZE];
        buff[0..value.len()].copy_from_slice(&value);

        Self(buff)
    }
}

#[derive(Debug, Clone)]
struct Node {
    children: [usize; CHARS_COUNT],
    end: bool,
}

impl Default for Node {
    #[inline]
    fn default() -> Self {
        Self {
            children: [0usize; CHARS_COUNT],
            end: false,
        }
    }
}

/// The Trie data structure is a tree-like data structure used for storing a dynamic set of strings.
/// It is commonly used for efficient retrieval and storage of keys in a large dataset.
///
/// Trie stores unique FeedIds to ensure no repetition of the same feed in given datastructure.
#[derive(Debug, Clone)]
pub struct Trie {
    nodes: Vec<Node>,
}

impl Default for Trie {
    fn default() -> Self {
        let nodes = vec![Node::default()];
        Self { nodes }
    }
}

macro_rules! is_pattern {
    ($obj:expr, $($matcher:pat),*) => {
       match $obj {
           $($matcher => true),*,
            _ => false,
       }
   }
}

impl Trie {
    /// Stores the feed_id in Trie and indicates is the feed_id a newly inserted value.
    ///
    /// The purpose of this function is to indicate if given feed_id already exist in the Trie datastructure,
    /// which offers then to validate the uniqueenes of given feed_id.
    ///
    /// # Arguments
    ///
    /// * `feed_id` - A FeedId value.
    /// It is acceptable for FeedId to contain ASSCI numbers and uppercase letters.
    /// All characters from the list are allowed: [b'0', ..., b'9', b'A' ..., b'Z'].
    ///
    /// # Returns
    ///
    /// Returns a `Result<bool, Error>`, which indicates if given FeedId.
    /// If The Trie contains FeedId already 'true' is returned, otherwise function returns 'false'.
    pub fn store(&mut self, feed_id: &FeedId) -> Result<bool, Error> {
        let mut cur_node = 0;

        for c in feed_id.0.iter() {
            if *c == b'\0' {
                break;
            }
            if is_pattern!(*c, b':', b';', b'<', b'=', b'>', b'?', b'@')
                || *c < CHAR_START
                || *c > CHAR_END
            {
                return Err(Error::UnhandlableCharInFeedID(*c as char, feed_id.clone()));
            }

            let i = (c - CHAR_START) as usize;
            if self.nodes[cur_node].children[i] == 0 {
                self.nodes[cur_node].children[i] = self.nodes.len();
                self.nodes.push(Node::default());
            }

            cur_node = self.nodes[cur_node].children[i];
        }

        if self.nodes[cur_node].end == true {
            return Ok(false);
        }

        self.nodes[cur_node].end = true;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_store_only_unique() -> Result<(), Error> {
        // given
        let test_cases = vec![
            "ABC", "AB", "ABCD", "ABCDE", "B", "BCD", "BCDE", "C", "CD", "CDE", "D", "DE",
        ];

        // when
        let mut test_processor = Trie::default();

        for feed_id in test_cases.iter() {
            // then
            let result = test_processor.store(&feed_id.as_bytes().to_vec().into())?;
            assert!(result);
        }

        Ok(())
    }

    #[test]
    fn test_trie_attempt_to_store_repeated() -> Result<(), Error> {
        // given
        let test_cases = vec![
            "ABC", "AB", "ABCD", "ABCDE", "B", "BCD", "BCDE", "C", "CD", "CDE", "D", "DE",
        ];

        // when
        let mut test_processor = Trie::default();

        for feed_id in test_cases.iter() {
            // with
            let result = test_processor.store(&feed_id.as_bytes().to_vec().into())?;
            assert!(result);
        }

        for feed_id in test_cases.iter() {
            // then
            let result = test_processor.store(&feed_id.as_bytes().to_vec().into())?;
            assert!(!result);
        }

        Ok(())
    }

    #[test]
    fn test_trie_store_unhandlable_characters() {
        // given
        let test_cases = vec![
            ("AB%", '%'),
            ("aB", 'a'),
            ("ABcD", 'c'),
            ("A^", '^'),
            ("AA!", '!'),
            ("A@!", '@'),
            ("CC<C", '<'),
        ];

        // when
        let mut test_processor = Trie::default();

        for (feed_id, unhandlable) in test_cases.iter() {
            // then
            let result = test_processor.store(&feed_id.as_bytes().to_vec().into());
            assert_eq!(
                result,
                Err(Error::UnhandlableCharInFeedID(
                    *unhandlable,
                    feed_id.as_bytes().to_vec().into()
                ))
            );
        }
    }
}
