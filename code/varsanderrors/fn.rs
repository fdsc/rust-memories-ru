﻿fn main()
{
	let a = 1;
	let b = 2;
	
	// Замыкания не требуют объявления типов параметров и возвращаемого значения
	let x = 
	||
	{
		println!("a=={}, b=={}", a, b);
	};

	let y =
	|b|
	{
		println!("a+{}=={}", b, a);
	};
	
	let z = || a+b;

	x();
	y(2);
	y(z());
}

// Как возвращать замыкания???
/*
fn create() -> Box<dyn Fn() -> ()>
{
	Box::new(|| 1 + 2)
}
*/
