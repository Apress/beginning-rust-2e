/* It prints:
Opening port usb4:879
Sent to usb4:879 the message 'Message 1'
Opening port eth1:12000
Sent to eth1:12000 the message 'Message 2'
Closing port eth1:12000
Sent to usb4:879 the message 'Message 3'
Closing port usb4:879
*/
fn main() {
    struct CommunicationChannel {
        address: String,
        port: u16,
    }
    impl Drop for CommunicationChannel {
        fn drop(&mut self) {
            println!("Closing port {}:{}", self.address, self.port);
        }
    }
    impl CommunicationChannel {
        fn create(address: &str, port: u16) -> CommunicationChannel {
            println!("Opening port {}:{}", address, port);
            CommunicationChannel {
                address: address.to_string(),
                port: port,
            }
        }
        fn send(&self, msg: &str) {
            println!("Sent to {}:{} the message '{}'",
                self.address, self.port, msg);
        }
    }
    let channel_a = CommunicationChannel::create("usb4", 879);
    channel_a.send("Message 1");
    {
        let channel_b = CommunicationChannel::create("eth1", 12000);
        channel_b.send("Message 2");
    }
    channel_a.send("Message 3");
}
