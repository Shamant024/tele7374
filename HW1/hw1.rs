static A: u8 = 12;
static B: u16 = 1250;
static C: u32 = 123673;
static D: u64 = 372872674;
static E: u128 = 3618395768493827109;
static F: u8 = 25;

fn main()
{
	let a: u8 = 14;	
	let b: u16 = 324;
	let c: u32 = 244573;
	let d: u64 = 4729573829;
	let e: u128 = 37289457301938467;
	let f: u8 = 24;
	
	println!("Global Variables and their Address:");
    	println!("A = {}, Address = {:p}", A, &A);
    	println!("B = {}, Address = {:p}", B, &B);
	println!("C = {}, Address = {:p}", C, &C);
    	println!("D = {}, Address = {:p}", D, &D);
    	println!("E = {}, Address = {:p}", E, &E);
    	println!("F = {}, Address = {:p}", F, &F);

    	println!("\nLocal Variables and their Address:");
    	println!("a = {}, Address = {:p}", a, &a);
    	println!("b = {}, Address = {:p}", b, &b);
    	println!("c = {}, Address = {:p}", c, &c);
    	println!("d = {}, Address = {:p}", d, &d);
    	println!("e = {}, Address = {:p}", e, &e);
    	println!("f = {}, Address = {:p}", f, &f);
}
