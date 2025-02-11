#[derive(Debug, Clone, Copy)]
enum GarageState {
   Empty,
   Partial(u32),
   Full,
}

#[derive(Debug, Clone, Copy)]
enum Signal {
   CarIn,
   CarOut,
   None,
}

fn update(state: GarageState, signal: Signal) -> GarageState {
   const MAX_CARS: u32 = 6;
   
   match (state, signal) {
       (GarageState::Empty, Signal::CarIn) => {
           println!("Car Entered,\n cars in garage: 1");
           GarageState::Partial(1)
       }
       
       (GarageState::Partial(n), Signal::CarIn) => {
           if n == 0 {
               println!("Car Entered,\n cars in garage: 1");
               GarageState::Partial(1)
           } else if n < MAX_CARS - 1 {
               println!("Car Entered,\n cars in garage: {}", n + 1);
               GarageState::Partial(n + 1)
           } else if n == MAX_CARS - 1 {
               println!("Car Entered,\n cars in garage: {}", MAX_CARS);
               println!("Garage is full");
               GarageState::Full
           } else {
               println!("Invalid state");
               state
           }
       }
       
       (GarageState::Partial(1), Signal::CarOut) => {
           println!("Car left,\n cars in garage: 0");
           println!("Garage is empty");
           GarageState::Empty
       }
       
       (GarageState::Partial(n), Signal::CarOut) => {
           println!("Car left,\n cars in garage: {}", n - 1);
           GarageState::Partial(n - 1)
       }
       
       (GarageState::Full, Signal::CarOut) => {
           println!("Car left,\n cars in garage: {}", MAX_CARS - 1);
           GarageState::Partial(MAX_CARS - 1)
       }
       
       (state, Signal::None) => state,
       
       (GarageState::Full, Signal::CarIn) => {
           println!("Garage is full");
           GarageState::Full
       }
       
       (GarageState::Empty, Signal::CarOut) => {
           println!("Garage is empty");
           GarageState::Empty
       }
   }
}

fn main() {
   let mut state = GarageState::Empty;
   println!("Garage is Empty");
   
   for _ in 0..6 {
       state = update(state, Signal::CarIn);
   }
  
   for _ in 0..2 {
       state = update(state, Signal::CarOut);
   }
}
