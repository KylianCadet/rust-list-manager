# Rust list manager

List manager is a program that performs a variety of tasks on a given list through a command line interface.

## List of available commands

### Help

Display list of available commands.

### Define

Sets a list to be used by following tasks.

```
> define [9, 8, 7, 6, 5, 4, 3, 2, 1]
```

### Cut

Takes an index as parameter and splits the previous list(s) at the given index.

```
> define [9, 8, 7, 6, 5, 4, 3, 2, 1]
> cut 3
[9, 8, 7, 6]
[5, 4, 3, 2, 1]
```

### Swap

Takes an index as parameter and swaps the elements of the previous list(s) using the index as a swapping point.

```
> cut 3
[9, 8, 7, 6]
[5, 4, 3, 2, 1]
> swap 1
[7, 6, 8, 9]
[3, 2, 1, 4, 5]
```

### Add

Increases every element of the previous list(s) by the given amount.

```
> mirror_swap 2
[9, 8, 6, 7]
[5, 4, 1, 2, 3]
> add 2
[11, 10, 8, 9]
[7, 6, 3, 4, 5]
```

### Sort

Sorts the previous list(s) in increasing order.

```
> add 2
[11, 10, 8, 9]
[7, 6, 3, 4, 5]
> sort
[8, 9, 10, 11]
[3, 4, 5, 6, 7]
```

### Chunks

Separates the previous list(s) in chunks of the given amount.

```
> sort
[8, 9, 10, 11]
[3, 4, 5, 6, 7]
> chunks 2
[8, 9]
[10, 11]
[3, 4]
[5, 6]
[7]
```

### Flatten

Unifies the previous lists as one.

```
> chunks 2
[8, 9]
[10, 11]
[3, 4]
[5, 6]
[7]
> flatten
[8, 9, 10, 11, 3, 4, 5, 6, 7]
```

### Sum

Performs the cumulated sum of the previous list if there is only one list, and consumes it.

```
> flatten
[8, 9, 10, 11, 3, 4, 5, 6, 7]
> sum
63
```
