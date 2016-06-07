use std::rc::Rc;

/// A component with a name and data.
pub struct Comp<C, D> {
	pub name: Rc<C>,
	pub data: Rc<D>,
}

impl<C, D> Comp<C, D> {
	/// Creates a new component.
	pub fn new<T, U>(name: T, data: U) -> Comp<C, D>
		where	T: Into<Rc<C>>,
				U: Into<Rc<D>>,
	{
		Comp{ name: name.into(), data: data.into() }
	}

	/*pub fn get_name(&self) -> Rc<C> {
		self.name.clone()
	}

	pub fn get_data(&self) -> Rc<D> {
		self.data.clone()
	}*/
}

impl<C, D> Into<(Rc<C>, Rc<D>)> for Comp<C, D> {
	fn into(self) -> (Rc<C>, Rc<D>) {
		(self.name, self.data)
	}
}

impl<C, D> From<(Rc<C>, Rc<D>)> for Comp<C, D> {
	fn from(t: (Rc<C>, Rc<D>)) -> Self {
		Comp::new(t.0, t.1)
	}
}
