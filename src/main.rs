const MAPSIZE:usize = 10;
use tokio::time::{interval, Duration};
use proconio::input;

#[tokio::main]
async fn main() {
    let mut map = start();
    loop{
        view(&map);
        println!("モード選択 i = 114514 -> 実行 それ以外の数字 -> マスの色変え");
        input! {i:usize,j:usize};
        if i == 114514{
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
    let mut tick = interval(Duration::from_secs(1));
    loop{
        tick.tick().await;
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
                print!("■ ");
            }
            else {
                print!("□ ");
            }
        }
        println!("{}",i);
    }
}
fn start() -> Vec<Vec<bool>>{
    vec![vec![false;MAPSIZE];MAPSIZE]
    // let mut map:Vec<Vec<bool>> = Vec::new();
    // for _ in 0..MAPSIZE{
    //     let mut j_map = Vec::new();
    //     for _ in 0..MAPSIZE{
    //         j_map.push(false);
    //     }
    //     map.push(j_map);
    // }
    // map
}
fn check_edit(map: Vec<Vec<bool>>)->Vec<Vec<bool>>{
    let max_size = MAPSIZE-1;
    let mut next_map = map.clone();
    for i in 0..MAPSIZE{
        for j in 0..MAPSIZE{
            let mut bools = Vec::new();
            //i に変更           
            if i != max_size{bools.push(map[i+1][j])};
            if i != 0{bools.push(map[i-1][j])};
            //jに変更
            if j != max_size{bools.push(map[i][j+1])};
            if j != 0{bools.push(map[i][j-1])};
            //複合(同じ)
            if i != max_size && j != max_size{bools.push(map[i+1][j+1])};
            if i != 0 && j != 0{bools.push(map[i-1][j-1])};
            //複合(ズレ)
            if i != max_size && j != 0{bools.push(map[i+1][j-1])};
            if i != 0 && j != max_size{bools.push(map[i-1][j+1])};

            let live = bool_check(bools);
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

fn bool_check(b:Vec<bool>) -> usize{
    b.iter().filter(|&&x| x).count()
    // let mut i = 0;
    // for bs in b{
    //     if bs {i += 1}
    // }
    // i
}