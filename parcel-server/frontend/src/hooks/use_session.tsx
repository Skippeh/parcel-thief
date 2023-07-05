import { useContext } from "react";
import { SessionContext } from "../context/session_context";

const useSession = () => {
  const session = useContext(SessionContext);

  if (session == null) {
    throw new Error("No session context found");
  }

  const getUser = () => session.user;
  const getAuthToken = () => session.authToken;
  const isLoggedIn = () => session.user != null && session.authToken != null;
  const logout = () => session.setUserAndToken(null, null);

  return { getUser, getAuthToken, isLoggedIn, logout };
};

export default useSession;
