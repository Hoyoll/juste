pub trait Renderer {
    fn first_pass(&mut self);
    fn seconf_pass(&mut self);
}
