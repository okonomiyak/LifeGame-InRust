use std::{thread, time::Duration};
use proconio::input;

const MAPSIZE:usize = 10;

const DIRS:[(isize,isize);8]=[
    (-1,-1),(-1,0),(-1,1),
    (0,-1),      (0,1),
    (1,-1),(1,0),(1,1)
];

fn main() {
    let mut countup = 0;
    let mut map = start();
    loop{
        view(&map);
        println!("400 400 -> 実行 x,yの形で入力 -> マスの色変え");
        input! {i:usize,j:usize};
        if i == 400 && j == 400{
            break;
        }else {
            if i >= MAPSIZE || j >= MAPSIZE {
                println!("有効な数値を入力してください!");
                println!("現在のマップサイズは{MAPSIZE}x{MAPSIZE}です。")
            }else {
                map[j][i] = !map[j][i];
            }
        }
    }
    loop{
        thread::sleep(Duration::from_millis(500));
        countup+=1;
        println!("Now {countup}");
        let next = check_edit(map.clone());
        view(&next);
        map = next;
    }
    
}
fn view(map: &[Vec<bool>]){
    for i in 0..MAPSIZE{
        print!("{i} ");
    }
    println!("");
    for (i,m) in map.iter().enumerate(){
        for j in m{
            if *j{
                print!("◼ ");
            }
            else {
                print!("◻ ");
            }
        }
        println!("{}",i);
    }
}
fn start() -> Vec<Vec<bool>>{
    vec![vec![false;MAPSIZE];MAPSIZE]
}
fn check_edit(map: Vec<Vec<bool>>)->Vec<Vec<bool>>{
    let mut next_map = map.clone();
    for i in 0..MAPSIZE{
        for j in 0..MAPSIZE{
            let live = count_dirs(&map, i, j);
            if map[i][j] {
                if live == 2 || live == 3 {
                }else{
                    next_map[i][j] = false;
                }
            }else{
                if live == 3 {
                    next_map[i][j] = true;
                }
            }
        }
    }
    next_map
}
fn count_dirs(map: &Vec<Vec<bool>>,x:usize,y:usize) -> usize{
    let mut count = 0;
    for (dx,dy) in DIRS{
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0|| ny < 0 || nx >= MAPSIZE as isize || ny >= MAPSIZE as isize {
            continue;
        }
        if map[nx as usize][ny as usize]{
            count += 1;
        }
    }
    count
}