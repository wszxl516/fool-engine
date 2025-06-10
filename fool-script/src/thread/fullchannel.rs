use crossbeam_channel::{Receiver, Sender, bounded};
#[derive(Debug)]
pub struct FullChannel<C, R> {
    sender: Sender<C>,
    receiver: Receiver<R>,
}

impl<C, R> FullChannel<C, R> {
    pub fn new(max_capacity: usize) -> (FullChannel<C, R>, FullChannel<R, C>) {
        let (sender1, receiver1) = bounded::<C>(max_capacity);
        let (sender, receiver) = bounded::<R>(max_capacity);
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
    pub fn receiver(&mut self) -> &mut Receiver<R> {
        &mut self.receiver
    }
    pub fn sender(&self) -> &Sender<C> {
        &self.sender
    }
}
