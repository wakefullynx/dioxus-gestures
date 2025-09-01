pub trait PointerEventReceiver<E> {
    fn pointer_over(&mut self, event: E);

    fn pointer_enter(&mut self, event: E);

    fn pointer_down(&mut self, event: E);

    fn pointer_move(&mut self, event: E);

    fn pointer_up(&mut self, event: E);

    fn pointer_cancel(&mut self, event: E);

    fn pointer_out(&mut self, event: E);

    fn pointer_leave(&mut self, event: E);
}
