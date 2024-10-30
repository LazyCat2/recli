use rive::prelude::Channel;
use rive::prelude::Channel::*;

pub fn server_of(channel: &Channel) -> Option<&String> {
	match channel {
		TextChannel { server, .. } => Some(server),
		VoiceChannel { server, .. } => Some(server),
		
		_=>None
	}
}