import * as React from "react";
import styled from "styled-components";
import { RouterProvider } from "react-router-dom";
import router from "./routes";

const App = () => {
  return (
    <div>
      <RouterProvider router={router} />
    </div>
  );
};

export default App;
