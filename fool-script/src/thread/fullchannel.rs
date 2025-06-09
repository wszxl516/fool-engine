use crossbeam_channel::{Receiver, Sender, bounded};
pub struct FullChannel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> FullChannel<T> {
    pub fn new(max_capacity: usize) -> (FullChannel<T>, FullChannel<T>) {
        let (sender1, receiver1) = bounded::<T>(max_capacity);
        let (sender, receiver) = bounded::<T>(max_capacity);
        (
            FullChannel {
                receiver,
                sender: sender1,
            },
            FullChannel {
                receiver: receiver1,
                sender,
            },
        )
    }
    pub fn receiver(&mut self) -> &mut Receiver<T> {
        &mut self.receiver
    }
    pub fn send(&self) -> Sender<T> {
        self.sender.clone()
    }
}
