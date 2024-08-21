use enum_handler::EnumHandler;

#[test]
fn test_mock() {
    #[derive(EnumHandler)]
    #[enum_handler(mock_name = "TestEvent", pass_args_by_ref = true)]
    enum Event {
        Hello(String),
    }

    let event = Event::Hello("world".to_string());
    let mut mock = MockTestEvent::new();
    mock.expect_on_hello()
        .times(1)
        .withf(|s| s == "world")
        .returning(|_| ());
    mock.on(&event);
}

#[tokio::test]
async fn test_async_mock() {
    #[derive(EnumHandler)]
    #[enum_handler(is_async = true, mock_name = "AsyncTestEvent", pass_args_by_ref = true)]
    enum AsyncEvent {
        Hello(String),
    }

    let event = AsyncEvent::Hello("world".to_string());
    let mut mock = MockAsyncTestEvent::new();
    mock.expect_on_hello()
        .times(1)
        .withf(|s| s == "world")
        .returning(|_| ());
    mock.on(&event)
        .await;
}
