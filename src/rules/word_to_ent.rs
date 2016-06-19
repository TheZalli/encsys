#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

//use enc::Word;
//use enc::Tag;
//use ecs::Entity;
//use ecs::Comp;
use EncSysType;

pub struct W2ERule<Tag, CompName, CompData>
	where	Tag: EncSysType + Hash + Debug,

			CompName: EncSysType + Hash + Debug,
			CompData: EncSysType + Debug,
{
	tags:	HashSet<Rc<Tag>>,
	comps:	HashMap<Rc<CompName>, Rc<CompData>>,
}
