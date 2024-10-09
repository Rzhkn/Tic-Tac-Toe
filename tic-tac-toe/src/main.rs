const W: usize = 30;

fn main() {
    game();
}

fn game() {
    let mut place: [[char;W];W] = [[' '; W]; W];

    for i in 0..W {
        for j in 0..W {
            if i==0 || j==0 || i==W-1 || j==W-1 || j==(W as f64/0.3)as usize || j==(W as f64/0.6)as usize || i==(W as f64/0.3)as usize || i==(W as f64/0.6)as usize {
                place[i][j]='0';
            }
        }
    }

    print_legent(1);
    print_place(place);
}

fn print_place(place: [[char;W];W]) {
    for i in 0..W {
        for j in 0..W {
            print!("{} ",place[i][j]);
        }
        println!("");
    }
}

fn print_legent(player: u8) {
    let border = || { 
        for _i in 0..W {
            print!("=");
        }
        println!("");
    };
    let print_mes = |mes: &str| {
        let margin: i32 = ((W - mes.chars().count())/2).try_into().unwrap();
        for _i in 0..margin {
            print!(" ");
        }
        print!("{}",mes);
        for _i in 0..margin {
            print!(" ");
        }
        println!("");
    };

    border();
    let message = format!("Ходит Игрок №{}",player);
    print_mes(&message);
    border();
    println!("");
}

