use discord_flows::{model::Message, Bot, ProvidedBot};
use flowsnet_platform_sdk::logger;
use phf::phf_map;
use rand::Rng;

const N: usize = 10;

const QUOTES: [&str;N] = ["Success is not final, failure is not fatal: It is the courage to continue that counts - Winston Churchill" 
,"We may encounter many defeats but we must not be defeated - Maya Angelou" 
,"Your time is limited, don't waste it living someone else's life - Steve Jobs" 
,"Don't let anyone tell you what you can't do. Follow your dreams and persist - Barack Obama" 
,"If you want something you've never had, you must be willing to do something you've never done - Unknown" 
,"You only live once, but if you do it right, once is enough - Mae West" 
,"If you want to live a happy life, tie it to a goal, not to people or things - Albert Einstein" 
,"The only Limit to our realization of tomorrow will be our doubts of today - Franklin D. Roosevelt" 
,"Don't let yesterday take up too much of today - Will Rogers" 
,"It is never too late to be what you might have been - George Eliot" 
];

const ANIMAL_FACTS: [&str; N] = [
"A rhinoceros' horn is made of hair.",
"A snail can sleep for three years.",
"The fingerprints of a koala are so indistinguishable from humans that they have on occasion been confused at a crime scene.",
"Elephants are the only animal that can't jump.",
"It takes a sloth two weeks to digest its food.",
"Bats always turn left when leaving a cave.",
"An ostrich's eye is bigger than its brain.",
"Around 50 percent of orangutans have fractured bones, due to falling out of trees on a regular basis.",
"Hummingbirds are the only known birds that can also fly backwards",
"The pangolin is able to roll up into an armour-plated ball, so lions can't eat them.",
];

const fn select_random(arr: &[&'static str], rand_num: usize) -> &'static str {
    arr[rand_num]
}

static QA_MAP: phf::Map<&'static str, fn(usize) -> &'static str> = phf_map! {
    "can you tell me a fact about animals?" => |rand_num| {select_random(&ANIMAL_FACTS,rand_num)},
    "give me a famous quote ?" => |rand_num| {select_random(&QUOTES,rand_num)},
    "how many countries on earth are there ?" => |_| {"There are 195 Countries On earth"},
    "do we have a room temperature ambient pressure super conductor ?" => |_| {"Its still up for debate, but i sure hope so."},
    "what do vc's in silicon valley wear ?" => |_| {"anything that goes with a Patagonia Beter Sweater"},
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let discord_token = std::env::var("discord_token").unwrap();
    let bot = ProvidedBot::new(discord_token);

    bot.listen(|msg| handler(&bot, msg)).await;
    Ok(())
}

async fn handler(bot: &ProvidedBot, msg: Message) {
    logger::init();
    let discord = bot.get_client();

    if msg.author.bot {
        log::debug!("ignored bot message");
        return;
    }

    if msg.member.is_some() {
        log::debug!("ignored channel message");
        return;
    }

    let rand_num = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..N)
    };

    let formatted_questions = QA_MAP
        .keys()
        .enumerate()
        .fold(String::new(), |acc, (i, n)| format!("{}\n{}.{}", acc, i, n));

    let resp = match QA_MAP.get(&msg.content) {
        Some(my_func) => my_func(rand_num).to_string(),
        None => {
            format!("Unfortunately i am limited in my responses, try asking one of these questions '{:?}",formatted_questions)
        }
    };

    _ = discord
        .send_message(
            msg.channel_id.into(),
            &serde_json::json!({
                "content": resp
            }),
        )
        .await;
}
