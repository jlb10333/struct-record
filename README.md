# struct-record

This is a macro utility to generate struct definitions from enum definitions, in a similar way to Typescript's `Record<K, V>` type when used with a string union type as `K`.

## Usage

The following enum definition with the macro is more or less analogous to `type ExampleEnumToF32 = Record<ExampleEnum, f32>`:

```rs
#[record(f32, ExampleEnumToF32)]
enum ExampleEnum {
  FooExample,
  BarExample,
  BazExample,
}

const ex: ExampleEnumToF32 = ExampleEnumToF32 {
  foo_example: 1.0,
  bar_example: 2.0,
  baz_example: 3.0,
}
```

## Installation

`cargo install struct-record`
