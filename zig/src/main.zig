const std = @import("std");
const print = std.debug.print;
const assert = std.debug.assert;
const expect = std.testing.expect;

const queue = @import("queue.zig");

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

// Pass value as immutable
fn printU32(value: *const u32) void
{
    print("value: {}\n", .{ value.* });
}

// Pass value as mutable
fn printU32Mut(value: *u32) void
{
    value.* *= 2;
    print("value: {}\n", .{ value.* });
}

// Pass slice as immutable
fn printSlice(slice: []const u8) void
{
    print("slice: {any}\n", .{ slice });
}

// Pass slice as mutable
fn printSliceMut(slice: []u8) void
{
    // Modify by reference
    for(slice) |*val| { val.* *= 2; }
    print("slice: {any}\n", .{ slice });
}

test "queue"
{
    const alloc = std.testing.allocator;

    var int_queue = queue.Queue(i32).init(alloc);
    
    try int_queue.enqueue(25);
    try int_queue.enqueue(50);
    try int_queue.enqueue(75);
    try int_queue.enqueue(100);

    try std.testing.expectEqual(int_queue.dequeue(), 25);
    try std.testing.expectEqual(int_queue.dequeue(), 50);
    try std.testing.expectEqual(int_queue.dequeue(), 75);
    try std.testing.expectEqual(int_queue.dequeue(), 100);
    try std.testing.expectEqual(int_queue.dequeue(), null);
}

test "slicing"
{
    const array = [_]u8 { 1, 2, 3, 4, 5, 6, 7, 8 };
    const slice = array[0..3];

    assert(slice[2] == 3);
}

test "pointer"
{
    var array = [_]u8 { 1, 2, 3, 4, 5, 6, 7, 8 };
    const ptr = &array;

    // Increment value in array
    ptr[3] += 2;

    assert(ptr[3] == 6);
}

test "More pointers"
{
    const locked: u8 = 5;
    var unlocked: u8 = 10;

    // Both point to a constant value that cannot change
    // P2 has the ability to change address it points to
    // P1 is locked into the address
    const   p1: *const u8 = &locked;
    var     p2: *const u8 = &locked;

    // Both are able to modify the value they point to
    // P3 is locked to the address
    const   p3: *u8 = &unlocked;
    var     p4: *u8 = &unlocked;

    // Both act exactly the same as P1 and P2
    const   p5: *const u8 = &unlocked;
    var     p6: *const u8 = &unlocked;
}

pub fn main() void {
    const one_bit: u1 = 1;
    const two_bit: i22 = -2;

    var hella: ?u8 = null;
    print("Value of hella: {}\n", .{ hella });

    const maybe_u32 = getValue(MAYBE_BOOL);
    MAYBE_BOOL = true;
    const maybe_u32_true = getValue(MAYBE_BOOL);

    print("Maybe: {} and {}\n", .{ maybe_u32, maybe_u32_true });

    var hello: *const [5:0]u8 = undefined;
    hello = "Hello";
    printString(hello);

    // Optionals
    var yes: ?u8 = null orelse 10;
    print("Value of yes: {}\n", .{ yes });

    const hello_world = "Hello, World!";
    printString(hello_world);

    print("Hello, World!\n", .{});
    print("Bit - 1: '{}', 22: '{}'\n", .{ one_bit, two_bit });

    // Create a list
    const list_i32 = List(i32);

    var array = [_]u8 { 1, 2, 3, 4, 5, 6, 7, 8 };
    printSlice(array[2..5]);
    printSliceMut(array[2..5]);

    // Can print string as slice
    printSlice(hello);

    var array_u32 = [_]u32 { 1, 2, 3, 4, 5, 6 };

    printU32(&array_u32[2]);
    printU32Mut(&array_u32[2]);
}

// Test cases
test "Try to compare bools"
{
    try expect(max(bool, false, true) == true);
}