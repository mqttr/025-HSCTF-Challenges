use text_io::read;

pub fn print_intro_fast(base_directory: &str) {
    println!("Sam: Thanks for agreeing on helping me out.");
    println!("Sam: I'll create a new directory at ");
    println!("Sam: \"{}\"", base_directory);
    println!("Sam: and send everything there.");
    println!("Sam: It should start soon.");
}

pub fn print_intro(base_directory: &str) {
    println!("Sam: Hey, Dude. I really need you're help.");
    println!("Sam: You know that flag that Barry asked me to get?");
    println!("Sam: Well I sorta dropped it into a directory full of a bunch of other flags.");
    println!("Sam: Can you help me? You'd be doing me a huge favor!");

    match get_y_or_n("\nYou: ", "Sam: I'm sorry I don't know what you mean by that...") {
        true => (),
        false => {
            println!("Sam: Oh... nvm im sorry to bother you");
            std::process::exit(0);
        }
    }

    println!("Sam: Alrightly! I'll create a new directory at ");
    println!("Sam: \"{}\"", base_directory);
    println!("Sam: and send everything there if you're ready for me to now?");

    match get_y_or_n("\nYou: ", "Sam: I'm sorry I don't know what you mean by that...") {
        true => (),
        false => {
            println!("Sam: Oh... uhm I'll just wait I guess.");

            print!("\nYou: ");
            let _: String = read!();

            println!("\nSam: Does that mean you are ready now?");


            loop {
                match get_y_or_n("\nYou: ", "Sam: I'm sorry I don't know what you mean by that...") {
                    true => { 
                        break;
                    }
                    false => {
                        println!("Sam: Oh... uhm I'll just wait I guess.\n");
                    }
                }
                print!("You: ");
                let _: String = read!();
    
                println!("\nSam: Does that mean you are ready now?");
            }
        }
    }



    println!("Sam: Okay I initiated the file transfer if all goes well it should start soon...");
    println!("Sam: Thanks again this really means a lot. You're really saving my skin.");
    println!("Sam: Oh and one other thing. The flag that Barry asked me to get had an even amount of characters.");
    println!("Sam: And it was in the form \"flag{{base64_encode}}\".");
    println!("Sam: I'm sorry I couldn't help more");
    println!("Sam: Thank you so much!!! I owe you one!");

    println!("System: File transfer started...");

}

fn completed_flag_generation() {
    println!("System: File transfer completed...");
    
    println!("Sam: Okay. It should be done saving all the data to your computer.");
    println!("Sam: Oh if you do find it. Barry asked me to \"submit it to the jeapardy board\".");
    println!("Sam: If I'm being quite Frank, I'm not sure what he meant by that.");
    println!("Sam: If you could do that for me it would be really sick too!!");
    println!("Sam: I owe you big time!!");

    println!("Sam: bee tea dubs, for next time you can skip my dialogue if you pass in any parameter!");
}

fn get_y_or_n(user_prompt: &str, ambiguity_response: &str) -> bool {
    print!("{}", &user_prompt);
    loop {
        let mut r: String = read!();
        println!();
        r = r.trim().to_lowercase();

        if [ "yes", "y", "yeah", "yea", "ofc", "of course" ].contains(&r.as_ref()) {
            return true;
        } else if [ "no", "n", "naw", "never", "nope" ].contains(&r.as_ref()) {
            return false; 
        } else {
            println!("{}", &ambiguity_response);
            print!("{}", &user_prompt);
        }
    };
}