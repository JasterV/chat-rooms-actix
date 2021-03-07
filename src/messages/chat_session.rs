use actix::Message as ActixMessage;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);
