extern crate specs;

use super::*;
use enc::*;
use enc::ling::LingTag;

type ESMan = EncSysWorld<String>;

#[derive(Debug, PartialEq, Eq)]
struct IntComp(i32);

impl specs::Component for IntComp {
    type Storage = specs::VecStorage<Self>;
}

#[test]
fn ent_from_word() {
    let mut man: ESMan = ESMan::new();
    // the word
    let word = Word::new_from_collection("word1", vec![LingTag::Custom("tag1".to_owned())]);

    // register a component
    man.ecs.register_w_comp_id::<IntComp>("comp".to_owned());

    // the word to entity rule function
    let foo = |word: Word, builder: &mut EncEntityBuilder<String>| {
        if word.has_tag(&LingTag::Custom("tag1".to_owned())) {
            builder.add_comp::<IntComp>("comp".to_owned(), IntComp(1));
        }
    };

    let ent = man.entity_from_word(word, &foo);

    assert_eq!(man.ecs.read_w_comp_id::<IntComp>("comp".to_owned()).get(ent),
               Some(&IntComp(1)));
}
