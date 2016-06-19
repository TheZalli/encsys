#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;
use std::fmt::Debug;

//use enc::Word;
//use enc::Tag;
//use ecs::Entity;
//use ecs::Comp;
use EncSysType;

pub struct W2ERule<TagName, TagInfo, CompName, CompData>
	where	TagName: EncSysType + Hash + Debug,
			TagInfo: EncSysType + Debug,

			CompName: EncSysType + Hash + Debug,
			CompData: EncSysType + Debug,
{
	tags:	HashMap<Rc<TagName>, Option<TagInfo>>,
	comps:	HashMap<Rc<CompName>, Rc<CompData>>,
}
