use serenity::{all::{Context, EventHandler, GatewayIntents, Message}, async_trait, Client};
use serde::{Deserialize, Serialize};

mod modules;

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    command_identifier: String
}

struct Handler {command_identifier : String}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(&self.command_identifier) {
            return;
        }

        let command : String = msg.content.clone().split_off(self.command_identifier.len()).split(" ").next().unwrap().to_string();
        
        //idk why but I need to make a variable for msg clone first before splitting it
        let msg_clone = msg.content.clone();
        let arguments : Vec<&str> = msg_clone.split(' ').collect();

        println!("Command received: {}, from user: {}", &command, &msg.author.name);

        if &command == "ping" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Something wen't wrong trying to send a message: {e:?}")
            }
        }

        if &command == "randomcase" {
            let input : String = msg.content.clone().split_off(self.command_identifier.len() + command.len());

            if input.len() == 0 {
                if let Err(e) = msg.channel_id.say(&ctx.http, "No arguments received.").await {
                    println!("Something wen't wrong trying to send a message: {e:?}");
                }
                return;
            }
            
            if let Err(e) = msg.channel_id.say(&ctx.http, modules::functions::randomcase(input)).await {
                println!("Something wen't wrong trying to send a message: {e:?}");
            }
        }

        if &command == "add" {
            let mut out : i128 = 0;
            for arg in arguments {
                match arg.parse::<i128>() {
                    Ok(num) => {out += num;}
                    Err(_) => {}
                }
            }

            if let Err(e) = msg.channel_id.say(&ctx.http, format!("``{out}``")).await {
                println!("Something wen't wrong trying to send a message: {e:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config : Config;
    match modules::load::load_config() {
        Ok(c) => {
            println!("Config loaded.");
            config = c;
        }
        Err(e) => {
            println!("Error loading config: {e:?}");
            return;
        }
    }

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&config.token, intents).event_handler(Handler {command_identifier: config.command_identifier.clone()}).await.expect("Err creating client.");

    if let Err(e) = client.start().await {
        println!("Error starting client: {e:?}");
    }
}