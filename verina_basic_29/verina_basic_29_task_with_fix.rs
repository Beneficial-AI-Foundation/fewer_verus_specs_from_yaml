/*
-----Description-----
This task requires writing a Verus method that removes an element from a given vector of integers at a specified index. The resulting vector should contain all the original elements except for the one at the given index. Elements before the removed element remain unchanged, and elements after it are shifted one position to the left.

-----Input-----
The input consists of:
• s: A vector of integers.
• k: A natural number representing the index of the element to remove (0-indexed).

-----Output-----
The output is a vector of integers that:
• Has a length one less than the input vector.
• Contains the same elements as the input vector, except that the element at index k is omitted.
• Preserves the original order with elements after the removed element shifted left by one position.

-----Note-----
It is assumed that k is a valid index (0 ≤ k < the length of the vector).
*/
use vstd::prelude::*;

verus! {

spec fn removeElement_precond(s: Vec<i32>, k: usize) -> bool {
    k < s.len()
}
// <vc-helpers>
// </vc-helpers>
spec fn removeElement(s: Vec<i32>, k: usize) -> Vec<i32>
    recommends removeElement_precond(s, k)
// <vc-implementation>
{
    arbitrary() // TODO: Remove this line and implement the function body
}
// </vc-implementation>
spec fn removeElement_postcond(s: Vec<i32>, k: usize, result: Vec<i32>) -> bool {
    &&& result.len() == s.len() - 1
    &&& (forall|i: int| 0 <= i < k as int ==> result@.index(i) == s@.index(i))
    &&& (forall|i: int| k as int <= i < result.len() as int ==> result@.index(i) == s@.index(i + 1))
}

proof fn removeElement_spec_satisfied(s: Vec<i32>, k: usize)
    requires removeElement_precond(s, k)
    ensures removeElement_postcond(s, k, removeElement(s, k))
// <vc-proof>
{
    assume(false); // TODO: Remove this line and implement the proof
}
// </vc-proof>
/*
// Invalid Inputs
[
    {
        "input": {
            "s": "vec![1]",
            "k": 2
        }
    }
]
// Tests
[
    {
        "input": {
            "s": "vec![1, 2, 3, 4, 5]",
            "k": 2
        },
        "expected": "vec![1, 2, 4, 5]",
        "unexpected": [
            "vec![1, 2, 3, 5]",
            "vec![1, 3, 4, 5]",
            "vec![2, 3, 4, 5]"
        ]
    },
    {
        "input": {
            "s": "vec![10, 20, 30, 40]",
            "k": 0
        },
        "expected": "vec![20, 30, 40]",
        "unexpected": [
            "vec![10, 30, 40]",
            "vec![10, 20, 30, 40]",
            "vec![20, 30, 40, 0]"
        ]
    },
    {
        "input": {
            "s": "vec![10, 20, 30, 40]",
            "k": 3
        },
        "expected": "vec![10, 20, 30]",
        "unexpected": [
            "vec![20, 30, 40]",
            "vec![10, 20, 40]",
            "vec![10, 30, 40]"
        ]
    },
    {
        "input": {
            "s": "vec![7]",
            "k": 0
        },
        "expected": "vec![]",
        "unexpected": [
            "vec![7]",
            "vec![0]"
        ]
    }
]
*/

fn main() { }

}