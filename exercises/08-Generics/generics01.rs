struct MyGenericStruct {
        // TODO: Make the struct generic over two types
}

impl MyGenericStruct {
        // TODO: Implement getters for references to x & y
}

// DO NOT TOUCH MAIN - solve only by changing the struct and its implementation
fn main() {
        let my_struct = MyGenericStruct { x: 5, y: 'a' };
        assert_eq!(my_struct.x(), &5);
        assert_eq!(&my_struct.x, my_struct.x());
        assert_eq!(my_struct.y(), &'a');
        assert_eq!(&my_struct.y, my_struct.y());

        let my_other_struct = MyGenericStruct { x: "hello".to_string(), y: 5.0f32 };
        assert_eq!(my_other_struct.x(), "hello");
        assert_eq!(&my_struct.x, my_struct.x());
        assert_eq!(my_other_struct.y(), &5.0); 
        assert_eq!(&my_struct.y, my_struct.y());
}



