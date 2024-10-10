use std::io;

const WIDTH: usize = 101;
const W: usize = 49;
const H: usize = 22;
const KOEF_1: f64 = 0.33;
const KOEF_2: f64 = 0.66;

fn main() {
    game();
}

fn game() {
    let mut place: [[char;W];H] = [[' '; W]; H];

    create_place(&mut place);
    let mut player: u8 = 1;

    loop {

        let mut message_error = "";
        let x: usize;
        let y: usize;
        loop {
            print_legent(player,message_error);
            print_place(place);

            let mut num = String::new();
            io::stdin().read_line(&mut num).expect("Ошибка");
            //let num = num.trim().parse::<i32>().unwrap();

            if num.matches("1").count()>0 {
                x = 0;
                y = 0;
                break;
            }
            else if num.matches("2").count()>0 {
                x = (W as f64 * KOEF_1)as usize;
                y = 0;
                break;
            }
            else if num.matches("3").count()>0 {
                x = (W as f64 * KOEF_2)as usize;
                y = 0;
                break;
            }
            else if num.matches("4").count()>0 {
                x = 0;
                y = (H as f64 * KOEF_1)as usize;
                break;
            }
            else if num.matches("5").count()>0 {
                x = (W as f64 * KOEF_1)as usize;
                y = (H as f64 * KOEF_1)as usize;
                break;
            }
            else if num.matches("6").count()>0 {
                x = (W as f64 * KOEF_2)as usize;
                y = (H as f64 * KOEF_1)as usize;
                break;
            }
            else if num.matches("7").count()>0 {
                x = 0;
                y = (H as f64 * KOEF_2)as usize;
                break;
            }
            else if num.matches("8").count()>0 {
                x = (W as f64 * KOEF_1)as usize;
                y = (H as f64 * KOEF_2)as usize;
                break;
            }
            else if num.matches("9").count()>0 {
                x = (W as f64 * KOEF_2)as usize;
                y = (H as f64 * KOEF_2)as usize;
                break;
            }
            else {
                message_error = "Введите номер ячейки";
                continue;
            }
        }
        if player==1 {
            add_zero(&mut place, x, y);
            player=2;
        }
        else {
            add_cross(&mut place, x, y);
            player=1;
        }
    }
}

fn print_place(place: [[char;W];H]) {
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

fn print_legent(player: u8, string: &str) {
    let border = || { 
        for _i in 0..WIDTH {
            print!("=");
        }
        println!("");
    };
    let print_mes = |mes: &str| {
        let margin: i32 = ((WIDTH - mes.chars().count())/2).try_into().unwrap();
        for _i in 1..margin {
            print!(" ");
        }
        print!("{}",mes);
        for _i in 1..margin {
            print!(" ");
        }
        println!("");
    };

    border();
    let message = format!("Ходит Игрок №{}",player);
    print_mes(&message);
    print_mes(string);
    border();
}

fn add_zero(place: &mut[[char;W];H], x: usize, y: usize) {
    let margin_y = (H/3-4)/2+1 as usize;
    let margin_x = (W/3-7)/2+1 as usize;
    for i in 0..3 {
        place[y+margin_y][x+margin_x+2+i]='●';
    }
    for i in 0..2 {
        place[y+margin_y+1+i][x+margin_x]='●';
        place[y+margin_y+1+i][x+margin_x+6]='●';
    }
    for i in 0..3 {
        place[y+margin_y+3][x+margin_x+2+i]='●';
    }
}

fn add_cross(place: &mut[[char;W];H], x: usize, y: usize) {
    let margin_y = (H/3-4)/2+1 as usize;
    let margin_x = (W/3-7)/2+1 as usize;
    for i in 0..7 {
        place[y+margin_y+1][x+margin_x+i]='▃';
        place[y+margin_y+1+1][x+margin_x+i]='▀';
    }
    for i in 0..4 {
        place[y+margin_y+i][x+margin_x+2+1]='█';
    }
}

fn create_place(place: &mut[[char;W];H]) {
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