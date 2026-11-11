use rand::RngExt;
use std::collections::HashMap;

type Board = [i8; 9];
type QLearningTable = HashMap<(Board, usize), f32>;

const GAMES: i32 = 100000;
const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];
const ALPHA: f32 = 0.03;
const GAMMA: f32 = 0.6;

fn display_board(b: &Board) {
    for i in 0..3 {
        for j in 0..3 {
            match b[i * 3 + j] {
                1 => {
                    print!("o");
                }
                -1 => {
                    print!("x");
                }
                _ => {
                    print!("=");
                }
            }
        }
        println!("");
    }
}

fn get_valid_set(b: &Board) -> Vec<usize> {
    (0..9usize).filter(|&index| b[index] == 0).collect()
}

fn set(b: &Board, i: usize) -> Board {
    let mut setted: Board = [0; 9];
    setted.copy_from_slice(b);
    setted[i] = 1;
    setted
}

fn get_action(b: &Board, t: &QLearningTable, e: u32) -> usize {
    let mut rng = rand::rng();
    let explore = rng.random_ratio(e, 100);
    let valid_moves = get_valid_set(b);

    if !explore {
        let q = |a: usize| t.get(&(*b, a)).copied().unwrap_or(0.0);
        let best_value = max_q(&t, &b);
        let best_moves: Vec<usize> = valid_moves
            .into_iter()
            .filter(|&x| q(x) == best_value)
            .collect();
        best_moves[rng.random_range(0..best_moves.len())]
    } else {
        return valid_moves[rng.random_range(0..valid_moves.len())];
    }
}

fn wins(b: &Board) -> bool {
    (0..8usize).any(|index| b[LINES[index][0]] + b[LINES[index][1]] + b[LINES[index][2]] == 3)
}

fn filled(b: &Board) -> bool {
    (0..9usize).all(|index| b[index] != 0)
}

fn flip(b: &Board) -> Board {
    b.map(|x| -x)
}

fn max_q(q: &HashMap<(Board, usize), f32>, b: &Board) -> f32 {
    get_valid_set(b)
        .iter()
        .map(|&a| *q.get(&(*b, a)).unwrap_or(&0.0))
        .fold(f32::NEG_INFINITY, f32::max)
}

fn main() {
    let mut q_learn_table: QLearningTable = HashMap::new();

    let mut games_count: i32 = 0;

    // let mut board: Board = [0; 9];

    loop {
        games_count += 1;

        let mut board: Board = [0; 9];

        let won = loop {
            let a = get_action(&board, &q_learn_table, 80);

            let next = set(&board, a);
            let is_win = wins(&next);
            let is_draw = filled(&next);

            let target: f32 = if is_win {
                1.0
            } else if is_draw {
                0.0
            } else {
                -GAMMA * max_q(&q_learn_table, &flip(&next))
            };

            let q_s_a = q_learn_table.entry((board, a)).or_insert(0.0);
            *q_s_a += ALPHA * (target - *q_s_a);

            if is_win || is_draw {
                board = next;
                break is_win;
            }

            board = flip(&next);
        };

        if games_count % 10000 == 0 {
            println!("GAME: {}", games_count);
            display_board(&board);
            if won {
                println!("End with winner")
            } else {
                println!("End with no winner")
            }
        }

        if games_count >= GAMES {
            println!("reached limit");
            return;
        }
    }
}
