use std::collections::VecDeque;
use nalgebra::Point2;
use nalgebra::Vector2;
use itertools::Itertools;
use time::Instant;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let width = 5;
    let height = 5;
    let mut start_snakes = Vec::new();

    let mut snake1 = Vec::<Point2<i32>>::new();
    snake1.push(Point2::new(1, 1));
    snake1.push(Point2::new(1, 1));
    snake1.push(Point2::new(1, 1));

    start_snakes.push(snake1);
    
    let mut snake2 = Vec::<Point2<i32>>::new();
    snake2.push(Point2::new(3, 3));
    snake2.push(Point2::new(3, 3));
    snake2.push(Point2::new(3, 3));

    start_snakes.push(snake2);

    let mut stack = VecDeque::<(i32, Vec<Vec<Point2<i32>>>)>::new();
    
    // let start_state = (0, start_snakes);
    // stack.push_front(start_state);
    
    let timer = Instant::now();

    let max_turn = 8;
    
    stack.append(&mut get_next_turn(1, start_snakes, width, height));
    // print_stack(stack.clone());

    let mut handles = Vec::<JoinHandle<()>>::new();

    for (key, group) in &stack.into_iter().group_by(|(turn, snakes)| snakes[0][0]) {
        // println!("{}", key);
        let group_stack = group.collect::<VecDeque::<(i32, Vec<Vec<Point2<i32>>>)>>();
        // print_stack(group_stack.clone());
        handles.push(thread::spawn(move || {
            run_stack(max_turn, group_stack, width, height);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Handle food
    // TODO: Handle head collision
    // TODO: Handle direction decision

    println!("{} seconds for predicting future to depth {}.", timer.elapsed().as_seconds_f32(), max_turn);

    // println!("stack contains {:#} unprocessed elements", stack.len());
}

fn run_stack(to_turn: i32, start_stack: VecDeque::<(i32, Vec<Vec<Point2<i32>>>)>, width: i32, height: i32) {
    let mut turn: i32;
    let mut total_processed = 0;
    let mut stack = start_stack.clone();
    
    while !stack.is_empty() {
        let (turn, snakes) = stack.pop_front().unwrap();
        total_processed += 1;

        if turn + 1 <= to_turn {
            stack.append(&mut get_next_turn(turn + 1, snakes, width, height));
            
            // print_stack(stack.clone());
        }
    }
    
    println!("processed {:#} elements", total_processed);
}

fn get_next_turn(turn: i32, snakes: Vec<Vec<Point2<i32>>>, width: i32, height: i32) -> VecDeque::<(i32, Vec<Vec<Point2<i32>>>)> {
    let bodies: Vec<Point2<i32>> = snakes.clone().into_iter().flatten().collect::<Vec<Point2<i32>>>();

    return snakes
    .iter()
    .map(|snake| get_possible_coords(*snake.first().unwrap(), width, height, bodies.to_vec()))
    .multi_cartesian_product()
    .map(|heads| heads
        .iter()
        .zip(snakes.iter())
        .map(|(new_head, body)| move_snake(new_head, body))
        .collect::<Vec<Vec<Point2<i32>>>>()
    )
    .map(|future_state| (turn, future_state))
    .collect::<VecDeque::<(i32, Vec<Vec<Point2<i32>>>)>>()
}

fn move_snake(new_head: &Point2<i32>, body: &Vec<Point2<i32>>) -> Vec<Point2<i32>> {
    let mut new_body = body.clone(); 
    new_body.truncate(new_body.len() - 1); 
    new_body.insert(0, *new_head); 
    return new_body;
}

fn print_stack(stack: VecDeque::<(i32, Vec<Vec<Point2<i32>>>)>) -> () {
    for (d, s) in stack.iter() {
        println!("{:#}", d);
        for snek in s {
            println!("{:?}", snek.iter().fold(String::new(), |acc, &coord| acc + &coord.to_string()));
        }
    } 
}

fn get_possible_coords(origin: Point2<i32>, width: i32, height: i32, bodies: Vec<Point2<i32>>) -> Vec<Point2<i32>> {
    return [
        origin + Vector2::new(0, 1),
        origin + Vector2::new(1, 0),
        origin + Vector2::new(0, -1),
        origin + Vector2::new(-1, 0)
    ]
    .iter()
    .cloned()
    .filter(|point| 
        point.coords[0] >= 0 && point.coords[0] < width && 
        point.coords[1] >= 0 && point.coords[1] < height &&
        ! bodies.contains(point)
    )
    .collect::<Vec<Point2<i32>>>();
}
