type Owned<T> = T;

const record: Owned<DataRecord> = loadRecord();
const reader = borrow(record);
reader.read();
const writer = borrowMut(record);
writer.update();
