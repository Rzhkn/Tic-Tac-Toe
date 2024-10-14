const W: usize = 49;
const H: usize = 25;

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