/** @owned */
const store = createStore();
{
  const writer = borrowMut(store);
  writer.set("key", "value");
}
const reader = borrow(store);
reader.get("key");
