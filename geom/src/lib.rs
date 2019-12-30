use ligen::ligen;

pub struct Rectangle {
    width: i32,
    height: i32
}

#[ligen(UE4Blueprint)]
impl Rectangle {
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    // pub fn get_vec(&self) -> Vec<i32> { Vec::new() }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }

    pub fn print_type() { println!("Rectangle"); }

    pub fn add(&self, rect: &Rectangle) -> Rectangle {
        Rectangle::new(self.get_width() + rect.get_width(), self.get_height() + rect.get_height())
    }
}


// #[ligen(UE4Blueprint(BlueprintCallable, BlueprintType))]
// impl Rectangle {
//     #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
//     pub fn new(width: i32, height: i32) -> Rectangle {
//         Rectangle {
//             width,
//             height
//         }
//     }
//
//     #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
//     pub fn digits() -> i32 { 987654321 }
//
//     #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
//     pub fn print(&self) {
//         println!("Rectangle(width: {}, height: {})", self.width, self.height);
//     }
//
//     #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
//     pub fn add_widths(&self, rect: &Rectangle) -> i32 {
//         self.width + rect.width
//     }
// }

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Dropping Rectangle");
    }
}
