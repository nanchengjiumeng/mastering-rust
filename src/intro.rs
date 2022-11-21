use std::collections::HashMap;

pub fn base_closure() {
    let doubler = |x| 2 * x;
    let value = 5;
    let twice = doubler(value);
    println!("{} double is {}", value, twice);

    let big_doubler = |b, c| {
        let z = b + c;
        z * twice
    };
    let some_number = big_doubler(1, 2);
    println!("Result from closure: {}", some_number);
}

pub fn base_string() {
    let question = "How old are you?";
    let person: String = "XiaoBai".to_string();
    let nameste = "Монгол хэл";
    println!("{}! {} {}", person, question, nameste);
}

pub fn base_if_else() {
    let rust_is_awesome = false;
    if rust_is_awesome {
        println!("Indeed");
    } else {
        println!("Well, you shuld try Rust!");
    }
}

pub fn base_if_assign() {
    let result = if 1 != 2 {
        "Wait, What ?"
    } else {
        "Rust makes sense"
    };
    println!("You know what? {}.", result);
}

pub fn base_if_esle_no_value() {
    let result = if 1 == 2 {
        "Nothing makes sense";
    } else {
        "Sanity reigns";
    };
    println!("Result of computation: {:?}", result);
}

fn req_status() -> u32 {
    200
}

pub fn base_match() {
    let status = req_status();
    match status {
        200 => println!("success"),
        404 => println!("Not Found"),
        other => {
            println!("Request failed with code: {}", other)
        }
    }
}

pub fn base_for() {
    let mut x = 1024;
    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go", x);
        x -= 1;
    }
}

fn base_loop_lables(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            loop {
                if dec == 0 {
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}

pub fn test_base_loop_lables() {
    let res = base_loop_lables(10, 5);
    println!("{} minus {} is {}", 10, 5, res)
}

pub fn base_while() {
    let mut x = 1000;
    while x > 0 {
        println!("{} more runs to go", x);
        x -= 1;
    }
}

pub fn base_for_loops() {
    println!("Normal ranges:");
    for i in 0..10 {
        print!("{}", i);
    }
    println!();
    print!("Inclusive ranges: ");
    for i in 0..=10 {
        print!("{}", i)
    }
    println!();
}

struct Player {
    name: String,
    iq: u8,
    frends: u8,
    score: u16,
}

fn bump_player_score(mut player: Player) {
    // fn bump_player_score(player: Player) {
    player.score += 1;
    println!("Updated player ststus:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.frends);
    println!("Score: {}", player.score);
}

pub fn base_struct() {
    let name = "Wang".to_string();
    let player = Player {
        name,
        iq: 171,
        frends: 150,
        score: 199,
    };
    bump_player_score(player);
}

pub fn base_struct_tuple() {
    struct Color(u8, u8, u8);
    let white = Color(255, 255, 255);
    let red = white.0;
    let green = white.1;
    let blue = white.2;
    println!("R: {}, G: {}, B: {}", red, green, blue);
    let orange = Color(255, 165, 0);
    let Color(r, _, b) = orange;
    println!("R: {}, G: skip, B: {}", r, b);
}

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}
pub enum PlayerAction {
    Move { direction: Direction, speed: u8 },
    Wait,
    Attack(Direction),
}

pub fn base_emun() {
    let simulated_player_action = PlayerAction::Move {
        direction: Direction::E,
        speed: 2,
    };
    match simulated_player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move { direction, speed } => {
            println!(
                "Player wants to move in direction {:?} with speed {}",
                direction, speed
            )
        }
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    }
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            frends: 100,
            score: 100,
        }
    }
    fn get_frients(&self) -> u8 {
        self.frends
    }
    fn set_frients(&mut self, count: u8) {
        self.frends = count;
    }
}

pub fn base_struct_methods() {
    let mut player = Player::with_name("wang");
    player.set_frients(20);
    println!("{}'s firends count: {}", player.name, player.get_frients());
    println!(
        "{}'s firends count: {}",
        player.name,
        Player::get_frients(&player)
    );
}

pub enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {}", amt)
}

fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {}", amt)
}

fn pay_by_paypal(amt: u64) {
    println!("Processing paypal payment of {}", amt)
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Paypal => pay_by_paypal(amount),
        }
    }
}

pub fn base_emun_methods() {
    let payment_mode = PaymentMode::Debit;
    payment_mode.pay(1000)
}

pub fn base_array() {
    let number: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.1f64, 0.2, 0.3];
    println!("Number :{}", number[5]);
    println!("Number :{}", floats[2]);
}

pub fn base_tuples() {
    let num_and_str: (u8, &str) = (40, "Have a good day");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);
}

pub fn base_vec() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![1];
    vec_with_macro.push(2);
    let _ = vec_with_macro.pop();

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };
    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}

pub fn base_hashmaps() {
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    fruits.remove("orange");
    fruits.insert("avocado", fruits["avocado"] + 5);
    println!("\nI now have {} avocados", fruits["avocado"]);
}

pub fn base_slice() {
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }
    {
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
    }

    println!("Look ma! I can modify through slices: {:?}", numbers);
}
