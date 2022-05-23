A simple rust implementation of the Quake III reverse square root alghorithm.

## Safety

The main trait implementations are not marked as unsafe. However, given that
`std::mem::transmute` is used under the hood undefined behaviour is always a possibility.

The library guarantees, however, that needed conversions are done between equally sized types.
