use std::io;

mod views;
use views::{print_start, print_legent, print_place, create_place};

mod controls;
use controls::{add_cross, add_zero, winner_1, winner_2, winner_all, cell, bot};

const W: usize = 49;
const H: usize = 25;

fn main() {
    let f: u8;
    let mut message_error="";

    loop {
        print_start(message_error);
        let mut mode = String::new();
        io::stdin().read_line(&mut mode).expect("Ошибка");

        if mode.trim().chars().count()==1 {
            if mode.matches("1").count()>0 {f=1; break;}
            else if mode.matches("2").count()>0 {f=2; break;}
            else {message_error="Введите номер желаемого режима";}
        } else {message_error="Введите номер желаемого режима";}
    }
    
    game(f);

    println!("Для завершения программы нажмите на любую кнопку: ");
    io::stdin().read_line(&mut String::new()).expect("Ошибка");
}

fn game(mode: u8) {
    let mut place: [[char;W];H] = [[' '; W]; H];
    let mut place_2: [[u8;W];H] = [[0; W]; H];

    create_place(&mut place);
    let mut player: u8 = 1;

    loop {
        let mut x: usize = 0;
        let mut y: usize = 0;

        if player==1 {
            new_coordinates(&mut x, &mut y, player, &mut place, &mut place_2);
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
            if mode==1 {
                bot(&mut x, &mut y, &mut place, &mut place_2);
            }
            else {
                new_coordinates(&mut x, &mut y, player, &mut place, &mut place_2);
            }
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
    let mut message_error = "";
    loop {
        print_legent(player,message_error,0);
        print_place(*place);

        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Ошибка");

        if num.trim().chars().count()==1 {
            if num.matches("1").count()>0 {
                if cell(1, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("2").count()>0{
                if cell(2, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("3").count()>0 {
                if cell(3, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("4").count()>0 {
                if cell(4, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("5").count()>0 {
                if cell(5, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("6").count()>0 {
                if cell(6, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("7").count()>0 {
                if cell(7, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("8").count()>0 {
                if cell(8, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else if num.matches("9").count()>0 {
                if cell(9, player, place_2, x, y, &mut message_error)==1 {break;} else {continue;}
            }
            else {
                message_error = "Введите номер ячейки";
                continue;
            }
        }
        else {
            message_error = "Введите номер ячейки";
            continue;
        }
    }
}