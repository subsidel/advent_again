#[cfg(test)]
mod tests {
    use ::*;
    const EXAMPLE: &str = "0
3
0
1
-3";
    #[test]
    fn escapes_when_not_panicy() {
        let mut bunny = RoboBunny::new(EXAMPLE);
        assert_eq!(bunny.debug_escape(true), 5);
    }

    #[test]
    fn escapes_when_running_scared() {
        let mut bunny = RoboBunny::new(EXAMPLE);
        bunny.see_a_big_nasty_fox_wolf();
        assert_eq!(bunny.debug_escape(true), 10);
        assert_eq!(bunny.steps, vec![2, 3, 2, 3, -1])
    }
}

pub struct RoboBunny {
    in_danger: bool,
    steps: Vec<i32>,
    current_position: i32,
    current_hops: i32,
}

impl RoboBunny {
    pub fn new(map: &str) -> RoboBunny {
        use std::str::FromStr;
        let steps = map.split_whitespace()
            .map(|s_num| i32::from_str(s_num).unwrap())
            .collect();

        RoboBunny {
            steps,
            current_position: 0,
            current_hops: 0,
            in_danger: false,
        }
    }

    pub fn see_a_big_nasty_fox_wolf(&mut self) {
        self.in_danger = true
    }

    pub fn escape(&mut self) -> i32 {
        self.debug_escape(false)
    }

    pub fn debug_escape(&mut self, print_path: bool) -> i32 {
        loop {
            if print_path {
                println!("{:?}", self);
            }

            let next_instruction = self.steps[self.current_position as usize];
            self.current_hops += 1;
            if self.in_danger && next_instruction >= 3 {
                self.steps[self.current_position as usize] -= 1;
            } else {
                self.steps[self.current_position as usize] += 1;
            }
            let next_position = self.current_position + next_instruction;
            if next_position >= self.steps.len() as i32 {
                break;
            }
            if next_position < 0 {
                break;
            }
            self.current_position = next_position;
        }

        self.current_hops as i32
    }
}

use std::fmt;
impl fmt::Debug for RoboBunny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pretty_map: Vec<String> = Vec::new();
        for n in 0..self.steps.len() {
            if n == self.current_position as usize {
                pretty_map.push(format!("({})", self.steps[n]))
            } else {
                pretty_map.push(format!("{}", self.steps[n]))
            }
        }
        write!(
            f,
            "RoboBunny {{ map: {:?}, current_hops: {}, current_position: {} }}",
            pretty_map,
            self.current_hops,
            self.current_position
        )
    }
}
