#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum State{
    Strike,
    Spare,
    Open,
}

pub struct Frame{
    first : u16,
    second : Option<u16>,
    third : Option<u16>,
    state : State,
    ten : bool,
}

impl Frame{
    pub fn new(is_ten : bool) -> Self{
        Frame{
            first : 0,
            second : None,
            third : None,
            state : State::Open,
            ten : is_ten,
        }
    }

    pub fn is_finish(&mut self) -> bool {
        //열번쨰 프레임인 경우
        if self.ten {
            println!("열번쨰 프레임 입니다");
            //두번째 안친경우
            if self.second.is_none() {
                false
            }
            //첫번째 두번째 10개 다치고 3번째 안친경우
            else if self.second.unwrap() + self.first >= 10 && self.third.is_none() {
                false
            }
            else {
                true
            }
        }
        //열번째 이외의 프레임
        else {
            //스트라이크 친경우
            if self.first == 10 && self.second.is_none() {
                true
            }
            //두번째거 안친경우
            else if self.second.is_none(){
                false
            }
            else {
                true
            }
        }
    }
}

pub struct BowlingGame {
    now_frame : usize,
    frame_list : Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            now_frame : 1,
            frame_list : Vec::new()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        //10 프레임 에러 검출
        if self.now_frame == 10 && self.frame_list[self.now_frame - 1].is_finish(){
            println!("10프레임 다침");
            return Err(Error::GameComplete)
        }
        // 비정상적인 입력 검출
        // 10개 넘는 핀 쳤을 경우
        if pins > 10{
            return Err(Error::NotEnoughPinsLeft);
        }
        let mut left_pins : u16 = 10;
        // 1번째 프레임이 아닌 경우
        if self.frame_list.len() != 0{
            //10번째 프레임 첫번째가 아닌 경우
            if  self.frame_list.len() == 10{
                //첫번째 친거 빼기
                left_pins -= &self.frame_list[self.now_frame - 1].first;

                //만약 더 없으면 핀 보충
                if left_pins == 0 {
                    left_pins = 10;
                }

                // 두번째 친거 있다면 빼기
                if self.frame_list[self.now_frame - 1].second.is_some() {
                    left_pins -= &self.frame_list[self.now_frame - 1].second.unwrap();
                }

                //만약 더 없으면 핀 보충
                if left_pins == 0 {
                    left_pins = 10;
                }
                

                if left_pins < pins && left_pins != 10{
                    return Err(Error::NotEnoughPinsLeft)
                }
            }
            // 이외
            else {
                if  &self.frame_list[self.now_frame - 1].first + pins > 10 && !self.frame_list[self.now_frame - 1].is_finish() {
                    return Err(Error::NotEnoughPinsLeft)
                }
            }
        }
        //오류 검출 이후

        // 첫 프레임
        if self.frame_list.len() == 0{
            let mut frame = Frame::new(false);
            if pins == 10{
                frame.state = State::Strike;
            }
            frame.first = pins;
            self.frame_list.push(frame);
            return Ok(())
        }
        // 10프레임 시작
        else if self.frame_list.len() == 9 && self.frame_list[self.now_frame - 1].is_finish() {
            self.now_frame = self.now_frame + 1;
            
            let mut frame = Frame::new(true);
            if pins == 10{
                frame.state = State::Strike;
            }
            frame.first = pins;
            self.frame_list.push(frame);
            return Ok(())            
        }
        // 10프레임 도중
        else if self.frame_list.len() == 10 && !self.frame_list[self.now_frame - 1].is_finish(){
            //10프레임 세번째
            if self.frame_list[self.now_frame - 1].second.is_some() {
                self.frame_list[self.now_frame - 1].third = Some(pins);
                return Ok(())
            }
            //10프레임 두번째
            else{
                if pins + self.frame_list[self.now_frame - 1].first == 10{
                    self.frame_list[self.now_frame - 1].state = State::Spare;
                }
                if pins == 10{
                    self.frame_list[self.now_frame - 1].state = State::Strike;
                }
                self.frame_list[self.now_frame - 1].second = Some(pins);
                return Ok(())
            }
        }
        // 이외 프레임 시작
        // 이전 프레임 다친 경우
        else if self.frame_list[self.now_frame - 1].is_finish() {
            self.now_frame = self.now_frame + 1;
            
            let mut frame = Frame::new(false);
            if pins == 10{
                frame.state = State::Strike;
            }
            frame.first = pins;
            self.frame_list.push(frame);
            return Ok(())
        }
        //한프레임 두번째
        else {
            if pins + self.frame_list[self.now_frame - 1].first == 10{
                self.frame_list[self.now_frame - 1].state = State::Spare;
            }
            self.frame_list[self.now_frame - 1].second = Some(pins);
            return Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        //10개보다 덜친경우
        if self.frame_list.len() < 10{
            return None
        }
        let mut score : u16 = 0;
        let mut frame_count = 0;
        for frame in &self.frame_list{
            if frame.ten{
                //2번째가 없는경우
                if frame.second.is_none(){
                    return None
                }
                // 2번째에 스페어나 스트라이크 쳤는데 3번째가 없는경우
                if frame.third.is_none() && (frame.second.unwrap() + frame.first == 10 || frame.second.unwrap() + frame.first == 20) {
                    return None
                }
                if frame.third.is_none(){
                    score = score + frame.first + frame.second.unwrap();
                }
                else {
                    score = score + frame.first + frame.second.unwrap() + frame.third.unwrap();
                }
            }
            else {
                let next_frame = &self.frame_list[frame_count + 1];
                match frame.state {
                    State::Open => {
                        if frame.second.is_none(){
                            return None
                        }
                        score = score + frame.first + frame.second.unwrap();
                    }
                    State::Spare => {
                        score = score + frame.first + frame.second.unwrap() + next_frame.first;
                    }
                    State::Strike => {
                        if next_frame.second.is_none() {
                            score = score + frame.first + next_frame.first + &self.frame_list[frame_count + 2].first;
                        }
                        else{
                            score = score + frame.first + next_frame.first + next_frame.second.unwrap();
                        }
                    }
                }
            }
            frame_count += 1;
        }
        return Some(score);
    }
}

#[test]
fn roll_returns_a_result() {
    let mut game = BowlingGame::new();
    assert!(game.roll(0).is_ok());
}

#[test]
fn you_cannot_roll_more_than_ten_pins_in_a_single_roll() {
    let mut game = BowlingGame::new();

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn a_game_score_is_some_if_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert!(game.score().is_some());
}

#[test]
fn you_cannot_score_a_game_with_no_rolls() {
    let game = BowlingGame::new();

    assert_eq!(game.score(), None);
}

#[test]
fn a_game_score_is_none_if_fewer_than_ten_frames_have_been_rolled() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), None);
}

#[test]
fn a_roll_is_err_if_the_game_is_done() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }

    assert_eq!(game.roll(0), Err(Error::GameComplete));
}

#[test]
fn twenty_zero_pin_rolls_scores_zero() {
    let mut game = BowlingGame::new();

    for _ in 0..20 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(0));
}

#[test]
fn ten_frames_without_a_strike_or_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(3);
        let _ = game.roll(6);
    }

    assert_eq!(game.score(), Some(90));
}

#[test]
fn spare_in_the_first_frame_followed_by_zeros() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
fn points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(6);
    let _ = game.roll(4);
    let _ = game.roll(3);

    for _ in 0..17 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(16));
}

#[test]
fn consecutive_spares_each_get_a_one_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(3);
    let _ = game.roll(7);
    let _ = game.roll(4);

    for _ in 0..15 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(31));
}

#[test]
fn if_the_last_frame_is_a_spare_you_get_one_extra_roll_that_is_scored_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);
    let _ = game.roll(7);

    assert_eq!(game.score(), Some(17));
}

#[test]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    for _ in 0..18 {
        let _ = game.roll(0);
    }
    assert_eq!(game.score(), Some(10));
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..16 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(26));
}

#[test]
fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(5);
    let _ = game.roll(3);

    for _ in 0..12 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(81));
}

#[test]
fn a_strike_in_the_last_frame_earns_a_two_roll_bonus_that_is_counted_once() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(18));
}

#[test]
fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(7);
    let _ = game.roll(3);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn strikes_with_the_two_roll_bonus_do_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(30));
}

#[test]
fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(7);
    let _ = game.roll(3);
    let _ = game.roll(10);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn all_strikes_is_a_perfect_score_of_300() {
    let mut game = BowlingGame::new();

    for _ in 0..12 {
        let _ = game.roll(10);
    }

    assert_eq!(game.score(), Some(300));
}

#[test]
fn you_cannot_roll_more_than_ten_pins_in_a_single_frame() {
    let mut game = BowlingGame::new();

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn first_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(5).is_ok());
    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_can_be_a_strike_and_non_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert!(game.roll(6).is_ok());
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_be_a_non_strike_followed_by_a_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(6).is_ok());
    assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn second_bonus_ball_after_a_final_strike_cannot_score_an_invalid_number_of_pins_even_if_first_is_strike(
) {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());
    assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn if_the_last_frame_is_a_strike_you_cannot_score_before_the_extra_rolls_are_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
fn if_the_last_frame_is_a_spare_you_cannot_create_a_score_before_extra_roll_is_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(5);
    let _ = game.roll(5);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
fn cannot_roll_after_bonus_roll_for_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(7);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
fn cannot_roll_after_bonus_roll_for_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..9 {
        let _ = game.roll(0);
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(3);
    assert!(game.roll(2).is_ok());

    assert_eq!(game.roll(2), Err(Error::GameComplete));
}

#[test]
fn last_two_strikes_followed_by_only_last_bonus_with_non_strike_points() {
    let mut game = BowlingGame::new();
    for _ in 0..16 {
        let _ = game.roll(0);
    }
    let _ = game.roll(10);
    let _ = game.roll(10);
    let _ = game.roll(0);
    let _ = game.roll(1);

    assert_eq!(game.score(), Some(31));
}
