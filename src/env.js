function $Array$constructor() {
    return HEAP.add([]);
}

function $Array$push(array, value) {
    HEAP.get(array).push(HEAP.get(value));
}

function $Function$from(func) {
    return HEAP.add(function (...args) {
        WASM.function(func)(...args.map((arg) => HEAP.add(arg)));
    });
}

function $Integer$from_u32(integer) {
    return HEAP.add(integer);
}

function $Integer$from_i32(integer) {
    return HEAP.add(integer);
}

function $Float$from_f64(float) {
    return HEAP.add(float);
}

function $Float$to_f64(index) {
    return HEAP.get(index);
}

function $Object$call(object, property, args) {
    let retval = HEAP.get(object)[HEAP.get(property)](...HEAP.get(args));
    if (typeof retval == "undefined") {
        return 0;
    } else {
        return HEAP.add(retval);
    }
}

function $Object$get(object, property, value) {
    let retval = HEAP.get(object)[HEAP.get(property)];
    if (typeof retval == "undefined") {
        return 0;
    } else {
        return HEAP.add(retval);
    }
}

function $Object$set(object, property, value) {
    HEAP.get(object)[HEAP.get(property)] = HEAP.get(value);
}

function $Performance$now() {
    return performance.now();
}

function $String$from_str(ptr, len) {
    return HEAP.add(WASM.text(ptr, len));
}

function $Uint8Array$copy_to_slice(array, ptr, len) {
    WASM.memory(ptr, len).set(HEAP.get(array));
}

function $Value$clone(value) {
    return HEAP.add(HEAP.get(value));
}

function $Value$drop(value) {
    HEAP.rm(value);
}

function $Value$to_u32(value) {
    return HEAP.get(value);
}

function $panic() {
    throw new Error("panicked");
}

function $queueMicrotask(func) {
    queueMicrotask(WASM.function(func));
}
