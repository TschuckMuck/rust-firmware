pub trait Output {
    /// Set the GPIO pin high.
    fn set(&self);

    /// Set the GPIO pin low.
    fn clear(&self);

    /// Toggle the state of the the GPIO pin.
    fn toggle(&self);
}

pub trait Input {
    /// Get the current state of the input pin.
    fn read(&self) -> bool;
}


