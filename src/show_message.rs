use crossterm::cursor::MoveToColumn;
use rive::prelude::Member;

use crossterm::ExecutableCommand;
use crossterm::style::PrintStyledContent;
use crossterm::style::Color;
use crossterm::style::Stylize;
use crossterm::execute;

use std::io::Write;

use crate::name_of;

pub fn show_message(
    message: &rive::prelude::Message,
    user: Option<&rive::prelude::User>,
    member: Option<&Member>,
    has_mention: bool
) {
    let username = name_of::user(
        member, user
    );
    let mut stdout = std::io::stdout();
    
    stdout.execute(
        PrintStyledContent(
            format!(
                "<{username}{}",
                if has_mention { "" } else { "> " }
            )
                .with(Color::DarkBlue)
        )
    ).unwrap();
    
    if has_mention {
        stdout.execute(
    PrintStyledContent(
                "> "
                    .to_string()
                    .with(Color::Red)
            )
        ).unwrap();
    }
    
    stdout.flush().unwrap();
    
    match message
            .content
            .clone() {
        Some(content) => write!(stdout, "{content}").unwrap(),
        None => {
            stdout.execute(
        PrintStyledContent(
                    "[No content]"
                        .with(Color::DarkGrey)
                )
            ).unwrap();
    
            stdout.flush().unwrap();
        }
    }
    
    if let Some(attachments) = &message.attachments {
        for attachment in attachments {
            stdout.execute(
        PrintStyledContent(
                    format!("[{}] ", attachment.filename)
                        .with(Color::Yellow)
                )
            ).unwrap();
        
            stdout.flush().unwrap();
        }
    }
    println!();
    execute!(
        stdout,
        MoveToColumn(0)
    ).unwrap();
}