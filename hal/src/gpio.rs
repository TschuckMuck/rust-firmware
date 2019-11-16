pub trait Out {
    /// Set the GPIO pin high.
    fn on(&mut self);

    /// Set the GPIO pin low.
    fn off(&mut self);

    /// Toggle the state of the the GPIO pin.
    fn toggle(&mut self);
}

pub trait In {
    /// Get the current state of the input pin.
    fn read(&self) -> bool;
}
