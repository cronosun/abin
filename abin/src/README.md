# Code organization

 * `binary`: The interface for the binary type.
 * `string`: The interface for the string type.
 * `implementation`: Binary and string implementation (there's just one module for both implementations, since the string type more or less just wraps the binary type).
 * `common`: Thinsgs used by the binary- and the string-type.
 * `serde_support`: Serde-support, obviously.
 * `boo`: Borrowed-or-owned.
 * `spi`: Service provider interface: This is only required if you want to implement your own binary.