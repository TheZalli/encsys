use ecs::component::Comp;

pub struct Entity<C, D> {
	comps: Vec<Comp<C, D>>
}
