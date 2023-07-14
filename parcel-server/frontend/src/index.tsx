import * as React from "react";
import { createRoot } from "react-dom/client";
import App from "./app";
import styled from "styled-components";
import "normalize.css";
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-alpine.css";

const rootElement = document.getElementById("root");

if (rootElement == null) {
  throw new Error("Failed to find root element");
}

const root = createRoot(rootElement);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
