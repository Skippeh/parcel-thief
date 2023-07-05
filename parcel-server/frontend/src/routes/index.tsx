import * as React from "react";
import { Outlet, Route, createBrowserRouter, redirect } from "react-router-dom";
import Login from "./pages/login";
import PageError from "./page_error";
import Layout from "./layout";
import Home from "./pages/home";

export default createBrowserRouter(
  [
    {
      path: "/",
      element: <Layout />,
      errorElement: <PageError />,
      children: [
        {
          path: "",
          element: <Home />,
        },
      ],
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
