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
    add_zero(&mut place, 0, 0);
    add_cross(&mut place, 16, 0);

    print_legent(1);
    print_place(place);
}

fn print_place(place: [[char;W];H]) {
    for i in 0..H {
        for j in 0..W {
            print!("{}",place[i][j]);
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

fn add_zero(place: &mut[[char;W];H], x: usize, y: usize) {
    let margin_y = (H/3-4)/2+1 as usize;
    let margin_x = (W/3-7)/2+1 as usize;
    for i in 0..3 {
        place[y+margin_y][x+margin_x+2+i]='o';
    }
    for i in 0..2 {
        place[y+margin_y+1+i][x+margin_x]='o';
        place[y+margin_y+1+i][x+margin_x+6]='o';
    }
    for i in 0..3 {
        place[y+margin_y+3][x+margin_x+2+i]='o';
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
}