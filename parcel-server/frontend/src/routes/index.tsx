import * as React from "react";
import { createBrowserRouter } from "react-router-dom";
import Login from "./pages/login";
import PageError from "./page_error";
import Layout from "./layout";
import Home from "./pages/home";
import ProtectedContent from "./protected_content";
import Items from "./pages/items";
import Accounts from "./pages/accounts";

export default createBrowserRouter(
  [
    {
      path: "/",
      element: <Layout />,
      errorElement: <PageError />,
      children: [
        {
          path: "",
          element: <ProtectedContent />,
          children: [
            // Protected routes that don't require any special permissions go here
            {
              path: "",
              element: <Home />,
            },
            {
              path: "/items",
              element: <Items />,
            },
          ],
        },
        {
          path: "/accounts",
          element: <ProtectedContent permissions={["manageAccounts"]} />,
          children: [
            {
              path: "",
              element: <Accounts />,
            },
          ],
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
