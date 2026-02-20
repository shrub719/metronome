use crate::{
    eadk::storage::*,
    constants::file::HIGH_SCORE_FILE
};
calc_use!(alloc::format);
calc_use!(alloc::vec::Vec);

// this file is so full of repetition

pub fn load_high_score(id: &str) -> u32 {
    let binary = match file_read(HIGH_SCORE_FILE) {
        Some(vec) => vec,
        None => return 0
    };

    let content = str::from_utf8(&binary).unwrap();

    for line in content.split('\n') {
        if line.is_empty() { continue; }

        let mut parts = line.split(':');
        let line_id = parts.next().unwrap();
        if line_id == id {
            let score: u32 = parts.next().unwrap().parse().unwrap();
            return score
        }
    }

    0
}

pub fn write_high_score(id: &str, score: u32) {
    match file_read(HIGH_SCORE_FILE) {
        Some(binary) => {
            let content = str::from_utf8(&binary).unwrap();
            let mut new_binary = Vec::<u8>::with_capacity(20);
            new_binary.push(b'\n');

            let mut found = false;
            for line in content.split('\n') {
                if line.is_empty() { continue; }

                let mut parts = line.split(':');
                let line_id = parts.next().unwrap();

                if line_id == id {
                    let new_line = &format!("{}:{}\n", id, score);
                    new_binary.extend_from_slice(new_line.as_bytes());
                    found = true;
                } else {
                    let new_line = &format!("{}\n", line);
                    new_binary.extend_from_slice(new_line.as_bytes());
                }
            }

            if !found {
                let new_line = &format!("{}:{}\n", id, score);
                new_binary.extend_from_slice(new_line.as_bytes());
            }

            let _ = file_erase(HIGH_SCORE_FILE);
            let _ = file_write(HIGH_SCORE_FILE, &new_binary);
        },
        None => {
            let line = &format!("\n{}:{}\n", id, score);
            let _ = file_write(HIGH_SCORE_FILE, line.as_bytes());
        }
    }
}
