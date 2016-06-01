use std::rc::Rc;

pub struct Comp<C, D> {
	pub name: Rc<C>,
	pub data: D,
}

impl<C, D> Comp<C, D> {
	pub fn new<T>(name: T, data: D) -> Comp<C, D>
		where T: Into<Rc<C>>
	{
		Comp{ name: name.into(), data: data }
	}

	pub fn get_name(&self) -> Rc<C> {
		self.name.clone()
	}

	pub fn get_data(&self) -> &D {
		&self.data
	}

}
