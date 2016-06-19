use EncSysContainer;
use enc::*;

#[test]
fn add_then_check_word() {
	let mut enc = Encyclopedia::new();

	let word = Word::from_tag_vec("word", vec![Tag::new_with_info("a", "x"),]);
	enc.add(word.clone());

	assert_eq!(Some(word), enc.get_by_id(0));
}

#[test]
fn add2_remove_last() {
	let mut enc = Encyclopedia::new();

	let word1 = Word::from_tag_vec("word1", vec![Tag::new_with_info("a", "x"),]);
	let word2 = Word::from_tag_vec("word2", vec![Tag::new_nullary("b"),]);

	enc.add(word1);
	enc.add(word2);
	enc.remove_by_id(1);

	assert_eq!(enc.get_count(), 1);
}

#[test]
fn add2_remove_first() {
	let mut enc = Encyclopedia::new();

	let word1 = Word::from_tag_vec("word1", vec![Tag::new_with_info("a", "x"),]);
	let word2 = Word::from_tag_vec("word2", vec![Tag::new_nullary("b"),]);

	enc.add(word1);
	enc.add(word2.clone());

	enc.remove_by_id(0);

	assert_eq!(enc.get_count(), 1);
	// the behaviour of vacant id slots is not required to stay the same
	//assert_eq!(enc.get_end_id(), 2);
	assert_eq!(Some(word2), enc.get_by_id(1));
}

#[test]
fn iterate_over_3() {
	let mut enc = Encyclopedia::new();

	let word1 = Word::from_tag_vec("word1", vec![Tag::new_with_info("a", "x"),]);
	let word2 = Word::from_tag_vec("word2", vec![Tag::new_nullary("b"),]);
	let word3 = Word::from_tag_vec("word3", vec![Tag::new_with_info("c", "y"),]);

	enc.add(word1.clone());
	enc.add(word2.clone());
	enc.add(word3.clone());

	let mut i = enc.iter();

	// the order of these assertions is the order of the id's.
	assert_eq!(Some(word1), i.next());
	assert_eq!(Some(word2), i.next());
	assert_eq!(Some(word3), i.next());
	assert_eq!(None, i.next());
}

#[test]
fn iterate_over_n() {
	let mut enc = Encyclopedia::<usize, usize, ()>::new();

	let words = 100;

	for w in 0..words {
		enc.add(Word::from_tag_vec(w, vec![Tag::new_nullary(w)]));
	}

	assert_eq!(words, enc.get_count());

	for (w, word_tags) in enc.iter().map(&Word::to_tag_vec).enumerate() {
		assert_eq!(word_tags.len(), 1);
		assert_eq!(word_tags[0], Tag::new_nullary(w));
	}
}

#[test]
fn test_tag_group() {
	let mut enc = Encyclopedia::new();

	let tag1 = Tag::new_with_info("a", "x");
	let tag2 = Tag::new_nullary("b");
	let tag3 = Tag::new_with_info("c", "y");

	// let's add our group of tags 1 and 2
	enc.add_tag_group("group", vec![tag1.clone(), tag2.clone()]);

	// let's create our word consisting of tag 3 and our group
	let word = Word::from_tag_vec("word", vec![tag3.clone(), Tag::new_nullary("group")]);

	// add the word
	enc.add(word);

	// the expected tag output
	let tags = vec![tag1, tag2, tag3];

	// check if we can found all three tags
	// the order of the vector is not preserved
	let mut found = 0;
	for w in enc.iter() {
		for i in tags.iter() {
			if w.has_tag(i.get_name().as_ref()) {
				assert_eq!(w.get_tag_data(i.get_name().as_ref()), i.get_data());
				found += 1;
			}
		}
	}
	assert_eq!(found, 3);
}

#[test]
fn test_name_get() {
	let mut enc = Encyclopedia::<&str, usize, usize>::new();
	let word1 = Word::new("word1");
	let word2 = Word::new("word2");
	let word3 = Word::new("word3");

	enc.add(word1.clone());
	enc.add(word2.clone());
	enc.add(word3.clone());

	assert_eq!(enc.get_count(), 3);

	assert_eq!(Some(word1.clone()), enc.get_by_name("word1"));
	assert_eq!(Some(word2.clone()), enc.get_by_name("word2"));
	assert_eq!(Some(word3.clone()), enc.get_by_name("word3"));
	assert_eq!(None, enc.get_by_name("none"));
	assert!(Some(word1.clone()) != enc.get_by_name("word3"));

	enc.remove_by_name("word2");

	assert_eq!(enc.get_count(), 2);
	assert_eq!(Some(word1), enc.get_by_name("word1"));
	assert_eq!(None, enc.get_by_name("word2"));
	assert_eq!(Some(word3), enc.get_by_name("word3"));
}
