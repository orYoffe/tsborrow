type Owned<T> = T;

async function readLater() {
  const file: Owned<FileHandle> = openFile("later.txt");
  const reader = borrow(file);
  await schedule();
  reader.read();
}
