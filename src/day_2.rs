pub fn day_2(data: Vec<String>) {
    let result = data.iter().fold((0 , 0, 0), parse_instuction);

    println!("Day 2 result: {}", result.0 * result.1)
}

fn parse_instuction(prev_pos: (i32, i32, i32), instruction: &String) -> (i32, i32, i32) {
    let (h_pos, depth, aim) = prev_pos;
    let mut split = instruction.split(" ");

    let instruction = split.next().unwrap();
    let arg: i32 = split.next().unwrap().parse().unwrap();

    match instruction {
        "forward" => (h_pos + arg, depth + arg * aim, aim),
        "down" => (h_pos, depth, aim + arg),
        "up" => (h_pos, depth, aim - arg),
        _ => panic!()
    }
}