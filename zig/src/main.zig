const std = @import("std");
const print = std.debug.print;
const assert = std.debug.assert;
const expect = std.testing.expect;

// Top level declaration, order-independent
var MAYBE_BOOL: bool = false;

fn max(comptime T: type, a: T, b: T) T
{
    if (T == bool) return a or b;
    return if (a > b) a else b;
}

fn biggestFloat(a: f32, b: f32) f32
{
    return max(f32, a, b);
}

fn biggestInt(a: i32, b: i32) i32
{
    return max(i32, a, b);
}

fn getValue(maybe: bool) u32
{
    return if (maybe) 0 else 1;
}

// Generic types
fn List(comptime T: type) type
{
    return struct
    {
        items: []T,
        len: usize,
    };
}

fn printString(string: [*:0]const u8) void
{
    print("str: '{s}'\n", .{ string });
}

fn multiply(value: *u32) void
{
    value.* *= 2;
}

test "c-like pointer"
{
    var x: u32 = 128;
    var y: *u32 = &x;

    // Dereference and add to x through y
    y.* += 32;
    multiply(&x);

    assert(x == 320);
}

test "slicing"
{
    const array = [_]u8 { 1, 2, 3, 4, 5, 6, 7, 8 };
    const slice = array[0..3];

    assert(slice[2] == 3);
}

pub fn main() void {
    const one_bit: u1 = 1;
    const two_bit: i22 = -2;

    const maybe_u32 = getValue(MAYBE_BOOL);
    MAYBE_BOOL = true;
    const maybe_u32_true = getValue(MAYBE_BOOL);

    print("Maybe: {} and {}\n", .{ maybe_u32, maybe_u32_true });

    var hello: *const [5:0]u8 = undefined;
    hello = "Hello";
    printString(hello);

    const hello_world = "Hello, World!";
    printString(hello_world);

    print("Hello, World!\n", .{});
    print("Bit - 1: '{}', 22: '{}'\n", .{ one_bit, two_bit });

    // Create a list
    const list_i32 = List(i32);
}

// Test cases
test "Try to compare bools"
{
    try expect(max(bool, false, true) == true);
}