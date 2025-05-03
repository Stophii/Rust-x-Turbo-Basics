
turbo::init! {
    struct GameState {
        //name: type,
        decimal_number: f32,
        positive_number: u32,
        negative_number: i32,
        true_or_false: bool,
        words: String,
        list_of_decimal_numbers: Vec<f32>,
        list_of_words: Vec<String>,
        optional_positive_number: Option<u32>,
    } = {
        Self {
            //name: value,
            decimal_number: 0.0,
            positive_number: 1,
            negative_number: -1,
            true_or_false: false,
            words: "Hello, World!".to_string(),
            list_of_decimal_numbers: Vec::new(),
            list_of_words: Vec::new(),
            optional_positive_number: None,
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    if gamepad(0).a.just_pressed() { //this is z on the keyboard and a on a gamepad!
        state.true_or_false = !state.true_or_false;
        log!("value changed from {} to {}", !state.true_or_false, state.true_or_false)
    }
    if gamepad(0).b.pressed() { //this is x on the keyboard and b on a gamepad!
        rect!( x = 128, y = 72, w = state.positive_number, h = state.decimal_number, color = 0xffffffff); // Basic rectangle macro!
    }
    if gamepad(0).start.just_pressed() { //this is space on the keyboard and start on a gamepad!
        state.list_of_decimal_numbers.push(1.1);
        state.list_of_decimal_numbers.push(2.2);
        state.list_of_decimal_numbers.push(3.3);
        log!("added the numbers!");
    }
    if gamepad(0).up.just_pressed() { //this is up arrow-key or w on the keyboard and dpad up on a gamepad!
        state.decimal_number += 1.2;
        state.positive_number += 1;
        state.negative_number -= 1;
        log!("my numbers have increased!")
    }

    let example = format!("decmial number:{}\npositive_number:{}\nnegative_number:{}\nbool:{}", state.decimal_number, state.positive_number, state.negative_number, state.true_or_false);
    text!(&example, x = 40, y = 5, font = "medium", color = 0xffffffff); // Basic text macro!

    let vector = format!("decimal list: {:?}", state.list_of_decimal_numbers);
    text!(&vector, x = 40, y = 55, font = "medium", color = 0xffffffff); // Basic text macro!

    state.save();
}
