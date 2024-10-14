const WIDTH: usize = 101;
const W: usize = 49;
const H: usize = 25;
const KOEF_1: f64 = 0.33;
const KOEF_2: f64 = 0.66;

fn border() { 
    for _i in 0..WIDTH {
        print!("=");
    }
    println!("");
}

fn print_mes(mes: &str) {
    let margin: i32 = ((WIDTH - mes.chars().count())/2).try_into().unwrap();
    for _i in 1..margin {
        print!(" ");
    }
    print!("{}",mes);
    for _i in 1..margin {
        print!(" ");
    }
    println!("");
}

pub fn print_start(message_error: &str) {
    border();
    print_mes("Выберите режим игры:");
    print_mes("1 - с ботом          2 - с другом");
    print_mes(message_error);
    border();
    println!("");
}

pub fn print_legent(player: u8, string: &str, finish: u8) {
    border();
    if finish==0 {
        let message = format!("Ходит Игрок №{}",player);
        print_mes(&message);
        print_mes(string);
    } else if finish==1 || finish==2 {
        print_mes(&format!("Победил Игрок №{}!",finish));
    } else{
        print_mes("Победила дружба!");
    }
    border();
}

pub fn print_place(place: [[char;W];H]) {
    for i in 0..H {
        for _i in 0..(WIDTH-W)/2 {
            print!(" ");
        }
        for j in 0..W {
            print!("{}",place[i][j]);
        }
        for _i in 0..(WIDTH-W)/2 {
            print!(" ");
        }
        println!("");
    }
}

pub fn create_place(place: &mut[[char;W];H]) {
    let set_w1 = (W as f64 * KOEF_1)as usize;
    let set_w2 = (W as f64 * KOEF_2)as usize;
    let set_h1 = (H as f64 * KOEF_1)as usize;
    let set_h2 = (H as f64 * KOEF_2)as usize; 

    for i in 0..H {
        for j in 0..W {
            if i==0 && j==0 {
                place[i][j]='┌';
            }
            else if i==0 && j==W-1 {
                place[i][j]='┐';
            }
            else if i==H-1 && j==0 {
                place[i][j]='└';
            }
            else if i==H-1 && j==W-1 {
                place[i][j]='┘';
            }
            else if (i==0 && j==set_w1) || (i==0 && j==set_w2) {
                place[i][j]='┬';
            }
            else if (i==H-1 && j==set_w1) || (i==H-1 && j==set_w2) {
                place[i][j]='┴';
            }
            else if (j==0 && i==set_h1) || (j==0 && i==set_h2) {
                place[i][j]='├';
            }
            else if (j==W-1 && i==set_h1) || (j==W-1 && i==set_h2) {
                place[i][j]='┤';
            }
            else if (i==set_h1 && j==set_w1) || (i==set_h1 && j==set_w2)
                    || (i==set_h2 && j==set_w1) || (i==set_h2 && j==set_w2) {
                place[i][j]='┼';        
            }
            else if j==0  || j==W-1 || j==set_w1 || j==set_w2 {
                place[i][j]='│';
            }
            else if i==0 || i==H-1 || i==set_h1 || i==set_h2 {
                place[i][j]='─';
            }
            
        }
    }

    place[1][1]='1';
    place[1][set_w1+1]='2';
    place[1][set_w2+1]='3';
    place[set_h1+1][1]='4';
    place[set_h1+1][set_w1+1]='5';
    place[set_h1+1][set_w2+1]='6';
    place[set_h2+1][1]='7';
    place[set_h2+1][set_w1+1]='8';
    place[set_h2+1][set_w2+1]='9';
}