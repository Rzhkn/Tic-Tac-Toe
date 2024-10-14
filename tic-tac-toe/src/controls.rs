const W: usize = 49;
const H: usize = 25;
const KOEF_1: f64 = 0.33;
const KOEF_2: f64 = 0.66;

pub fn add_zero(place: &mut[[char;W];H], x: usize, y: usize) {
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

pub fn add_cross(place: &mut[[char;W];H], x: usize, y: usize) {
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

pub fn winner_1 (place: &[[u8;W];H]) -> u8 {
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

pub fn winner_2 (place: &[[u8;W];H]) -> u8 {
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

pub fn winner_all (place: &[[u8;W];H]) -> u8 {
    let mut f: u8=0;
    let mut count = 0;
    for i in 0..H {
        for j in 0..W {
            if place[i][j]!=0 {count+=1;}
        }
    }
    if count==9 {
        f=1;
    }
    f
}

fn add_hod (player: u8, place_2: &mut[[u8;W];H], x: usize, y: usize) {
    if player==1 {
        place_2[x][y]=1;
    }
    else {
        place_2[x][y]=2;
    }
}

fn checked (player: u8, place_2: &mut[[u8;W];H], x: &mut usize, y: &mut usize, x2: usize, y2: usize, x3: usize, y3: usize, message_error: &mut &str) -> u8 {
    let mut f:u8 = 0;
    if place_2[x2][y2]==0 {
        *x = x3;
        *y = y3;
        add_hod(player, place_2, x2, y2);
        f=1;
    }
    else {
        *message_error = "Ячейка занята, введите другой номер";
    }
    f
}

pub fn cell (num: u8, player: u8, place_2: &mut[[u8;W];H], x: &mut usize, y: &mut usize, message_error: &mut &str) -> u8 {
    let f: u8;
    match num {
        1 => if checked(player, place_2, x, y, 0, 0, 0, 0, message_error)==1 {f=1;} else {f=0;},
        2 => if checked(player, place_2, x, y, 0, 1, (W as f64 * KOEF_1)as usize, 0, message_error)==1 {f=1;} else {f=0;},
        3 => if checked(player, place_2, x, y, 0, 2, (W as f64 * KOEF_2)as usize, 0, message_error)==1 {f=1;} else {f=0;},
        4 => if checked(player, place_2, x, y, 1, 0, 0, (H as f64 * KOEF_1)as usize, message_error)==1 {f=1;} else {f=0;},
        5 => if checked(player, place_2, x, y, 1, 1, (W as f64 * KOEF_1)as usize, (H as f64 * KOEF_1)as usize, message_error)==1 {f=1;} else {f=0;},
        6 => if checked(player, place_2, x, y, 1, 2, (W as f64 * KOEF_2)as usize, (H as f64 * KOEF_1)as usize, message_error)==1 {f=1;} else {f=0;},
        7 => if checked(player, place_2, x, y, 2, 0, 0, (H as f64 * KOEF_2)as usize, message_error)==1 {f=1;} else {f=0;},
        8 => if checked(player, place_2, x, y, 2, 1, (W as f64 * KOEF_1)as usize, (H as f64 * KOEF_2)as usize, message_error)==1 {f=1;} else {f=0;},
        9 => if checked(player, place_2, x, y, 2, 2, (W as f64 * KOEF_2)as usize, (H as f64 * KOEF_2)as usize, message_error)==1 {f=1;} else {f=0;},
        _ => f=0
    }
    f
}

pub fn bot(x: &mut usize, y: &mut usize, _place: &mut[[char;W];H], place_2: &mut[[u8;W];H]) {
    extern crate rand;
    use rand::Rng;
    
    let mut rng = rand::thread_rng();

    let mut message_error = "";

    while cell(rng.gen_range::<u8>(1,10),2,place_2,x,y,&mut message_error)!=1 {

    }
}