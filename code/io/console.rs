﻿// 5BaC0do3JexEWC6jwB: decl
#![allow(non_snake_case)]

use std::io;

// https://doc.rust-lang.org/std/prelude/index.html - список того, что уже подключено без use

// Эту структуру мы будем выводить на экран с помощью println!("{:?}", s);
// #[derive(Debug)] говорит компилятору вставить информацию для вызова типажа форматирования {:?}
#[derive(Debug)]
struct Struct
{
	firstField:  u64,
	secondField: String
}

fn main()
{
	loop
	{
		// Это макрос для вывода строки на консоль. Для него не нужно use std::io
		println!("Введите число");

		// Создаём строку
		// let - создать переменную, mut - создать именно изменяемую переменную, а не константу
		let mut guess = String::new();

		// Ждёт ввода пользователя
		// Можно вызвать полным именем std::io::stdin
		// &mut берём адрес ссылки guess, причём mut - то есть его можно изменять.
		// Ввод пользователя будет вместе с \r\n, а под Linux - с символом \n
		io::stdin().read_line(&mut guess)
		.expect("Не смогли прочитать строку");	// Обрабатываем ошибку: если будет ошибка, программа будет аварийно завершена с этим сообщением


		// let len = guess.len() - 2;	// 2 символа - это \r\n
		// Вот здесь можно вызвать падение программы, введя ctrl+z (завершение ввода).
		// thread 'main' panicked at 'attempt to subtract with overflow'
		// Чтобы этого не было, нужно использовать функцию trim() - тогда пробелы и служебные символы будут обрезаны


		let len = guess.trim().len();
		println!("Вы ввели строку, длинной {} символов. Ваш ввод:\r\n{}", len, guess);
		// "{}" символ подстановки для значения по порядку
		// Обратить внимание, что если забыть \n, то символ \r только вернёт каретку назад, но не переведёт её на новую строку

		// Затеняем переменную
		let guess: u64 = 
		match guess.trim().parse()
		{
			Ok (num) => num,
			Err(err) =>
			{
				println!("Число не удалось распознать!\r\n{}", err);
				0;
				continue;
			}
		};

		println!("Вы ввели число {}", guess);
		break;
	}
	
	// Типаж форматирования Display - {}
	// Типаж форматирования Debug - {:?}
	let s = Struct
	{
		firstField:  1000,
		secondField: String::from("01000")
	};

	println!("{:?}", s);
	println!("{:#?}", s);
	/* Вывод обоих команд:
	Struct { firstField: 1000, secondField: "01000" }
	Struct {
		firstField: 1000,
		secondField: "01000",
	}
*/
}
