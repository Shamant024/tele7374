mod module1;

mod module2;

fn main ()
{
	module1::module1();
	module2::module2();

	std::process::exit(0);
}
