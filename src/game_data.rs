use specs::{Dispatcher, World, System, DispatcherBuilder};
use bundle::SystemBundle;


/// Initialise trait for game data
pub trait DataInit<T> {
    /// Build game data
    fn build(self, world: &mut World) -> T;
}

pub struct GameData<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameData<'a, 'b> {
    /// Create new game data
    pub fn new(dispatcher: Dispatcher<'a, 'b>) -> Self {
        GameData { dispatcher }
    }

    /// Update game data
    pub fn update(&mut self, world: &World) {
        self.dispatcher.dispatch(&world.res);
    }
}

/// Builder for default game data
pub struct GameDataBuilder<'a, 'b> {
    pub disp_builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for GameDataBuilder<'a, 'b> {
    fn default() -> Self {
        GameDataBuilder::new()
    }
}

impl<'a, 'b> GameDataBuilder<'a, 'b> {
    /// Create new builder
    pub fn new() -> Self {
        GameDataBuilder {
            disp_builder: DispatcherBuilder::new(),
        }
    }

    pub fn with<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.disp_builder.add(system, name, dependencies);
        self
    }

    pub fn with_bundle<B>(mut self, bundle: B) -> Result<Self, ()>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.disp_builder);
        Ok(self)
    }
}

impl<'a, 'b> DataInit<GameData<'a, 'b>> for GameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> GameData<'a, 'b> {
        let mut dispatcher = self.disp_builder.build();
        dispatcher.setup(&mut world.res);
        GameData::new(dispatcher)
    }
}

impl DataInit<()> for () {
    fn build(self, _: &mut World) -> () {
        ()
    }
}