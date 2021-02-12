use std::ops::Deref;

use futures::future::BoxFuture;

use crate::{
    system::{AsyncSystem, DynamicSystemData, System},
    world::World,
};

pub type ThreadRun = Box<dyn for<'a> Run<'a> + Send>;
pub type LocalRun = Box<dyn for<'a> Run<'a>>;

pub type ThreadRunAsync<E> = Box<dyn for<'a> RunAsync<'a, Error = E> + Send>;
pub type LocalRunAsync<E> = Box<dyn for<'a> RunAsync<'a, Error = E>>;

/// Trait for fetching data and running systems.
/// Automatically implemented for systems.
pub trait Run<'a> {
    /// Runs the system now.
    ///
    /// # Panics
    ///
    /// Panics if the system tries to fetch resources
    /// which are borrowed in an incompatible way already
    /// (tries to read from a resource which is already written to or
    /// tries to write to a resource which is read from).
    fn run(&mut self, world: &'a World);
}

impl<'a, T> Run<'a> for T
where
    T: System<'a>,
{
    fn run(&mut self, world: &'a World) {
        let data = T::SystemData::fetch(self.accessor().deref(), world);

        self.run(data)
    }
}

/// Trait for fetching data and running systems with async/await.
/// Automatically implemented for systems.
pub trait RunAsync<'a> {
    /// Error returned when [`run_async`][AsyncSystem::run_async] fails.
    type Error: std::fmt::Debug;
    /// Runs the system now.
    ///
    /// # Panics
    ///
    /// Panics if the system tries to fetch resources
    /// which are borrowed in an incompatible way already
    /// (tries to read from a resource which is already written to or
    /// tries to write to a resource which is read from).
    fn run(&mut self, world: &'a World) -> BoxFuture<'a, Result<(), Self::Error>>;
}

impl<'a, T> RunAsync<'a> for T
where
    T: AsyncSystem<'a>,
{
    type Error = <T as AsyncSystem<'a>>::Error;

    fn run(
        &mut self,
        world: &'a World,
    ) -> BoxFuture<'a, Result<(), <T as AsyncSystem<'a>>::Error>> {
        let data = T::SystemData::fetch(self.accessor().deref(), world);

        self.run_async(data)
    }
}
