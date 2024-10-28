pub trait Observer {
    fn update(&self, message: &str);
}