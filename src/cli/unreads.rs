use rive::{Rive, models::data::FetchMessagesData};
use rive::prelude::BulkMessageResponse;
use rive::prelude::MessageSort;

use std::io::{Write, stdout};

use crossterm::ExecutableCommand;
use crossterm::style::{Color, Stylize, PrintStyledContent};

use crate::name_of;
use crate::auth::auth;
use crate::last_message_id::last_message_id;
use crate::server_of::server_of;
use crate::show_message::show_message;

pub async fn main(
    with_id: bool,
    mentions_only: bool,
    server_id: Option<String>
) {
    let show_id = |id: &String| {
        if !with_id { return; }
        
        stdout().execute(
            PrintStyledContent(
                format!("{id} ")
                    .with(Color::DarkGrey)
            )
        ).unwrap();
        stdout().flush().unwrap();
    };
    
    let rive = Rive::new(auth()).await.unwrap();
    
    let unreads = rive.http.fetch_unreads().await.unwrap();
    
    let needed_channels: Option<Vec<String>> = 
        if let Some(ref server) = server_id {
            Some(
                rive.http
                    .fetch_server(server)
                    .await.expect("Probably invalid server ID")
                    .channels
            )
        } else { None };
    
    for unread in unreads {
        if mentions_only {
            match unread.mentions.clone() {
                Some(mentions) if mentions.is_empty() => continue,
                None => continue,
                _=>{}
            }
        }
        
        if needed_channels.clone().is_some_and(|c|!c.contains(&unread.id.channel)) {
            continue;
        }
        
        let channel = rive.http.fetch_channel(unread.id.channel.clone()).await;
        if channel.is_err() { continue; }
        let channel = channel.unwrap();
        if let Some(server) = server_id.clone() {
            match server_of(&channel) {
                Some(channel_server) if *channel_server != server => continue,
                
                None => continue,
                
                _=>{}
            }
        }
        
        if unread.last_id == *last_message_id(&channel) { continue; }
        
        
        match rive.http.fetch_messages(
            &unread.id.channel, 
            FetchMessagesData {
                    limit: Some(10),
                    after: unread.last_id.clone(),
                    include_users: Some(true),
                    sort: Some(MessageSort::Oldest),
                    
                    ..Default::default()
                }).await {
            Ok(
                BulkMessageResponse::MessagesAndUsers {
                    messages, 
                    users, 
                    members 
                }
            ) => {
                let members = members.unwrap_or_default();
                if messages.is_empty() { continue; }
    
                show_id(&unread.id.channel);
                
                stdout().execute(
                    PrintStyledContent(
                        format!("#{} ", name_of::channel(&channel, &unread.id.user))
                            .with(Color::Blue)
                    )
                ).unwrap();
    
                stdout().flush().unwrap();
                
                println!("{}",
                    if unread.mentions.clone().is_some_and(|m|!m.is_empty()) {
                        format!("\n{} mentions", unread.mentions.unwrap().len())
                    } else { "".to_string() }
                );
                
                for message in messages {
                    show_id(&message.id);
                    show_message(
                        &message, 
                        users.iter().find(|m| m.id == message.author),
                        members.iter().find(|m| m.id.server == message.author),
                        message.mentions.clone().is_some_and(|m|m.contains(&unread.id.user))
                    );
                }
            }
            
            Err(error) => println!("{error}"),
            _=>{}
        }
        println!();
    }
}
