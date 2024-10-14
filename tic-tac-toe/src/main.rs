use std::io;

mod views;
use views::{print_legent, print_place, create_place};

mod controls;
use controls::{add_cross, add_zero, winner_1, winner_2, winner_all};

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
                print_legent(player,"",1);
                break;
            } else if winner_all(&place_2)==1 {
                print_legent(player,"",3);
                break;
            }
            player=2;
        }
        else {
            add_cross(&mut place, x, y);
            if winner_2(&place_2)==1 {
                print_legent(player,"",2);
                break;
            } else if winner_all(&place_2)==1 {
                print_legent(player,"",3);
                break;
            }
            player=1;
        }
    }

    print_place(place);
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

    let checked = |player: u8, place_2: &mut[[u8;W];H], x: &mut usize, y: &mut usize, x2: usize, y2: usize, x3: usize, y3: usize, message_error: &mut &str| -> u8 {
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
    };

    let mut message_error = "";
    loop {
        print_legent(player,message_error,0);
        print_place(*place);

        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Ошибка");

        if num.matches("1").count()>0 {
            if checked(player, place_2, x, y, 0, 0, 0, 0, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("2").count()>0{
            if checked(player, place_2, x, y, 0, 1, (W as f64 * KOEF_1)as usize, 0, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("3").count()>0 {
            if checked(player, place_2, x, y, 0, 2, (W as f64 * KOEF_2)as usize, 0, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("4").count()>0 {
            if checked(player, place_2, x, y, 1, 0, 0, (H as f64 * KOEF_1)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("5").count()>0 {
            if checked(player, place_2, x, y, 1, 1, (W as f64 * KOEF_1)as usize, (H as f64 * KOEF_1)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("6").count()>0 {
            if checked(player, place_2, x, y, 1, 2, (W as f64 * KOEF_2)as usize, (H as f64 * KOEF_1)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("7").count()>0 {
            if checked(player, place_2, x, y, 2, 0, 0, (H as f64 * KOEF_2)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("8").count()>0 {
            if checked(player, place_2, x, y, 2, 1, (W as f64 * KOEF_1)as usize, (H as f64 * KOEF_2)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else if num.matches("9").count()>0 {
            if checked(player, place_2, x, y, 2, 2, (W as f64 * KOEF_2)as usize, (H as f64 * KOEF_2)as usize, &mut message_error)==1 {break;} else {continue;}
        }
        else {
            message_error = "Введите номер ячейки";
            continue;
        }
    }
}