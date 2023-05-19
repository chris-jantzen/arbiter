/// Used to communicate with a UCI-compliant chess engine.
pub struct Engine;

impl Engine {
    /// Initialize a new chess engine given the path
    /// to the engine executable.
    pub fn new(path: &str) -> Self {
        todo!()
    }

    /// Sets option [name] to [value].
    pub fn set_option(&self, name: &str, value: &str) {
        todo!()
    }

    /// Tells the engine to start a new game.
    pub fn new_game(&self) {
        todo!()
    }

    /// Sets the current position from which to search.
    pub fn set_position(&self, fen: &str) {
        todo!()
    }

    /// Start searching from the current position.
    pub fn go<T>(&self, listener: T)
    where
        T: Fn() -> (),
    {
        todo!()
    }

    /// Tells the engine to stop searching.
    pub fn stop(&self) {
        todo!()
    }

    /// Tell the engine that the expected move was played.
    pub fn ponder_hit(&self) {
        todo!()
    }

    /// Terminates the engine.
    pub fn quit(&self) {
        todo!()
    }
}
