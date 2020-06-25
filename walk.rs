
// https://blog.rust-lang.org/2015/05/11/traits.html

pub trait Walk {

    fn walk( &self ) -> u32;


}


struct Human {

       legs: u32,

}

struct Lion {

       legs: u32,


}


impl Walk for Lion {

    fn walk ( &self) -> u32 {

      15

    }

}


impl Walk for Human {


    fn walk ( &self) -> u32 {

       3 
    }

}


fn print_walk<T : Walk> (t: &T ) {

    println!("This specie walked {}", t.walk() )


}



fn main() {

    let h = Human{ legs: 2 };

     print_walk(&h);

    

    let l = Lion { legs : 4 };

    print_walk(&l);
    

}