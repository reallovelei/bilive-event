use crate::cmd::Cmd;

#[test]
fn super_chat_test() {
    let json = include_str!("./mock/cmd/SuperChatMessage.json");
    let json_val = serde_json::from_str(json).unwrap();
    let cmd = Cmd::deser(json_val).unwrap();
    dbg!(cmd);
}

#[test]
fn send_gift_test() {
    let json = include_str!("./mock/cmd/SendGift.json");
    let json_val = serde_json::from_str(json).unwrap();
    let cmd = Cmd::deser(json_val).unwrap();
    dbg!(cmd);
}

#[test]
fn follow_test() {
    let json = include_str!("./mock/cmd/InteractWord.json");
    let json_val = serde_json::from_str(json).unwrap();
    let cmd = Cmd::deser(json_val).unwrap();
    let data = cmd.as_event();
    
    // println!("data:{:?}", data);

    // match crate::cmd::Cmd::deser(json_val) {
        
    //     Err(e) => return Err(EventParseError::CmdDeserError(e))
    // }
    dbg!(data);
}