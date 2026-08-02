type Owned<T> = T;

const cache: Owned<Cache> = createCache();
const reader = borrow(cache);
const writer = borrowMut(cache);
reader.read();
writer.write();
