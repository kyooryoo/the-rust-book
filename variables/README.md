# Variables
Variables are immutable by default.
Use *mut* keyword for mutable variables.

Variables could be shadowed with same name.
So, the following code is correct:
```
let spaces = "   ";
let spaces = spaces.len();
```
The first variable has string data type.
The second one has number data type.
* use *let* keyword for creating a shadow
* as new variable, *mut* is not necessary

## Constants
Different from variables, constants are:
* always immutable and do not accept *mut*
* delcared with the keyword *const*
* use upcase in name and *_* between words
* data type MUST be annotated
Constants are used for:
* values used for many parts of the program
* update in one place applies to all places