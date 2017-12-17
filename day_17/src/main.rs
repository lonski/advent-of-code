fn main() {
    let jump = 370;

    //Part one
    let mut vortex: Vec<usize> = vec![0];
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + jump) % i + 1;
        vortex.insert(pos, i);
    }
    for (i, v) in vortex.iter().enumerate() {
        if *v == 2017 {
            println!("Part one: {}", vortex[i + 1]);
            break;
        }
    }

    //Part 2
    let mut pos = 0;
    let mut val = 0;
    for i in 1..50000000 {
        pos = (pos + jump) % i + 1;
        if pos == 1 {
            val = i;
        }
    }
    println!("Part two: {}", val);

}
