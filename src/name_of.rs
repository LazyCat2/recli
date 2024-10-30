use rive::prelude::{Member, User};
use rive::prelude::Channel;

pub fn user(
    member: Option<&Member>,
    user: Option<&User>
) -> String {
    if member.is_some_and(|m| m.nickname.is_some()) {
        return member.unwrap().clone().nickname.unwrap()
    }
    
    if let Some(user) = user {
        return user.display_name.as_ref().unwrap_or(&user.username).to_string();
    }
    
    "".to_string()
}

pub fn channel(
    channel: &Channel,
    user_id: &String
) -> String {
    match channel {
        Channel::SavedMessages { .. } => "Notes".to_string(),
        Channel::DirectMessage { recipients, .. }
            => format!(
                "DM with {}", 
                recipients.iter().find(|id| *id != user_id).unwrap_or(&"".to_string()).clone()
            ),
        Channel::Group { name, ..} => name.clone().to_string(),
        Channel::TextChannel { name, .. } => name.clone().to_string(),
        Channel::VoiceChannel { name, ..} => name.clone().to_string()
    }
}