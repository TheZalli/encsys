use encyclopedia::Encyclopedia;
use tag::*;

#[test]
fn add_then_check_word() {
	let mut enc = Encyclopedia::new();

	let word = vec![
		Tag::new("a", Some("x")),
	];

	enc.add_word(word.clone());

	assert_eq!(Some(word), enc.get_word_by_id(0));
}

#[test]
fn add_then_remove_word() {
	let mut enc = Encyclopedia::new();

	let word = vec![
		Tag::new("a", Some("x")),
	];

	enc.add_word(word);
	enc.remove_word_by_id(0);

	// since we removed the latest id, it's id slot should be freed.
	assert_eq!(enc.get_end_id(), 0);
}

#[test]
fn add2_remove1() {
	let mut enc = Encyclopedia::new();

	let word1 = vec![
		Tag::new("a", Some("x")),
	];
	let word2 = vec![
		Tag::new("b", None),
	];

	enc.add_word(word1);
	enc.add_word(word2.clone());

	enc.remove_word_by_id(0);

	assert_eq!(enc.get_word_count(), 1);
	// the behaviour of vacant id slots is not required to stay the same
	//assert_eq!(enc.get_end_id(), 2);
	assert_eq!(Some(word2), enc.get_word_by_id(1));
}
