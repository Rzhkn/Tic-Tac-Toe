use std::io;

const WIDTH: usize = 101;
const W: usize = 49;
const H: usize = 25;
const KOEF_1: f64 = 0.33;
const KOEF_2: f64 = 0.66;

fn main() {
    game();

    println!("Для завершения программы нажмите на любую кнопку: ");
    io::stdin().read_line(&mut String::new()).expect("Ошибка");
}

fn game() {
    let mut place: [[char;W];H] = [[' '; W]; H];
    let mut place_2: [[u8;W];H] = [[0; W]; H];

    create_place(&mut place);
    let mut player: u8 = 1;

    loop {
        let mut x: usize = 0;
        let mut y: usize = 0;
        
        new_coordinates(&mut x, &mut y, player, &mut place, &mut place_2);

        if player==1 {
            add_zero(&mut place, x, y);
            if winner_1(&place_2)==1 {
                println!("Победил Игрок №1!");
                break;
            }
            player=2;
        }
        else {
            add_cross(&mut place, x, y);
            if winner_2(&place_2)==1 {
                println!("Победил Игрок №2!");
                break;
            }
            player=1;
        }
    }

    print_place(place);
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
    let margin_y = (H/3-5)/2+1 as usize;
    let margin_x = (W/3-7)/2+1 as usize;
    for i in 0..3 {
        place[y+margin_y][x+margin_x+2+i]='●';
    }
    for i in 0..3 {
        place[y+margin_y+1+i][x+margin_x]='●';
        place[y+margin_y+1+i][x+margin_x+6]='●';
    }
    for i in 0..3 {
        place[y+margin_y+4][x+margin_x+2+i]='●';
    }
}

fn add_cross(place: &mut[[char;W];H], x: usize, y: usize) {
    let margin_y = (H/3-5)/2+1 as usize;
    let margin_x = (W/3-7)/2+1 as usize;
    place[y+margin_y][x+margin_x]='X';
    place[y+margin_y][x+margin_x+6]='X';
    place[y+margin_y+1][x+margin_x+2]='X';
    place[y+margin_y+1][x+margin_x+4]='X';
    place[y+margin_y+2][x+margin_x+3]='X';
    place[y+margin_y+3][x+margin_x+2]='X';
    place[y+margin_y+3][x+margin_x+4]='X';
    place[y+margin_y+4][x+margin_x]='X';
    place[y+margin_y+4][x+margin_x+6]='X';
}

fn winner_1 (place: &[[u8;W];H]) -> u8 {
    let mut f: u8 = 0;
    if place[0][0]==1 && place[0][1]==1 && place[0][2]==1 ||
        place[1][0]==1 && place[1][1]==1 && place[1][2]==1 ||
        place[2][0]==1 && place[2][1]==1 && place[2][2]==1 ||
        place[0][0]==1 && place[1][0]==1 && place[2][0]==1 ||
        place[0][1]==1 && place[1][1]==1 && place[2][1]==1 ||
        place[0][2]==1 && place[1][2]==1 && place[2][2]==1 ||
        place[1][1]==1 && (place[0][0]==1 &&  place[2][2]==1 || place[0][2]==1 && place[2][0]==1) {
            f=1;
        }
    f
}

fn winner_2 (place: &[[u8;W];H]) -> u8 {
    let mut f: u8 = 0;
    if place[0][0]==2 && place[0][1]==2 && place[0][2]==2 ||
        place[1][0]==2 && place[1][1]==2 && place[1][2]==2 ||
        place[2][0]==2 && place[2][1]==2 && place[2][2]==2 ||
        place[0][0]==2 && place[1][0]==2 && place[2][0]==2 ||
        place[0][1]==2 && place[1][1]==2 && place[2][1]==2 ||
        place[0][2]==2 && place[1][2]==2 && place[2][2]==2 ||
        place[1][1]==2 && (place[0][0]==2 &&  place[2][2]==2 || place[0][2]==2 && place[2][0]==2) {
            f=1;
        }
    f
}

fn new_coordinates(x: &mut usize, y: &mut usize, player: u8, place: &mut[[char;W];H], place_2: &mut[[u8;W];H]) {
    let add_hod = |player: u8, place_2: &mut[[u8;W];H], x: usize, y: usize| {
        if player==1 {
            place_2[x][y]=1;
        }
        else {
            place_2[x][y]=2;
        }
    };

    let mut message_error = "";
    loop {
        print_legent(player,message_error);
        print_place(*place);

        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Ошибка");
        //let num = num.trim().parse::<i32>().unwrap();

        if num.matches("1").count()>0 {
            if place_2[0][0]==0 {
                *x = 0;
                *y = 0;
                add_hod(player, place_2, 0, 0);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("2").count()>0{
            if place_2[0][1]==0 {
                *x = (W as f64 * KOEF_1)as usize;
                *y = 0;
                add_hod(player, place_2, 0, 1);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("3").count()>0 {
            if place_2[0][2]==0 {
                *x = (W as f64 * KOEF_2)as usize;
                *y = 0;
                add_hod(player, place_2, 0, 2);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("4").count()>0 {
            if place_2[1][0]==0 {
                *x = 0;
                *y = (H as f64 * KOEF_1)as usize;
                add_hod(player, place_2, 1, 0);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("5").count()>0 {
            if place_2[1][1]==0 {
                *x = (W as f64 * KOEF_1)as usize;
                *y = (H as f64 * KOEF_1)as usize;
                add_hod(player, place_2, 1, 1);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("6").count()>0 {
            if place_2[1][2]==0 {
                *x = (W as f64 * KOEF_2)as usize;
                *y = (H as f64 * KOEF_1)as usize;
                add_hod(player, place_2, 1, 2);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("7").count()>0 {
            if place_2[2][0]==0 {
                *x = 0;
                *y = (H as f64 * KOEF_2)as usize;
                add_hod(player, place_2, 2, 0);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("8").count()>0 {
            if place_2[2][1]==0 {
                *x = (W as f64 * KOEF_1)as usize;
                *y = (H as f64 * KOEF_2)as usize;
                add_hod(player, place_2, 2, 1);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else if num.matches("9").count()>0 {
            if place_2[2][2]==0 {
                *x = (W as f64 * KOEF_2)as usize;
                *y = (H as f64 * KOEF_2)as usize;
                add_hod(player,place_2, 2, 2);
                break;
            }
            else {
                message_error = "Ячейка занята, введите другой номер";
                continue;
            }
        }
        else {
            message_error = "Введите номер ячейки";
            continue;
        }
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