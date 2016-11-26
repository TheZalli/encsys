//use super::*;
use super::grammeme::*;

#[test]
/// A simple sanity test for grammatical category's methods.
fn test_gramm_category() {
	let vals = &["def", "val2", "val3", "val4"];
	let categ = GrammCategory::new(vals[0], Vec::from(&vals[1..]));

	assert_eq!(categ.get_default_value(), &Some(vals[0].to_owned()) );

	// check if the values match
	assert_eq!(
		categ.get_values(),

		// sorry for this ugly expression. it transforms the `vals` slice into a hash set
		&vals.iter()
			.map(|x| (*x).to_owned() )
			.collect()
	);

	for val in vals {
		assert!(categ.is_value_valid(val));
	}
}
