pub trait Bar {
    async fn set_length(&mut self, length: u64);
    async fn update_progress(&mut self, delta: u64);
    async fn set_progress(&mut self, progress: u64);
    async fn finish(&mut self);
}
