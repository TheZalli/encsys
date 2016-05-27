pub type TagName = String;
pub type TagInfo = Option<String>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tag {
	name: TagName,
	info: TagInfo
}

impl Tag {
	pub fn new(name: &str, info: Option<&str>) -> Tag {
		Tag{ name: name.to_owned(), info: info.map(str::to_owned) }
	}

	pub fn reconstruct(name: &TagName, info: &TagInfo) -> Tag {
		Tag{ name: name.to_owned(), info: info.clone() }
	}


	pub fn get_name<'a>(&'a self) -> &'a TagName {
		&self.name
	}

	pub fn get_info<'a>(&'a self) -> &'a TagInfo {
		&self.info
	}

	pub fn to_tuple(self) -> (TagName, TagInfo) {
		(self.name, self.info)
	}
}
