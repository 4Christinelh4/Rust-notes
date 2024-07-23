# My notes when learning COMP6991 @UNSW

## Lifetimes
| Type | Requirements | Access |
|:----:|:---------:|:-----:|
|T (String) |Exactly one owner (Contiguous, dynamic length) |R&W (Owned) |
|&T (&str) |Shared borrow (Shared borrow, a string slice) |Read only |
|&mut T (&mut str) |Exclusive borrow (Exclusive borrow of the slice, can't extend or shrink) |R&W |

