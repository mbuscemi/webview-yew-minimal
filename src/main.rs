mod inline_code;

use web_view::*;
use rand::{thread_rng, Rng};

fn main() {
    web_view::builder()
        .title("WebView/Yew Minimal")
        .content(Content::Html(inline_code::html()))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "receive" => {
                    webview.eval(
                        &format!(
                            r#"document.dispatchEvent(
                                new CustomEvent("send_back", {{ detail: {{ communique: "{}" }} }})
                            );"#,
                            random_communique(),
                        )
                    ).expect("failed to dispatch event to Yew");
                }
                _ => ()
            }
            Ok(())
        })
        .run()
        .unwrap();
}

fn random_communique() -> String {
    let communiques: [&str; 10] = [
        "Your mother was a hamster, and your father smelt of elderberries!",
        "You've got two empty halves of coconuts and your bangin' 'em together.",
        "All right, we'll call it a draw.",
        "Oh, stop your whining. First we kill him, then we have biscuts and tea.",
        "What is the airspeed velocity of an unladen swallow?",
        "Once the number three, being the third number, be reached, then lobbest thou thy Holy Hand Grenade of Antioch towards thy foe, who, being naughty in my sight, shall snuff it.",
        "Oh, look. There's some lovely filth over here.",
        "What are you going to do, bleed on me?",
        "Help, help, I'm being oppressed! Come see the violence inherent in the system!",
        "Strange women lying in ponds distributing swords is no basis for a system of government. Supreme executive power derives from a mandate from the masses, not from some farcical aquatic ceremony.",
    ];

    let mut rng = thread_rng();
    let index: u32 = rng.gen_range(0, 10);
    communiques[index as usize].to_string()
}