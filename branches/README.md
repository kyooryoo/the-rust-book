# Control Flow
Branches are created per conditions of true or false.

Only boolean type value can be used for conditions.
Other type values are not converted implicitly.
This is different from Ruby or JavaScript, for example:
```
let num = 3;
if num {
    println!("is not equal to 0!", {});
}
```
The code above causes panic since *num* is not a boolean.

## Returned Value
Conditional expression return unit value of *()* by default.
The possiblely returned values MUST in the same data type.
*if* can work with *let* for assigning values conditionally.