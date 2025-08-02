/// Trait converting snapshots into commands for a [`Bot`].
pub trait DecisionMaker<Snap, Command>: Send + 'static {
    /// Produce a command for the provided snapshot.
    fn decide(&mut self, snapshot: Snap) -> Command;
}
