pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;


    /*
    
    The pointer to the heap region you reserved.
    The length of the string, i.e. how many bytes are in the string.
    The capacity of the string, i.e. how many bytes have been reserved on the heap.
    |ptr|len|cap|
     */
    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 8 * 3);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 24 * 3);
    }
}
