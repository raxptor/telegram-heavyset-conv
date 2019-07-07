extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate tokio_request;

use std::env;

use futures::{Future, Stream};
use telegram_bot::*;
use tokio_core::reactor::Core;
use tokio_request::str::get;

pub mod conv;
mod tests;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");    
    let api = Api::configure(token.clone()).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.spawn(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )));
            }            
            if let MessageKind::Document { ref data, .. } = message.kind {
                println!("File is {} ", data.file_id);                                                

                let request = api.send(types::requests::GetFile::new(data));
                let h2 = handle.clone();
                let h3 = handle.clone();             
                let chat = message.from.clone();   
                let a = api.clone();
                let future = request.and_then(move |r| { 
                    if let Some(path) = &r.file_path {                                                
                        let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
                        let url = format!("https://api.telegram.org/file/bot{}/{}", &token, &path);
                        println!("url is {:?}", url);
                        let future = get(&url).send(h2).and_then(move |uu| {
                            if let Some(text) = uu.body_str() {
                                if let Ok(apa) = conv::convert(text) {
                                    println!("Convert success");
                                    a.spawn(chat.text(apa));
                                }
                            }
                            Ok(())
                        });
                        h3.spawn(future.map_err(|_| ()));              
                    }   
                    Ok(())                     
                });
                handle.spawn(future.map_err(|_| ()));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
