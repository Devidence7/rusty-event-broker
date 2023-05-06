pub mod in_memory {
    use crate::{EntryTransport, ExitTransport};

    struct InMemoryTransport {
        //
    }

    impl ExitTransport for InMemoryTransport {
        fn can_handle_request(&self, message: std::rc::Rc<dyn crate::Request>) -> bool {
            todo!()
        }

        fn request(
            &mut self,
            message: std::rc::Rc<dyn crate::Request>,
        ) -> Option<std::rc::Rc<dyn crate::Response>> {
            todo!()
        }
    }

    impl EntryTransport for InMemoryTransport {
        fn listen(&self) {
            todo!()
        }

        fn register_request_handler(&mut self, handler: &dyn crate::RequestHandler) {
            todo!()
        }
    }
}
