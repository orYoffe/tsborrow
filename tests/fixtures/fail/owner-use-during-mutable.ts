type Owned<T> = T;

const buffer: Owned<Buffer> = allocate();
const writer = borrowMut(buffer);
buffer.clear();
writer.write("data");
