pub mod tag;
pub mod word_manager;

#[cfg(test)]
mod test {
	use word_manager::WordManager;
	use tag::*;
	#[test]
	fn add_then_check_word() {
		let mut wordman = WordManager::new();
		let dummytags = vec![
			Tag::new("a", None),
			Tag::new("b", Some("x")),
			Tag::new("c", Some("y")),
		];
		wordman.add_word(dummytags.clone());

		assert_eq!(dummytags, wordman.get_word_by_id(0));
	}
}
