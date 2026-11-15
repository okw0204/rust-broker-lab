use rust_broker_lab::Broker;

#[test]
fn messages_are_received_once_in_publication_order() {
    let mut broker = Broker::new();

    assert_eq!(broker.receive(), None);

    let first = String::from("注文を受け付けた");
    let second = String::from("在庫を確保した");
    broker.publish(first);
    broker.publish(second);

    let received = broker.receive().expect("最初のメッセージがある");
    assert_eq!(received, "注文を受け付けた");

    broker.publish(String::from("発送を依頼した"));
    assert_eq!(broker.receive(), Some(String::from("在庫を確保した")));
    assert_eq!(broker.receive(), Some(String::from("発送を依頼した")));
    assert_eq!(broker.receive(), None);

    broker.publish(String::from("配達が完了した"));
    assert_eq!(broker.receive(), Some(String::from("配達が完了した")));
    assert_eq!(broker.receive(), None);
}
