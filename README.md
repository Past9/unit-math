# unit-math

Simple macro library for keeping track of units when using physical quantities. Simply define the units, then the relationships between them.

## Physical Formulas

For example, Ohm's Law states:

```
Voltage = Current * Resistance

or

V = IR

or

Volts = Amps * Ohms
```

This gives us three relationships:

```
Volts = Amps * Ohms
Amps = Volts / Ohms
Ohms = Volts / Amps
```

To express this using this library, first define the units and their symbols (for display purposes):
```rust
unit!(Volt, "V");
unit!(Amp, "A");
unit!(Ohm, "Ω");
```

Then define any one of the equations above:
```rust
multiply_units!(Amp, Ohm, Volt); // Amps * Ohms = Volts
// OR
divide_units!(Volt, Amp, Ohm);   // Volts / Amps = Ohms
// OR
divide_units!(Volt, Ohm, Amp);   // Volts / Ohms = Amps
```

You must choose only one equation to describe the relationship between the units. Macros will generate all the others. If you define more than one, you will get compiler errors due to conflicting implementations.

The compiler will now allow arithmetic on these quantities while keeping track of the units for you:
```rust
let volts = Volt(10.0);
let amps = Volt(2.0);

let ohms: Ohms = volts / amps;

println!("{}", ohms); // prints "5Ω"
```

## Unit Conversions

You can also do unit conversions by defining the units and their conversion constants. For example, to convert between degrees and radians, simply define the two units and the conversion factor to go from one to the other. This must be defined in both directions:

```rust
unit!(Meter, "m");
unit!(Millimeter, "mm");

convert_units!(Meter, Millimeter, 1000.0);
convert_units!(Millimeter, Meter, 0.001);

let meters = Meter(2.0);
let millimeters: Millimeter = meters.into();

assert_eq!(Millimeter(2000.0), millimeters); 
assert_eq!(Meter(2.0), millimeters.into());
```

## Limitations

This library is mostly intended to be used in `#[no_std]` embedded contexts, so all units are internally represented by `f32` for efficiency.  
