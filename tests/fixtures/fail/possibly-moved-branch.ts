type Owned<T> = T;

const session: Owned<Session> = createSession();
if (shouldTransfer()) {
  move(session);
}
session.send("hello");
