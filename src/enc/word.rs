//! Contains the `Word` trait and an iterator for it's tags.
use std::collections::{HashSet, hash_set};
use std::iter::Iterator;
use std::borrow::Cow;

use enc::ling::LingTag;

/// A view to a word that has a name and associated tags.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Word<'a> {
    name: Cow<'a, str>,
    tags: HashSet<Cow<'a, LingTag>>,
}

/// A word that has a name and a set of tags.
impl<'a> Word<'a> {
    /// Creates a new empty word.
    pub fn new<T: 'a + Into<Cow<'a, str>>>(name: T) -> Self {
        Word {
            name: name.into(),
            tags: HashSet::new(),
        }
    }

    /// Creates a word from a name and a collection of words.
    pub fn new_from_collection<T, U, V>(name: T, coll: U) -> Self
        where T: 'a + Into<Cow<'a, str>>,
              U: IntoIterator<Item = V>,
              V: Into<Cow<'a, LingTag>>
    {
        Word {
            name: name.into(),
            tags: coll.into_iter().map(&Into::into).collect(),
        }
    }

    /// Returns the name of the word.
    pub fn get_name(&self) -> Cow<'a, str> {
        self.name.clone()
    }

    // Returns the tags of the word.
    pub fn get_tags(&self) -> &HashSet<Cow<'a, LingTag>> {
        &self.tags
    }

    /// Adds the given tag to the word, replacing any previous tag with the same value.
    pub fn add_tag(&mut self, tag: Cow<'a, LingTag>) {
        self.tags.insert(tag);
    }

    /// Returns true if the word has the given tag.
    pub fn has_tag(&self, tag: &'a LingTag) -> bool {
        self.tags.contains(tag)
    }

    /// Returns the amount of tags stored.
    pub fn tag_amount(&self) -> usize {
        self.tags.len()
    }

    /// Returns true if the word has no tags.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    /// Returns an iterator over the tags of the word.
    pub fn iter(&'a self) -> TagIter<'a> {
        TagIter { iter: self.tags.iter() }
    }

    /// Consumes self and returns a tuple that can be used to store this into a map structure.
    pub fn into_map_entry(self) -> (String, HashSet<LingTag>) {
        (self.name.into_owned(), self.tags.into_iter().map(&Cow::into_owned).collect())
    }
}

/// An iterator over the tags of a word.
pub struct TagIter<'a> {
    // Please, do not edit this variable outside this module.
    iter: hash_set::Iter<'a, Cow<'a, LingTag>>,
}
impl<'a> Iterator for TagIter<'a> {
    type Item = Cow<'a, LingTag>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.clone())
    }
}
