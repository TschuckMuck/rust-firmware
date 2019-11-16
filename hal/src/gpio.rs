pub trait Output {
    /// Set the GPIO pin high.
    fn set(&mut self);

    /// Set the GPIO pin low.
    fn clear(&mut self);

    /// Toggle the state of the the GPIO pin.
    fn toggle(&mut self);
}

pub trait Input {
    /// Get the current state of the input pin.
    fn read(&self) -> bool;
}
