# Data Types
Rust is a statically typed language.
It must know the types of all variables at compile time.
When many types are possible, use parse with type annotation:
`let guess: u32 = "42".parse().expect("Not a number!");`

## Scalar Types
A scalar type represents a single value. 
Rust has four primary scalar types: 
* integers
* floating-point numbers
* Booleans
* characters.

### Integers
Could be signed *i* or unsigned *u*.
* means could be negative value or not
Could be 8/16/32/64/128/arch length.
* the default type is i32
* arch means depends on CPU type
* arch uses *isize* and *usize*
Integer type is denotated by suffix
* such as *57u8* means unsigned 8-bit

### Integer Literals
It means supporting the following numbers:
* Decimal:  98_222
* Hex:      0xff
* Octal:    0o77
* Binary:   0b1111_0000
* Byte:     b'A'
The visual seperator *_* facilitate human reading.
It will be ignored by complier as if does not exist.

### Integer Overflow
Variable value should not cross its data type range.
In debug mode, compiler panic when overflow happens.
In release mode, overflow is ignored by wrapping.
Overflow on one end is wrapped to the other end. 

Handle the possible overflow with the methods:
* Request wrapping explicitly: wrapping_*
* Return *None* if overflow: checked_*
* Return with a boolean indicator: overflowing_*
* Return the min or max value: saturating_*

## Floating Point
The default type is *f64*, *f32* is also supported.

## Characters
The type *char* support Unicode characters.
Use single quotation mark for marking *char* value.

## Tuple
Tuple can have mix type of value but with fix length.
Element in a tuple is immutable in value and type.
Destructure or using index to access tuple elements.
An empty tuple *()* is in *unit type* with *unit value*.
Expression returns *unit value* if no value to return.

## Array
Array also has fixed length but only one type for all.
Vector is mutable array and will be introduced later.