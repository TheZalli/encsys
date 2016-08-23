use enc::*;

#[test]
fn add_then_check_word() {
	let mut enc = Encyclopedia::new();

	let word = Word::from_collection("word", vec!["tag"]);
	enc.add(word.clone());

	assert_eq!(Some(word), enc.get("word"));
}

#[test]
fn add2_remove_one() {
	let mut enc = Encyclopedia::new();

	let word1 = Word::from_collection("word1", vec!["a"]);
	let word2 = Word::from_collection("word2", vec!["b"]);

	enc.add(word1);
	enc.add(word2);
	enc.remove("word2");

	assert_eq!(enc.amount(), 1);
}

#[test]
fn iterate_over_n() {
	let mut enc = Encyclopedia::<usize, usize>::new();

	let words = 100;

	for w in 0..words {
		enc.add(Word::from_collection(w, vec![10]));
	}

	assert_eq!(words, enc.amount());

	for word in enc.iter() {
		assert_eq!(word.tag_amount(), 1);
	}
}

#[test]
fn test_get() {
	let mut enc = Encyclopedia::<&str, ()>::new();
	let word1 = Word::new_empty("word1");
	let word2 = Word::new_empty("word2");
	let word3 = Word::new_empty("word3");

	enc.add(word1.clone());
	enc.add(word2.clone());
	enc.add(word3.clone());

	assert_eq!(enc.amount(), 3);

	assert_eq!(Some(word1.clone()), enc.get("word1"));
	assert_eq!(Some(word2.clone()), enc.get("word2"));
	assert_eq!(Some(word3.clone()), enc.get("word3"));
	assert_eq!(None, enc.get("none"));
	assert!(Some(word1.clone()) != enc.get("word3"));

	enc.remove("word2");

	assert_eq!(enc.amount(), 2);
	assert_eq!(Some(word1), enc.get("word1"));
	assert_eq!(None, enc.get("word2"));
	assert_eq!(Some(word3), enc.get("word3"));
}
