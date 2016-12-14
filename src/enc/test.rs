use super::*;
use super::ling::LingTag;

#[test]
fn add_then_check_word() {
    let mut enc = Encyclopedia::new();

    let word = Word::new_from_collection("word".to_owned(),
                                         vec![LingTag::Custom("tag".to_owned())]);

    enc.add_word(word.clone());

    assert_eq!(Some(word), enc.get_word("word".to_owned()));
}

#[test]
fn add2_remove_one() {
    let mut enc = Encyclopedia::new();

    let word1 = Word::new_from_collection("word1".to_owned(),
                                          vec![LingTag::Custom("a".to_owned())]);

    let word2 = Word::new_from_collection("word2".to_owned(),
                                          vec![LingTag::Custom("b".to_owned())]);

    enc.add_word(word1);
    enc.add_word(word2);
    enc.remove_word("word2".to_string());

    assert_eq!(enc.word_amount(), 1);
}

#[test]
fn iterate_over_n() {
    let mut enc = Encyclopedia::new();

    let words = 100;

    for w in 0..words {
        enc.add_word(Word::new_from_collection(w.to_string(),
                                               vec![LingTag::Custom(10.to_string())]));
    }

    assert_eq!(words, enc.word_amount());

    for word in enc.iter_words() {
        assert_eq!(word.tag_amount(), 1);
    }
}

#[test]
fn test_get() {
    let mut enc = Encyclopedia::new();
    let word1 = Word::new("word1".to_string());
    let word2 = Word::new("word2".to_string());
    let word3 = Word::new("word3".to_string());

    enc.add_word(word1.clone());
    enc.add_word(word2.clone());
    enc.add_word(word3.clone());

    assert_eq!(enc.word_amount(), 3);

    assert_eq!(Some(word1.clone()), enc.get_word("word1".to_string()));
    assert_eq!(Some(word2.clone()), enc.get_word("word2".to_string()));
    assert_eq!(Some(word3.clone()), enc.get_word("word3".to_string()));
    assert_eq!(None, enc.get_word("none".to_string()));
    assert!(Some(word1.clone()) != enc.get_word("word3".to_string()));

    enc.remove_word("word2".to_string());

    assert_eq!(enc.word_amount(), 2);
    assert_eq!(Some(word1), enc.get_word("word1".to_string()));
    assert_eq!(None, enc.get_word("word2".to_string()));
    assert_eq!(Some(word3), enc.get_word("word3".to_string()));
}
