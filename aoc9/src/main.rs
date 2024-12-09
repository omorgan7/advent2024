#[derive(Clone, Copy, PartialEq)]
enum Block {
    Free,
    File(i64),
}

fn part1(mut disk: Vec<Block>, map: &[i64]) -> i64 {
    let mut disk_front_pointer: usize = 0;
    let mut i = 0;
    let mut last_id = -1;
    let mut j = (disk.len() - 1) as i64;

    loop {
        loop {
            if let Block::File(x) = disk[disk_front_pointer] {
                if x != last_id {
                    disk_front_pointer += map[i] as usize;
                    i += 2;
                    last_id = x;
                }
            }
            if disk[disk_front_pointer] == Block::Free {
                break;
            }
        }

        while disk[j as usize] == Block::Free {
            j -= 1;
        }

        if disk_front_pointer > j as usize {
            break;
        }

        if disk[j as usize] != Block::Free {
            disk[disk_front_pointer] = disk[j as usize];
            disk[j as usize] = Block::Free;
            j -= 1;
        }
        disk_front_pointer += 1;
    }

    disk.iter()
        .enumerate()
        .map(|(i, x)| match x {
            Block::File(y) => y * i as i64,
            _ => 0,
        })
        .sum::<i64>()
}

fn part2(mut disk: Vec<Block>, map: &[i64]) -> i64 {
    let mut backfilled = vec![0; map.len()];

    for j in (0..map.len()).step_by(2).rev() {
        for i in 0..j {
            if i % 2 == 1 && (map[i] - backfilled[i]) >= map[j] {
                let offset = map[..i].iter().sum::<i64>() + backfilled[i];

                backfilled[i] += map[j];
                backfilled[i - 1] += map[j];

                let end_offset = map[..=j].iter().sum::<i64>();
                for k in 0..map[j] {
                    disk[(offset + k) as usize] = disk[(end_offset - k - 1) as usize];
                    disk[(end_offset - k - 1) as usize] = Block::Free;
                }

                break;
            }
        }
    }
    disk.iter()
        .enumerate()
        .map(|(i, x)| match x {
            Block::File(y) => y * i as i64,
            _ => 0,
        })
        .sum::<i64>()
}

fn dense_to_sparse(map: &[i64]) -> Vec<Block> {
    let mut disk = Vec::<Block>::new();

    let mut id = 0;
    for i in 0..map.len() {
        for _ in 0..map[i] {
            disk.push(if i % 2 == 1 {
                Block::Free
            } else {
                Block::File(id)
            });
        }
        if i % 2 == 0 {
            id += 1;
        }
    }

    disk
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();

    let disk = dense_to_sparse(&input);
    println!("{}", part1(disk.clone(), &input));
    println!("{}", part2(disk.clone(), &input));
}
