type Owned<T> = T;

const user: Owned<User> = loadUser();
const avatar = move(user.profile.avatar);
render(user.profile.avatar);
upload(avatar);
