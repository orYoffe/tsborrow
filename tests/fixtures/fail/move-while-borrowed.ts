type Owned<T> = T;

const session: Owned<Session> = createSession();
const view = borrow(session);
const transferred = move(session);
view.inspect();
transferred.close();
