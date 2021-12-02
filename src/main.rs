use std::{env, error::Error};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl Handler {
    pub async fn write_message(
        &self,
        ctx: &Context,
        msg: &Message,
        content: &str,
    ) -> Result<(), Box<dyn Error>> {
        msg.channel_id.say(&ctx.http, content).await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot && msg.content.to_lowercase().contains("league") {
            self.write_message(&ctx, &msg, "Did someone say league?")
                .await
                .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("DISCORD_TOKEN")?;
    let mut client = Client::builder(&token).event_handler(Handler).await?;

    client.start().await?;
    Ok(())
}
