use crossterm::execute;
use crossterm::cursor::MoveToColumn;
use crossterm::terminal;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode::Enter;
use crossterm::event::KeyCode::Char;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

use rive::Rive;
use rive::prelude::*;

use std::io::Write;
use tokio::time;

use crate::auth::auth;
use crate::show_message::show_message;


const PREFIX: &str = "[ReCLI]# ";

// Here be dragons
pub async fn main(selected_channel_id: String) -> Result<(), Box<dyn std::error::Error>> {
	let mut stdout = std::io::stdout();
	let mut text_input = "".to_string();
	
 	terminal::enable_raw_mode().unwrap();
	
	let mut rive = Rive::new(auth()).await?;
	
	let me= rive.http.fetch_self().await.unwrap();
	
	loop {
		tokio::select! {
			event = rive.gateway.next() => {
            		if let Some(event) = event {
                	let event = event?;
		
					rive.update(&event);
					
					match event {
						ServerEvent::Ready(_) => {
							print!("{PREFIX}");
							stdout.flush().unwrap();
						},
						
						ServerEvent::Message(message) => {
							if message.channel != selected_channel_id { continue; }
							
							let user = match rive.cache.user(&message.author) {
								Some(user) => Some(user.clone()),
								None => {
									match rive.http.fetch_user(&message.author).await {
										Ok(user) => Some(user),
								
										Err(_) => None
									}
								}
							};
							
							//TODO: Fetch member and show server-wide nickname
							//Curnety, this is the shittiest code in whole project, I guess
							/*
							let member: Option<Member> = if let Some(channel) = 
								if let Some(cnl_cache) = rive.cache.channel(message.channel.as_ref()) {
									Some(cnl_cache.clone())
								} else if let Ok(cnl_http) = rive.http.fetch_channel(&message.channel).await {
										Some(cnl_http)
									} else { None }
							{
								let server_id = server_of(&channel);
								
								if server_id.is_none() { () }
								
								let server_id = server_id.unwrap();
								
								let member_id = MemberCompositeKey {
									server: server_id.clone(),
									user: message.author.clone()
								};
								
								match rive.cache.member(&member_id) {
									Some(member) => Some(member.clone()),
									
									None => {
										match rive.http.fetch_member(
											member_id.server,
											member_id.user
										).await {
											Ok(member) => Some(member),
									
											Err(_) => None
										}
									}
								}
							} else { None };*/
							
							let member: Option<Member> = None;
						
								show_message(
								&message, 
								user.as_ref(), 
								member.as_ref(),
								message.mentions.clone().unwrap_or(vec![]).contains(&me.id)
							);
						},
						_=>{}
					}
            		} else {
        	        	// Handle the case where event is None
                	break; // or continue, depending on your logic
            		}
			},
            	_ = time::sleep(time::Duration::from_millis(100)) => {}
        	}
	
		if poll(std::time::Duration::from_millis(100)).unwrap() {
			match read().unwrap() {
				Event::Key(
					KeyEvent { 
						code: Char('c'), 
						modifiers: KeyModifiers::CONTROL,
						..
					}, ..
				) => break,
				
				Event::Key(KeyEvent { code: Enter, .. }) => {
					execute!(
						stdout,
						MoveToColumn(0)
					).unwrap();
					if let Err(error) 
					= rive.http.send_message(
						selected_channel_id.clone(), 
						SendMessageData {
							content: Some(text_input),
							
							..SendMessageData::default()
						}
					).await {
						println!("{error}")
					};
					execute!(
						stdout,
						MoveToColumn(0)
					).unwrap();
					print!("{PREFIX}");
					stdout.flush().unwrap();

					text_input = "".to_string();
				},
				
				Event::Key(KeyEvent {
					code: Char(char),
					modifiers,
					..
				}) if modifiers == KeyModifiers::NONE 
				|| modifiers == KeyModifiers::SHIFT => {
					print!("{char}");
					stdout.flush().unwrap();
					text_input = format!("{text_input}{char}");
				},
				
				_=>{}
			}	
		}
	}
	terminal::disable_raw_mode().unwrap();

	Ok(())
}
