# Arrays and Strings

## Exercises

[Contains Duplicates](src/remove_duplicate.rs): Checking if an array has duplicate values.

```bash
cargo run --bin contains_duplicate
```

[Remove Duplicates](src/remove_duplicate.rs): Removing duplicates from an array.

```bash
cargo run --bin remove_duplicate
```

## Techniques

### Two Pointers

```lua
function fn(arr)
  left = 0;
  right = arr.len() -1 

  while (left < right) do
    // Some logic
    // Then do one of the following:
    //   1. left++
    //   2. right--
    //   3. both left++ and right--
  end
end
```
