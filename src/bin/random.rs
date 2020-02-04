extern crate regex;
use regex::Regex;

fn main() {
    let msg: &str = ":) ?";
    let msg_new = msg.replace("\\W", "");
    println!("{:?}", msg_new);
    let response = match msg.trim() {
        msg if msg.ends_with("?") => "Sure.",
        msg if msg.to_uppercase() == msg && msg.ends_with("?") => "Calm down, I know what I'm doing!",
        "" => "Fine. Be that way!",
        msg if msg.to_uppercase() == msg => "Whoa, chill out!",
        _ => "Whatever."
    };
    println!("{:?}", response);

    // println!("{:?}", msg.replace("\w", ""));
}


// Bob is a lackadaisical teenager. In conversation, his responses are very limited.

// Bob answers 'Sure.' if you ask him a question, such as "How are you?".

// He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).

// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

// He says 'Fine. Be that way!' if you address him without actually saying anything.

// He answers 'Whatever.' to anything else.

// Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.