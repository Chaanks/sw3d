use specs::World;



pub trait State<T> {
    /// Executed when the game state begins.
    fn on_start(&mut self, _data: &mut StateData<T>);

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_event(&mut self, _data: &mut StateData<T>);

    /// Executed on every frame immediately, as fast as the engine will allow (taking into account the frame rate limit),
    /// if this is the active state.
    fn update(&mut self, _data: &mut StateData<T>);
}

pub struct StateData<'a, T>
where
    T: 'a,
{
    /// Main `World`
    pub world: &'a mut World,
    /// User defined game data
    pub data: &'a mut T,
}

impl<'a, T> StateData<'a, T>
where
    T: 'a,
{
    /// Create a new state data
    pub fn new(world: &'a mut World, data: &'a mut T) -> Self {
        StateData { world, data }
    }
}