#![allow(non_snake_case)]
use rand::{thread_rng, Rng};

mod algolism;
use algolism::keisan::*;

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    let index= use_ref(cx, Vec::new); //出題順番
    let x = use_ref(cx, Vec::<i32>::new);
    let y = use_ref(cx, Vec::<i32>::new);
    let ans = use_ref(cx, Vec::<i32>::new);
    let syo = use_ref(cx, Vec::<i32>::new);//余り
    let calctype =use_state(cx, || 0);//問題の種類 0:足し算,1:引き算,2:掛け算,3:割り算
    let question = use_ref(cx, String::new);
    let Questions = use_ref(cx, Vec::new);//全問題分
    let xmaxsize=9;//最大値
    let ymaxsize=9;//最大値
    let mut maxsize=xmaxsize*ymaxsize;

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h2{
                class: "m-4 text-4xl font-bold",
                "けいさんもんだいをえらんでね"
            }
            button { 
                class: "m-3 text-white bg-blue-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {
                index.with_mut(|index| *index=Vec::new());
                Questions.with_mut(|Questions| *Questions=Vec::new());
                Questions.with_mut(|Questions| Questions.append(&mut create_questions(xmaxsize,ymaxsize,0)));
                calctype.with_mut(|calctype|*calctype = 0);
                Questions.with_mut(|Questions| {
                    index.with_mut(|index| index.append(&mut create_index((Questions.len() as i32 ))));
                });
                //
                {count.with_mut(|count|{
                    let chiledindex =*count as usize;
                    index.with_mut(|index| {
                        let workindex = index[chiledindex] as usize;
                        question.with_mut(|question|Questions.with_mut(|Questions|{
                        *question=String::new();
                        let workquestion=&Questions[workindex].4;
                        *question =(&workquestion).to_string();
                           }))
                       })
                }
                )}
                //
            }
                , h4{"たしざん"} }
            button { 
                class: "m-3 text-white bg-blue-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {
                index.with_mut(|index| *index=Vec::new());
                Questions.with_mut(|Questions| *Questions=Vec::new());
                Questions.with_mut(|Questions| Questions.append(&mut create_questions(xmaxsize,ymaxsize,1)));
                calctype.with_mut(|calctype|*calctype = 1);
                Questions.with_mut(|Questions| {
                    index.with_mut(|index| index.append(&mut create_index((Questions.len() as i32 ))));
                });
                //
                {count.with_mut(|count|{
                    let chiledindex =*count as usize;
                    index.with_mut(|index| {
                        let workindex = index[chiledindex] as usize;
                        question.with_mut(|question|Questions.with_mut(|Questions|{
                        *question=String::new();
                        let workquestion=&Questions[workindex].4;
                        *question =(&workquestion).to_string();
                           }))
                       })
                }
                )}
                //
            }
                , h4{"ひきざん"} }
            button {
                class: "m-3 text-white bg-blue-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {
                index.with_mut(|index| *index=Vec::new());
                index.with_mut(|index| index.append(&mut create_index(maxsize)));
                Questions.with_mut(|Questions| *Questions=Vec::new());
                Questions.with_mut(|Questions| Questions.append(&mut create_questions(xmaxsize,ymaxsize,2)));
                calctype.with_mut(|calctype|*calctype = 2);
                //
                {count.with_mut(|count|{
                    let chiledindex =*count as usize;
                    index.with_mut(|index| {
                        let workindex = index[chiledindex] as usize;
                        question.with_mut(|question|Questions.with_mut(|Questions|{
                        *question=String::new();
                        let workquestion=&Questions[workindex].4;
                        *question =(&workquestion).to_string();
                           }))
                       })
                }
                )}
                //
            }
                , h4{"かけざん"} }
            button { 
                class: "m-3 text-white bg-blue-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {
                index.with_mut(|index| *index=Vec::new());
                index.with_mut(|index| index.append(&mut create_index(maxsize)));
                Questions.with_mut(|Questions| *Questions=Vec::new());
                Questions.with_mut(|Questions| Questions.append(&mut create_questions(xmaxsize,ymaxsize,3)));
                calctype.with_mut(|calctype|*calctype = 3);
                //
                {count.with_mut(|count|{
                    let chiledindex =*count as usize;
                    index.with_mut(|index| {
                        let workindex = index[chiledindex] as usize;
                        question.with_mut(|question|Questions.with_mut(|Questions|{
                        *question=String::new();
                        let workquestion=&Questions[workindex].4;
                        *question =(&workquestion).to_string();
                           }))
                       })
                }
                )}
                //
            }
                , h4{"わりざん"} }
        }


        div{
                style: "text-align: center;",
                h2 {
                    class: "m-4 text-4xl font-bold", 
                    "だい {count+1} もん" 
                }
                h1 {
                    class: "m-4 text-4xl font-bold",
                    "{question.read()}"
                }
        }


        div {
            style: "text-align: center;",
            button { 
                class: "m-3 text-white bg-green-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {count.with_mut(|count|{if count < &mut maxsize {
                    *count +=1;
                } else {
                    *count =maxsize;
                }
                //
                let chiledindex =*count as usize;
                index.with_mut(|index| {
                    let workindex = index[chiledindex] as usize;
                    question.with_mut(|question|Questions.with_mut(|Questions|{
                    *question=String::new();
                    let workquestion=&Questions[workindex].4;
                    *question =(&workquestion).to_string();
                       }))
                   })
                //
            }
            )}
            , h3{"つぎのもんだい"} }
            button { 
                class: "m-3 text-white bg-yellow-500 border-1 rounded py-1 px-4 focus:outline-none hover:bg-red-300",
                onclick: move |_| {count.with_mut(|count|{if count > &mut 0 {
                        *count -=1;
                    } else {
                       *count =0;
                    }
                //
                let chiledindex =*count as usize;
                index.with_mut(|index| {
                let workindex = index[chiledindex] as usize;
                question.with_mut(|question|Questions.with_mut(|Questions|{
                *question=String::new();
                let workquestion=&Questions[workindex].4;
                *question =(&workquestion).to_string();
                       }))
                  })
                //
                }
                )}
            , h3{"まえのもんだい"} }
        }
    ))
}
