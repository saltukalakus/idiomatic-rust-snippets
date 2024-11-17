pub trait Strategy {
    fn execute(&self, data: &str);
}