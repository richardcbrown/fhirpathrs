# fhirpathrs

A [FHIRPath](http://hl7.org/fhirpath/) implementation in Rust.

## Implementation Status

The current intention is to support the [N1 normative release of FHIRPath](https://hl7.org/fhirpath/N1/) and the [R5 specification](https://hl7.org/fhir/R5/fhirpath.html),
however due to the asynchronous nature of the R5 specification this will likely require a large refactor.


| Icon          | Status                           |
|---------------|----------------------------------|
| :x: | Feature has not been implemented |
| :exclamation: | Feature is partially implemented |
|:white_check_mark:| Feature has been implemented     |

### N1 normative Implementation Status

- 5. Functions
  - 5.1. Existence
    - 5.1.1. empty() : Boolean - :white_check_mark:
    - 5.1.2. exists([criteria : expression]) : Boolean - :white_check_mark:
    - 5.1.3. all(criteria : expression) : Boolean - :exclamation:
    - 5.1.4. allTrue() : Boolean - :white_check_mark:
    - 5.1.5. anyTrue() : Boolean - :white_check_mark:
    - 5.1.6. allFalse() : Boolean - :white_check_mark:
    - 5.1.7. anyFalse() : Boolean - :white_check_mark:
    - 5.1.8. subsetOf(other : collection) : Boolean - :white_check_mark:
    - 5.1.9. supersetOf(other : collection) : Boolean - :white_check_mark:
    - 5.1.10. count() : Integer - :white_check_mark:
    - 5.1.11. distinct() : collection - :white_check_mark:
    - 5.1.12. isDistinct() : Boolean - :white_check_mark:
  - 5.2. Filtering and projection
    - 5.2.1. where(criteria : expression) : collection - :white_check_mark:
    - 5.2.2. select(projection: expression) : collection - :white_check_mark:
    - 5.2.3. repeat(projection: expression) : collection - :white_check_mark:
    - 5.2.4. ofType(type : type specifier) : collection - :exclamation:
  - 5.3. Subsetting
    - 5.3.1. [ index : Integer ] : collection - :white_check_mark:
    - 5.3.2. single() : collection - :white_check_mark:
    - 5.3.3. first() : collection - :white_check_mark:
    - 5.3.4. last() : collection - :white_check_mark:
    - 5.3.5. tail() : collection - :white_check_mark:
    - 5.3.6. skip(num : Integer) : collection - :white_check_mark:
    - 5.3.7. take(num : Integer) : collection - :white_check_mark:
    - 5.3.8. intersect(other: collection) : collection - :white_check_mark:
    - 5.3.9. exclude(other: collection) : collection - :white_check_mark:
  - 5.4. Combining
    - 5.4.1. union(other : collection) - :white_check_mark:
    - 5.4.2. combine(other : collection) : collection - :white_check_mark:
  - 5.5. Conversion
    - 5.5.1. iif(criterion: expression, true-result: collection [, otherwise-result: collection]) : collection - :white_check_mark:
    - 5.5.2. Boolean Conversion Functions
      - toBoolean() : Boolean - :white_check_mark:
      - convertsToBoolean() : Boolean - :white_check_mark:
    - 5.5.3. Integer Conversion Functions
      - toInteger() : Integer - :white_check_mark:
      - convertsToInteger() : Boolean - :white_check_mark:
    - 5.5.4. Date Conversion Functions
      - toDate() : Date - :white_check_mark:
      - convertsToDate() : Boolean - :white_check_mark:
    - 5.5.5. DateTime Conversion Functions
      - toDateTime() : DateTime - :white_check_mark:
      - convertsToDateTime() : Boolean - :white_check_mark:
    - 5.5.6. Decimal Conversion Functions
      - toDecimal() : Decimal - :white_check_mark:
      - convertsToDecimal() : Boolean - :white_check_mark:
    - 5.5.7. Quantity Conversion Functions
      - toQuantity([unit : String]) : Quantity - :x:
      - convertsToQuantity([unit : String]) : Boolean - :x:
    - 5.5.8. String Conversion Functions
      - toString() : String - :white_check_mark:
      - convertsToString() : String - :white_check_mark:
    - 5.5.9. Time Conversion Functions
      - toTime() : Time - :white_check_mark:
      - convertsToTime() : Boolean - :white_check_mark:
  - 5.6. String Manipulation
    - 5.6.1. indexOf(substring : String) : Integer - :white_check_mark:
    - 5.6.2. substring(start : Integer [, length : Integer]) : String - :white_check_mark:
    - 5.6.3. startsWith(prefix : String) : Boolean - :white_check_mark:
    - 5.6.4. endsWith(suffix : String) : Boolean - :white_check_mark:
    - 5.6.5. contains(substring : String) : Boolean - :white_check_mark:
    - 5.6.6. upper() : String - :white_check_mark:
    - 5.6.7. lower() : String - :white_check_mark:
    - 5.6.8. replace(pattern : String, substitution : String) : String - :white_check_mark:
    - 5.6.9. matches(regex : String) : Boolean - :white_check_mark:
    - 5.6.10. replaceMatches(regex : String, substitution: String) : String - :white_check_mark:
    - 5.6.11. length() : Integer - :white_check_mark:
    - 5.6.12. toChars() : collection - :white_check_mark:
  - 5.7. Math
    - 5.7.1. abs() : Integer | Decimal | Quantity - :white_check_mark:
    - 5.7.2. ceiling() : Integer - :white_check_mark:
    - 5.7.3. exp() : Decimal - :white_check_mark:
    - 5.7.4. floor() : Integer - :white_check_mark:
    - 5.7.5. ln() : Decimal - :white_check_mark:
    - 5.7.6. log(base : Decimal) : Decimal - :white_check_mark:
    - 5.7.7. power(exponent : Integer | Decimal) : Integer | Decimal - :exclamation: - complex numbers need checking correctly
    - 5.7.8. round([precision : Integer]) : Decimal - :white_check_mark:
    - 5.7.9. sqrt() : Decimal - :white_check_mark:
    - 5.7.10. truncate() : Integer - :white_check_mark:
  - 5.8. Tree navigation
    - 5.8.1. children() : collection - :exclamation:
    - 5.8.2. descendants() : collection - :exclamation:
  - 5.9. Utility functions
    - 5.9.1. trace(name : String [, projection: Expression]) : collection
    - 5.9.2. Current date and time functions
      - now() : DateTime
      - timeOfDay() : Time
      - today() : Date
- 6. Operations
  - 6.1. Equality
    - 6.1.1. = (Equals)
    - 6.1.2. ~ (Equivalent)
    - 6.1.3. != (Not Equals)
    - 6.1.4. !~ (Not Equivalent)
  - 6.2. Comparison
    - 6.2.1. > (Greater Than)
    - 6.2.2. < (Less Than)
    - 6.2.3. <= (Less or Equal)
    - 6.2.4. >= (Greater or Equal)
  - 6.3. Types
    - 6.3.1. is type specifier
    - 6.3.2. is(type : type specifier)
    - 6.3.3. as type specifier
    - 6.3.4. as(type : type specifier)
  - 6.4. Collections
    - 6.4.1. | (union collections)
    - 6.4.2. in (membership)
    - 6.4.3. contains (containership)
  - 6.5. Boolean logic
    - 6.5.1. and
    - 6.5.2. or
    - 6.5.3. not() : Boolean
    - 6.5.4. xor
    - 6.5.5. implies
  - 6.6. Math
    - 6.6.1. * (multiplication)
    - 6.6.2. / (division)
    - 6.6.3. + (addition)
    - 6.6.4. - (subtraction)
    - 6.6.5. div
    - 6.6.6. mod
    - 6.6.7. & (String concatenation)
  - 6.7. Date/Time Arithmetic
    - 6.7.1. + (addition)
    - 6.7.2. - (subtraction)
- 7. Aggregates
  - 7.1. aggregate(aggregator : expression [, init : value]) : value
- 8. Lexical Elements
  - 8.1. Whitespace
  - 8.2. Comments
- 9. Environment variables
- 10. Types and Reflection
  - 10.2. Reflection
    - 10.2.1. Primitive Types
    - 10.2.2. Class Types
    - 10.2.3. Collection Types
    - 10.2.4. Anonymous Types
