#![allow(non_snake_case)]
//extern crate rand;

pub mod keisan {
    use rand::{thread_rng, Rng};
    use rand::prelude::*;

    pub fn create_index(maxsize:i32)->Vec<i32> {
        let mut rng = rand::thread_rng();// デフォルトの乱数生成器を初期化します
        let mut nums: Vec<i32> = (1..=maxsize).collect();
        nums.shuffle(&mut rng);

        return nums;
    }

    pub fn create_questions(x:i32,y:i32,calctype:i32)->Vec<(i32, i32,i32,i32,String)>{
        let mut questions=Vec::new();
        let mut ans=0;
        let mut syo=0;//余り
        let mut pushflag=true;
        let mut question=String::new();

        for x1 in 0..x {
            for y1 in 0..y {
                question=String::new();
                pushflag=true;

                if calctype ==1 { //引き算
                    ans=x1-y1;
                    question=format!("{}-{}=",x1,y1);
                    if ans <0{
                        pushflag=false;
                    }
                } else if calctype ==2{ //掛け算
                    ans=x1*y1;
                    question=format!("{}×{}=",x1,y1);
                } else if calctype ==3{ //割り算
                    ans=x1/y1;
                    syo=x1%y1;
                    question=format!("{}/{}=",x1,y1);
                } else {
                    ans=x1+y1;
                    question=format!("{}+{}=",x1,y1);
                    if ans >= 10{
                        pushflag=false;
                    }
                }

                if pushflag{
                    questions.push((x1,y1,ans,syo,question));
                }
                
            }
        }

        return questions;
    }
}