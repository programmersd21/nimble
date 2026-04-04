# list Module

List helpers.

## Functions
- `is_empty(lst [T]) -> bool`: Returns true when list has no elements.
- `contains(lst [T], item T) -> bool`: Returns true if `item` exists in list.
- `index_of(lst [T], item T) -> int`: Returns index of `item` or `-1`.
- `push(lst [T], item T) -> null`: Appends `item` to the list.
- `pop(lst [T]) -> T | error`: Removes and returns the last element.
- `first(lst [T]) -> T | error`: Returns first element.
- `last(lst [T]) -> T | error`: Returns last element.
- `insert(lst [T], idx int, item T) -> null`: Inserts at index.
- `remove(lst [T], idx int) -> T | error`: Removes at index.
- `slice(lst [T], start int, end int) -> [T]`: Returns a slice.
- `reverse(lst [T]) -> [T]`: Returns a reversed copy.
- `sort(lst [T]) -> [T] | error`: Sorts `int`, `float`, or `str` lists.

## Example
```nimble
load list
nums = [3, 1, 2]
list.sort(nums)?
out(nums)
```
