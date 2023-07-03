import * as React from "react";
import { createBrowserRouter } from "react-router-dom";
import Login from "./login";
import PageError from "./page_error";

const Index = () => {
  return <div>index</div>;
};

export default createBrowserRouter(
  [
    {
      path: "/",
      element: <Index />,
      errorElement: <PageError />,
    },
    {
      path: "/login",
      element: <Login />,
    },
  ],
  {
    basename: "/frontend",
  }
);
