import * as React from "react";
import { createContext, useState } from "react";

enum UserPermissions {
  None = 0,
}

interface User {
  name: string;
  avatarUrl: string;
  permissions: UserPermissions;
}

interface Session {
  user: User | null;
  authToken: string | null;

  setUserAndToken: (user: User | null, authToken: string | null) => void;
}

export const SessionContext = createContext<Session | null>(null);

export const SessionContextProvider: React.FC<React.PropsWithChildren> = ({
  children,
}) => {
  let [user, setUser] = useState<User | null>(null);
  let [authToken, setAuthToken] = useState<string | null>(null);

  let session: Session = {
    user,
    authToken,

    setUserAndToken: (user, authToken) => {
      setUser(user);
      setAuthToken(authToken);
    },
  };

  return (
    <SessionContext.Provider value={session}>
      {children}
    </SessionContext.Provider>
  );
};
