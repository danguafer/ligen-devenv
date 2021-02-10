use ligen::ligen;

pub struct Rectangle {
    width: i32,
    height: i32
}

#[ligen(UE4Blueprint(BlueprintCallable, BlueprintType))]
impl Rectangle {
    #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
    pub fn new(width: i32, height: i32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
    pub fn get_width() -> i32 { self.width }

    #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
    pub fn get_height() -> i32 { self.height }

    #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
    pub fn print_type(&self) {
        println!("Rectangle(width: {}, height: {})", self.width, self.height);
    }

    #[ligen(UE4Blueprint(BlueprintCallable, Category = "Rectangle"))]
    pub fn add(&self, rect: &Rectangle) -> Rectangle {
        Rectangle {
            width: self.width + rect.width,
            height: self.height + rect.height
        }
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Dropping Rectangle");
    }
}
