import { useContext } from "react";
import { SessionContext, User } from "../context/session_context";

const useSession = () => {
  const session = useContext(SessionContext);

  if (session == null) {
    throw new Error("No session context found");
  }

  const getUser = () => session.user;
  const getAuthToken = () => session.authToken;
  const isLoggedIn = () => session.user != null && session.authToken != null;
  const logout = () => session.setUserAndToken(null);
  const setSession = (user: User, authToken: string) =>
    session.setUserAndToken({
      user: {
        name: user.name,
        avatarUrl: user.avatarUrl,
        permissions: user.permissions,
        gameId: user.gameId,
      },
      authToken,
    });

  return { getUser, getAuthToken, isLoggedIn, logout, setSession };
};

export default useSession;
