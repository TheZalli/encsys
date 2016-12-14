//! Contains the `Encyclopedia` struct and an iterator to it's words.
use std::collections::{HashSet, HashMap, hash_map};
use std::iter::Iterator;
use std::borrow::Cow;
use std::ops::Deref;

use enc::word::*;
use enc::ling::LingTag;
use enc::ling::grammeme::{GrammCategory, Grammeme};
use enc::ling::error::LingError;

type LingResult<T> = Result<T, LingError>;

/// A word manager that stores information about the tags associated with words.
#[allow(dead_code)] // TODO
pub struct Encyclopedia {
    /// An association from word names into their tags.
    word_map: HashMap<String, HashSet<LingTag>>,
    /// The common shared tag groups
    tag_groups: HashMap<String, HashSet<LingTag>>,
    // TODO: optimize the default value into a raw pointer.
    /// All of the available grammatical categories.
    /// Map from category names into their values.
    /// The first value in the tuple is the default value if any.
    gramm_cats: HashMap<String, (Option<String>, HashSet<String>)>,
}

impl Encyclopedia {
    /// Creates a new empty encyclopedia.
    pub fn new() -> Encyclopedia {
        Encyclopedia {
            word_map: HashMap::new(),
            tag_groups: HashMap::new(),
            gramm_cats: HashMap::new(),
        }
    }

    // WORDS

    /// Adds a new word to the encyclopedia's word map.
    pub fn add_word(&mut self, word: Word) {
        let entry = word.into_map_entry();
        self.word_map.insert(entry.0, entry.1);
    }

    /// Returns a word with the given name or `None` if no such word was found.
    pub fn get_word<'a, T>(&'a self, name: T) -> Option<Word<'a>>
        where T: 'a + AsRef<str> + Into<Cow<'a, str>>
    {
        match self.word_map.get(name.as_ref()) {
            Some(&ref set) => Some(Word::new_from_collection(name.into(), set)),
            None => None,
        }
    }

    /// Removes the word with the given name.
    pub fn remove_word<'a, U: 'a + AsRef<str>>(&'a mut self, name: U) {
        self.word_map.remove(name.as_ref());
    }

    /// Returns the amount of words stored.
    pub fn word_amount(&self) -> usize {
        self.word_map.len()
    }

    /// Returns an iterator to the words
    pub fn iter_words<'a>(&'a self) -> WordIter<'a> {
        WordIter { iter: self.word_map.iter() }
    }

    // TAG GROUPS
    // TODO

    // GRAMMATICAL CATEGORIES

    /// Adds a new grammatical category.
    pub fn add_gramm_cat(&mut self, categ: GrammCategory) {
        let entry = categ.into_map_entry();
        self.gramm_cats.insert(entry.0, entry.1);
    }

    /// Returns a stored category if found.
    pub fn get_gramm_cat<'a>(&'a self, name: &'a str) -> Option<GrammCategory> {
        let opt = self.gramm_cats.get(name);
        if let Some(&(ref def_opt, ref set)) = opt {
            Some(GrammCategory::new(name.clone(), def_opt.clone(), set.clone()))
        } else {
            None
        }
    }

    /// Returns the amount of stored grammatical categories.
    pub fn gramm_cat_amount(&self) -> usize {
        self.gramm_cats.len()
    }

    /// Returns an iterator to the grammatical categories.
    pub fn iter_gramm_cats<'a>(&'a self) -> GrammCatIter<'a> {
        GrammCatIter { iter: self.gramm_cats.iter() }
    }

    // GRAMMEMES

    /// Constructs grammemes.
    ///
    /// Checks if given parameters make up for a valid grammeme value and then constructs it.
    pub fn create_grammeme<'a>(&'a self,
                               categ_name: &'a str,
                               value: Option<&str>)
                               -> LingResult<Grammeme<'a>> {
        if let Some(cat) = self.get_gramm_cat(categ_name) {
            let val = match value {
                Some(v) => {
                    cat.get_value(v)
                        .ok_or(LingError::NoSuchGrammeme(v.to_owned()))
                }
                None => {
                    cat.get_default_value()
                        .ok_or(LingError::NoDefaultOrValue(categ_name.to_owned()))
                }
            };

            // Return from function if errors were found.
            let val = try!(val);

            Ok(Grammeme::new(cat.get_name(), val))
        } else {
            Err(LingError::NoSuchGrammCat(categ_name.to_owned()))
        }
    }

    // OTHER

    /// Tells if the encyclopedia has no words or grammatical categories.
    ///
    /// `enc.is_empty()` is equivalent to `enc.word_amount() == 0 && enc.gramm_cat_amount() == 0`.
    pub fn is_empty(&self) -> bool {
        self.word_map.is_empty() && self.gramm_cats.is_empty()
    }
}

/// An iterator that goes through all of the words in an encyclopedia.
pub struct WordIter<'a> {
    iter: hash_map::Iter<'a, String, HashSet<LingTag>>,
}

impl<'a> Iterator for WordIter<'a> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((name, tags)) => Some(Word::new_from_collection(name.deref(), tags)),
            None => None,
        }
    }
}

pub struct GrammCatIter<'a> {
    iter: hash_map::Iter<'a, String, (Option<String>, HashSet<String>)>,
}

impl<'a> Iterator for GrammCatIter<'a> {
    type Item = GrammCategory<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((name, tuple)) => {
                Some(GrammCategory::new(name as &str,
                                        tuple.0.clone(),
                                        tuple.1.iter().map(&AsRef::as_ref)))
            }
            None => None,
        }
    }
}
