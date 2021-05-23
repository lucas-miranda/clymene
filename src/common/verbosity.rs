
pub trait Verbosity {
    fn verbose(&mut self, verbose: bool);
    fn is_verbose(&self) -> bool;
}
