import { useContext } from "react";
import { SessionContext, User } from "../context/session_context";
import { JwtPayload } from "../api_types";
import { logout as authServiceLogout } from "../services/auth_service";

function decodeJwtPayload(authToken: string): JwtPayload {
  let b64Payload = authToken.split(".")[1];
  return JSON.parse(atob(b64Payload));
}

const useSession = () => {
  const session = useContext(SessionContext);

  if (session == null) {
    throw new Error("No session context found");
  }

  const getUser = () => session.user;
  const getAuthToken = () => session.authToken;
  const isLoggedIn = () => session.user != null && session.authToken != null;
  const logout = async (): Promise<boolean> => {
    const logoutResponse = await authServiceLogout();

    // 401 means token is expired or otherwise invalid
    if (logoutResponse.statusCode == 200 || logoutResponse.statusCode == 401) {
      session.setUserAndToken(null);
      return true;
    }

    return false;
  };
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
  const getJwtPayload = (): JwtPayload | null => {
    if (session.authToken == null) {
      return null;
    }

    const jwtPayload = decodeJwtPayload(session.authToken);
    return jwtPayload;
  };

  return {
    getUser,
    getAuthToken,
    isLoggedIn,
    logout,
    setSession,
    getJwtPayload,
  };
};

export default useSession;
