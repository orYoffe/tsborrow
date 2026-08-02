type Owned<T> = T;

function exposeSecret() {
  const secret: Owned<Secret> = loadSecret();
  const view = borrow(secret);
  return view;
}
