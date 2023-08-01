import * as React from "react";
import { RouterProvider } from "react-router-dom";
import router from "./routes";
import { SessionContextProvider } from "./context/session_context";

const App = () => {
  return (
    <SessionContextProvider>
      <RouterProvider router={router} />
    </SessionContextProvider>
  );
};

export default App;
