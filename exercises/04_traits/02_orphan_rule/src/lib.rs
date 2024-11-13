// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/*
        error[E0117]: only traits defined in the current crate can be implemented for primitive types
         --> exercises\04_traits\02_orphan_rule\src\lib.rs:7:1
          |
        7 | impl PartialEq for u32 {
          | ^^^^^---------^^^^^---
          | |    |             |
          | |    |             `u32` is not defined in the current crate
          | |    `u32` is not defined in the current crate
          | impl doesn't use only types from inside the current crate
          |
          = note: define and implement a trait or new type instead

        For more information about this error, try `rustc --explain E0117`.
        error: could not compile `orphan` (lib test) due to 1 previous error
        error: could not compile `orphan` (lib) due to 1 previous error

        트레이트를 구현할 때는 아래 둘 중 하나는 반드시 "내가 만든 것"이어야 함
            트레이트가 내 크레이트에서 만든 것이거나
            구현하려는 타입이 내 크레이트에서 만든 것이어야 함

        // PartialEq - 표준 라이브러리(std)에서 정의된 트레이트 (= 남의 것)
        // u32 - 표준 라이브러리(std)에서 정의된 타입 (= 남의 것)

        // "남의 것" = 다른 크레이트(특히 std 등)에서 정의된 것들
        - String, u32, Vec 등의 타입들
        - PartialEq, Display 등의 트레이트들

        // "내 것" = 현재 크레이트에서 내가 정의한 것들
        struct MyType {}  // ✅ 내가 만든 타입
        trait MyTrait {} // ✅ 내가 만든 트레이트
*/

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }
