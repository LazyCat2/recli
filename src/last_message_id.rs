use rive::prelude::Channel;
use rive::prelude::Channel::*;

pub fn last_message_id(channel: &Channel) -> &Option<String> {
	match channel {
		DirectMessage { last_message_id, .. } => last_message_id,
		Group { last_message_id, .. } => last_message_id,
		TextChannel { last_message_id, .. } => last_message_id,
		
		_=>&None
	}
}