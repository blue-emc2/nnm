use std::io::{stdin, stdout, Write};

pub trait Prompt {
    fn exec_delete_link(&self, url: &str);

    fn delete_prompt(&self, urls: &mut Vec<String>) {
        println!("削除したいURLまたは番号を入力してください。");
        println!("q, quit, exit で終了します。");
        let link_itretor = urls.iter().enumerate();
        for (i, url) in link_itretor {
            println!("{}: {}", i, url);
        }

        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "q" || input == "quit" || input == "exit" {
                break;
            }

            println!("入力された内容: {}", input);

            if let Ok(index) = input.parse::<usize>() {
                if index < urls.len() {
                    let url = &urls[index];
                    self.exec_delete_link(url);
                    break;
                } else {
                    println!("無効な番号です。もう一度入力してください。");
                }
            } else {
                // 入力がURLの場合
                if urls.contains(&input.to_string()) {
                    self.exec_delete_link(input);
                    break;
                } else {
                    println!("URLが見つかりません。もう一度入力してください。");
                }
            }
        }
    }
}
