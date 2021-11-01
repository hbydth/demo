use std::{f64::consts::PI, sync::Arc};

//use std::{fmt::Result, io::repeat};

//作业红绿灯
enum TrafficLight{
    Red ,
    Yellow ,
    Green,
}
impl TrafficLight{
    fn light_time(&self)->u8{
            match *self{
                TrafficLight::Red=>60,
                TrafficLight::Green=>50,
                TrafficLight::Yellow=>6
            }
       
    }

}
//作业二
fn plus_one(x:&[u32])-> Option<u32>{
    //let len=x.len();
    let mut y=0;
    for i in 0..x.len(){
        y=x[i] as u64 + y;
    }
    let max32 = std::u32::MAX as u64;
    if y > max32{
        return None;
    }

    let result:u32=0;
    result.checked_add(x.iter().sum())
}
//作业三
pub trait Area{
    fn calcu_area(&self) -> f64;
    fn getname(&self)-> String;
}


#[derive(Debug)]
struct Rectangle{
    width:f64,
    length:f64,
}
impl Area for Rectangle{
    fn calcu_area(&self)->f64{
        self.width * self.length
    }
    fn getname(&self)->String{
        return String::from("Rectangle");
    }
}

struct Round{
    radius:f64,
}
impl Area for Round{
    fn calcu_area(&self)->f64{
        self.radius * self.radius * PI
    }
    fn getname(&self)->String{
        return String::from("Round");
    }
}
struct  Triangle{
    bottom:f64,
    high:f64,
}
impl Area for Triangle{
    fn calcu_area(&self)->f64{
        (self.bottom * self.high) / 2.0
    }
    fn getname(&self)->String{
        return String::from("Triangle");
    }
}
pub fn print_area<T:Area>(item:T){
    print!("{} aras is:{}\n",item.getname(),item.calcu_area());

}
fn main() {
    //作业红绿灯
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    print!("TrafficLight is red,time is :{} \n",red.light_time());
    print!("TrafficLight is green,time is :{} \n",green.light_time());
    print!("TrafficLight is yellow,time is :{} \n",yellow.light_time());
    //作业求和
    let arr:[u32;2] = [1,3];
    let result:Option<u32> = plus_one(&arr);
    print!("sum result is :{:?}\n",result);
    let arr:[u32;2] = [std::u32::MAX,1];
    let result:Option<u32> = plus_one(&arr);
    print!("u32 max sum result is :{:?}\n",result);
    //作业面积
    let rect=Rectangle{
        width:30.0,
        length:50.0,
    };
    let round=Round{
        radius:20.0,
    };
    let triangle=Triangle{
        bottom:20.0,
        high:10.0,
    };

    print_area(rect);
    print_area(round);
    print_area(triangle);
    //print!("area={}",area(&rect));
    //print!("area={}\n",rect.area());
    //print!("{:#?}\n",rect)

}

//fn area(rect:&Rectangle)-> u32{
 //   rect.width * rect.length
//}