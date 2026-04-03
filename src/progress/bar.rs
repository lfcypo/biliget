pub trait Bar {
    async fn update(&mut self, delta: u64);
    async fn finish(&mut self);

    async fn set_length(&mut self, length: u64);
}
