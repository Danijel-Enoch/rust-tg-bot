use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();
    println!("Bot started!");
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, "ℹ️ Help: 
 
Which tokens can I trade? 
Any SPL token that is a SOL pair, on Raydium, pump.fun, Meteora, Moonshot, or Jupiter, and will integrate more platforms on a rolling basis. We pick up pairs instantly, and Jupiter will pick up non-SOL pairs within approx. 15 minutes. 
 
How can I see how much money I've made from referrals? 
Tap the referrals button or type /referrals to see your referral rewards! 
  
How do I create a new wallet on SunamiBot? 
Tap the Wallet button or type /wallet, and you'll be able to configure your new wallets! 
 
Is SunamiBot free? How much do I pay for transactions? 
SunamiBot is completely free! We charge 1% on transactions, and keep the bot free so that anyone can use it. 
 
Why is my Net Profit lower than expected? 
Your Net Profit is calculated after deducting all associated costs, including Price Impact, Transfer Tax, Dex Fees, and a 1% SunamiBot fee. This ensures the figure you see is what you actually receive, accounting for all transaction-related expenses.").await?;
        Ok(())
    })
    .await;
}
