use super::{
    persistance::write_local_mald_history,
    state::{add_mald, get_mald_count, get_mald_history},
};
use chrono::Utc;
use serenity::{
    client::Context,
    model::{channel::Message, prelude::User},
    utils::MessageBuilder,
};

pub(crate) struct MaldManager;
impl MaldManager {
    pub fn new_mald(ctx: &Context, msg: &Message, user: &User) {
        let date = Utc::now().format("%d/%m/%Y").to_string();
        match add_mald(&ctx, &date, user.id) {
            Ok(_) => {},
            Err(e) => panic!(e)
        }

        let curr_malds = get_mald_count(&ctx, &date, user.id);

        let mut message = MessageBuilder::new();

        message.mention(user);

        match curr_malds {
            1 => message.push(format!(" has malded only once!")),
            _ => message.push(format!(" has malded `{}` times!", curr_malds)),
        };

        let _ = write_local_mald_history(&ctx);

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }

    pub fn mald_history(ctx: &Context, msg: &Message, user: &User) {
        let mut message = MessageBuilder::new();

        message.mention(user);
        message.push(" recent mald history:\n");

        let mald_history = get_mald_history(&ctx, user.id);
        
        match mald_history {
            Some(h) => {
                h.iter().for_each(|hist| {
                    let mald_formatted = format!("{} - {} mald(s)", hist.0, hist.1);
                    message.push_bold_line(mald_formatted);
                });
            },
            None => {
                message.push_bold_line(format!("{} is mald free!", user.name));
            }
        }

        if let Err(why) = msg.channel_id.say(&ctx.http, message.build()) {
            println!("Error sending message: {:?}", why);
        }
    }
}
