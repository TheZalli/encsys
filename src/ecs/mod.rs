pub mod component;
pub mod entity;
pub mod entity_manager;

pub use self::component::Comp;
pub use self::entity::Entity;
pub use self::entity_manager::EntMan;

#[cfg(test)]
mod test;
