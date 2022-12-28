use std::io::BufRead;
use std::collections::VecDeque;

type Grid = Vec<Vec<usize>>;
type Coord = (usize,usize);
type Path = Vec<Coord>;
type Queue = VecDeque<Path>;

fn bfs_path(map: &Grid, root: Coord, dest: Coord) -> Option<Path> {
    let w = map[0].len(); let h = map.len();
    let mut q: Queue = VecDeque::new();
    q.push_back(vec![root]);
    let mut seen: Vec<Coord> = vec![root];
    while q.len() > 0 {
        let path = q.pop_front();
        let path = path.unwrap();
        let pos = path.last().unwrap();
        if *pos == dest {
            return Some(path);
        }
        let x = pos.0; let y = pos.1;
        for (x2,y2) in [(x+1, y), (x.saturating_sub(1), y), (x, y+1), (x, y.saturating_sub(1))] {
            if x2 < w && y2 < h && !seen.contains(&(x2,y2)) {
                if map[y2][x2] <= map[y][x] + 1 {
                    seen.push((x2,y2));
                    let mut new_path = path.clone();
                    new_path.push((x2,y2));
                    q.push_back(new_path);
                }
            }
        }
    }
    None
}

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    let mut dest = (0, 0);
    for line in lines {
        map.push(Vec::new());
        let index = map.len() - 1;
        for c in line.chars() {
            match c {
                'S' => {
                    map[index].push(0);
                    start = (map[index].len() - 1, index);
                }
                'E' => {
                    map[index].push(25);
                    dest = (map[index].len() - 1, index);
                }
                _ => map[index].push((c as usize) - ('a' as usize)),
            }
        }
    }
/*
    for row in &map {
        println!("{row:?}");
    }
    println!("{start:?} {dest:?}");
*/
    let answer = bfs_path(&map, start, dest);
    println!("Answer: {:?}",answer.unwrap().len()-1);
}
