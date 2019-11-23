/// A trait for objects which are digital outputs.
///
/// Attention: The implementor needs to take care to abstract away hardware specifics like
///            (PullUp, PullDown) on allways should enable and off always should disable
///            the object logically.
pub trait Output {
    /// Turns on the output.
    fn on(&mut self);

    /// Disables the output.
    fn off(&mut self);

    /// Toggle the state of the Output.
    fn toggle(&mut self);
}

/// A trait for objects which are digital inputs.
///
/// Attention: The implementor needs to take care to abstract away hardware specifics like
///            (PullUp, PullDown) true always means on and false always means logically off.
pub trait Input {
    /// Read the state of the input.
    ///
    /// True (on), False (off).
    fn read(&self) -> bool;
}
