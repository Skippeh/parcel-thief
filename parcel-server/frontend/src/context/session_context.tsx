import * as React from "react";
import { createContext, useState } from "react";
import useStorage from "../hooks/use_storage";
import { JwtPayload } from "../api_types";

export enum UserPermissions {
  None = 0,
}

export interface User {
  name: string;
  avatarUrl: string;
  permissions: UserPermissions;
}

interface Session {
  user: User | null;
  authToken: string | null;

  setUserAndToken: (
    userAndToken: { user: User; authToken: string } | null
  ) => void;
}

interface SavedSession {
  user: User;
  authToken: string;
}

export const SessionContext = createContext<Session | null>(null);

export const SessionContextProvider: React.FC<React.PropsWithChildren> = ({
  children,
}) => {
  const storage = useStorage<SavedSession>("session", "session");
  let savedSession = storage.get();

  if (savedSession != null) {
    // check if session is expired
    const jwtToken = decodeJwtPayload(savedSession?.authToken);
    const expireDate = new Date(jwtToken.expiresAt * 1000);

    if (new Date() >= expireDate) {
      savedSession = null;
      storage.remove();
    }
  }

  let [user, setUser] = useState<User | null>(savedSession?.user || null);
  let [authToken, setAuthToken] = useState<string | null>(
    savedSession?.authToken || null
  );

  let session: Session = {
    user,
    authToken,

    setUserAndToken: (
      userAndToken: { user: User; authToken: string } | null
    ) => {
      setUser(userAndToken?.user || null);
      setAuthToken(userAndToken?.authToken || null);

      if (userAndToken != null) {
        storage.set({
          user: userAndToken.user,
          authToken: userAndToken.authToken,
        });
      } else {
        storage.remove();
      }
    },
  };

  return (
    <SessionContext.Provider value={session}>
      {children}
    </SessionContext.Provider>
  );
};

function decodeJwtPayload(token: string): JwtPayload {
  const b64 = token.split(".")[1];
  const json = atob(b64);

  return JSON.parse(json);
}
