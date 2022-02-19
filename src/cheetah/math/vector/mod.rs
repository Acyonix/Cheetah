pub mod vector2;
pub mod vector3;

trait Vector {
    /// Perform a dot product operation on this `Vector` and another.
    fn dot(&self, other: &Self) -> f64;

    /// Calculate the magnitude (length) of this `Vector`.
    fn magnitude(&self) -> f64;

    /// Return a `Vector` of magnitude 1 pointing in the same direction as this.
    fn normalised(&self) -> Self;
}
